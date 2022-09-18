target-linux := "x86_64-unknown-linux-musl"
target-win := "x86_64-pc-windows-gnu"
name := "kvstore-rs"

# build for linux & win64 on a linux host
default: build-linux build-win

build-host:
    cargo build --release

build-linux:
    rustup target add {{target-linux}}
    cargo build --release --target {{target-linux}}
    upx --best --lzma ./target/{{target-linux}}/release/{{name}}

build-win:
    cross build --release --target {{target-win}}
    upx --best --lzma ./target/{{target-win}}/release/{{name}}.exe
