# build for linux & win64 on a linux host
default: build-linux build-win

build-host:
    cargo build --release

build-linux:
    rustup target add x86_64-unknown-linux-musl
    cargo build --release --target x86_64-unknown-linux-musl

build-win:
  cross build --release --target x86_64-pc-windows-gnu
