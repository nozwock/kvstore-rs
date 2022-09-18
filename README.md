# kvstore-rs

Simple CLI program which stores key-value pairs in a file.

```console
kvstore-rs 0.2.0

USAGE:
    kvstore-rs <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    add       add the key-value pair to the database
    get       return the value corresponding to the given key
    help      Print this message or the help of the given subcommand(s)
    list      lists all key-value pairs in the database
    remove    remove the key-value pair from the database
```

---
Just me messing around with rust >.<  🦀


## 🏗️ Builds

This is how builds for this project are made at the moment.

### :penguin: Linux

- Using `x86_64-unknown-linux-musl` build target
    ```console
    rustup install x86_64-unknown-linux-musl
    cargo build --release --target=x86_64-unknown-linux-musl
    ```

### :toilet: Windows

- Using `x86_64-pc-windows-gnu` build target
    1. First of all, install [cross](https://github.com/cross-rs/cross)
    2. Build using cross
        ```console
        cross build --release --target x86_64-pc-windows-gnu
        ```


### :books: Resources

- [Taking Rust everywhere with rustup](https://blog.rust-lang.org/2016/05/13/rustup.html)
- [min-sized-rust](https://github.com/johnthagen/min-sized-rust)

- **Note:** Using UPX to make an even more compact binary
    ```console
    upx --best --lzma <executable>
    ```
