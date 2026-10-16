default: check

# Format code
fmt:
    cargo fmt --all

# Run all static checks and lints
lint:
    typos
    cargo deny check
    cargo fmt --all -- --check
    cargo clippy --workspace --all-targets --all-features -- -D warnings

# Run unit and integration tests
test:
    cargo nextest run --workspace --all-features

# Run full check suite
check: lint test

# Generate CHANGELOG.md with git-cliff
changelog:
    git-cliff -o CHANGELOG.md
