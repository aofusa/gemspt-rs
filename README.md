# gemspt in Rust
![glass-16spp](https://user-images.githubusercontent.com/18138131/36441739-dadea2a6-16b6-11e8-8fef-af541afd9722.png)  
Simple path tracing implementation for CGGems in Rust  
[Original](https://github.com/githole/gemspt)  

## How to compile and run
cargo build
cargo run

## How to release build and run
cargo rustc --release -- -C opt-level=s -C lto -C link-args=-Wl,-x,-S
target/release/gemspt-rs

