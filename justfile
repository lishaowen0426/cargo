build: 
    cargo build 
    ln -fs $(pwd)/target/release/cargo /Users/ziyuanliu/Code/rust-isolation/rust/build/host/stage1/bin/cargo
