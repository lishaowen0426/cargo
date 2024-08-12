build: 
    cargo build --release
    ln -fs $(pwd)/target/release/cargo /Users/ziyuanliu/Code/rust-isolation/rust/build/host/stage1/bin/cargo
