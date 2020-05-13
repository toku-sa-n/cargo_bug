~~`cargo xbuild` causes the following error.~~ [Solved](https://github.com/rust-osdev/cargo-xbuild/pull/71) with `cargo-xbuild` version 0.5.31. However, `xargo build --target cargo_settings` [still causes the error](https://github.com/japaric/xargo/issues/292).

```
error: failed to get bitcode from object file for LTO (Bitcode section not found in object file)
```
