Both `cargo xbuild` and `cargo build -Zbuild-std=core,alloc` succeed, but linking fails with the following error:
```
ld: target/cargo_settings/debug/librust_bug.a: error adding symbols: file format not recognized
```
