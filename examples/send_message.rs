use futures_util::StreamExt;
use iggy::prelude::*;
use std::str::FromStr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ===== 第一步：建立连接 =====
    // 使用默认账号 iggy:iggy 连接本机 8090 端口
    let client = IggyClient::from_connection_string("iggy://iggy:iggy@127.0.0.1:8090")?;
    client.connect().await?;
    println!("Connect successfully...");

    let stream_name = "message-platform";
    let topic_name = "orders";

    // ===== 第二步：创建 Stream（流） =====
    // Stream 是最顶层的数据容器，类似于一个"仓库"
    // 如果 Stream 已存在，忽略 StreamNameAlreadyExists 错误
    match client.create_stream(stream_name).await {
        Ok(_) => println!("Stream 'message-platform' created successfully"),
        Err(IggyError::StreamNameAlreadyExists(_)) => println!("Stream already exists, keep going"),
        Err(e) => return Err(e.into()),
    }

    // ===== 第三步：创建 Topic（主题） =====
    // Topic 是 Stream 下的消息分类，类似于"仓库中的货架"
    // 这里创建一个名为 "orders" 的主题，设置 1 个分区，消息永不过期
    match client
        .create_topic(
            &stream_name.try_into()?,
            topic_name,
            &TopicCreateOptions {
                partitions_count: Some(1),
                message_expiry: Some(IggyExpiry::NeverExpire),
                ..TopicCreateOptions::default()
            },
        )
        .await
    {
        Ok(_) => println!("Topic '{}' created successfully", topic_name),
        Err(IggyError::TopicNameAlreadyExists(_, _)) => println!("Topic 已存在，继续使用"),
        Err(e) => return Err(e.into()),
    }

    // ===== 第四步：创建生产者并发送消息 =====
    // producer() 方法返回 IggyProducerBuilder，通过链式调用配置后 .build() 创建生产者
    let producer = client.producer(stream_name, topic_name)?.build();
    producer.init().await?;

    let msg = IggyMessage::from_str("新订单：宫保鸡丁套餐，地址：成都市武侯区xxx号")?;

    let response = producer.send(vec![msg]).await?;
    for confirmation in &response.confirmations {
        println!(
            "消息发送成功！分区ID: {}，起始偏移量: {}",
            confirmation.partition_id, confirmation.base_offset
        );
    }

    // ===== 第五步：创建消费者并读取消息 =====
    // consumer(name, stream, topic, partition_id) 是"单消费者"，必须绑定一个
    // 已存在的分区：分区 ID 从 0 开始，上面创建的 Topic 只有 1 个分区，所以是 0
    //（写 1 会得到 PartitionNotFound，即使它看起来像"读取 1 条消息"）。
    // 想让多个消费者像投递服务一样分摊分区，改用 client.consumer_group(...)
    let consumer_name = "delivery-service";
    let mut consumer = client
        .consumer(consumer_name, stream_name, topic_name, 0)?
        .batch_length(10)
        .build();

    // init() 必须在第一次读取之前调用
    consumer.init().await?;

    // 读取一条消息
    if let Some(received) = consumer.next().await {
        match received {
            Ok(received) => {
                // 将消息内容从字节转换为字符串
                let content = String::from_utf8_lossy(&received.message.payload);
                println!(
                    "收到消息！偏移量: {}，内容: {}",
                    received.message.header.offset, content
                );
            }
            Err(e) => eprintln!("读取消息失败: {e}"),
        }
    }

    Ok(())
}
