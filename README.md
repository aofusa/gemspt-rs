# gemspt in Rust
Simple path tracing implementation for CGGems in Rust
[Original](https://github.com/githole/gemspt)

## How to compile and run
cargo build
cargo run

## How to release build
cargo rustc --release -- -C opt-level=s -C lto -C link-args=-Wl,-x,-S

