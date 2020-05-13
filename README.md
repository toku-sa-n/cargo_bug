Both `cargo xbuild` and `cargo build -Zbuild-std=core,alloc` succeed. However, they emit different errors when linking.

Linking after `cargo xbuild`
```
ld: target/cargo_settings/debug/librust_bug.a: error adding symbols: file format not recognized
```

Linking after `cargo build -Zbuild-std=core,alloc`
```
undefined reference to `memcpy'
```
