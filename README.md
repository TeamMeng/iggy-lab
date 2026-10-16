# iggy-lab

## 环境设置

### 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 安装开发工具

- `cargo-generate`：使用 GitHub 仓库模板生成新项目
- `cargo-deny`：依赖安全、许可证与来源检查
- `typos-cli`：源码拼写检查
- `git-cliff`：基于 Conventional Commits 生成 Changelog
- `cargo-nextest`：Rust 增强测试运行器
- `just`：常用开发命令任务运行器

```bash
cargo install --locked cargo-generate cargo-deny typos-cli git-cliff cargo-nextest just
```

（如果已安装 [`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall)，可直接免编译安装：`cargo binstall -y cargo-generate cargo-deny typos-cli git-cliff cargo-nextest just`）

### 使用模板创建新项目

```bash
cargo generate TeamMeng/template
```

### 安装并启用 pre-commit

```bash
pipx install pre-commit
pre-commit install
```

## 启动 Iggy 服务端

```bash
docker compose up -d     # 启动（首次自动拉镜像）
docker compose logs -f   # 查看日志
docker compose down      # 停止（数据保留在 iggy-data 卷）
```

健康检查与运行示例：

```bash
curl http://127.0.0.1:3000/ping    # pong
cargo run --example quick_start    # Connect successfully...
```

> 镜像 tag 与 iggy SDK 版本号是两套编号，二进制协议必须配对（当前已验证：
> `apache/iggy:0.9.1-edge.1` ↔ `iggy = "0.11.0"`）。换镜像 tag 之前先看
> [compose.yaml](compose.yaml) 顶部注释，否则会出现 `Incompatible binary
> protocol version` 或服务端直接起不来。

## 常用命令

```bash
# 代码格式化
just fmt

# 静态检查（typos + cargo-deny + fmt + clippy）
just lint

# 运行测试（cargo-nextest）
just test

# 运行全量检查与测试
just check

# 生成 CHANGELOG.md
just changelog
```
