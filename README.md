`cargo build -Zbuild-std=core,alloc` fails with the following error:
```
error[E0465]: multiple rlib candidates for `compiler_builtins` found
```

`cargo xbuild` succeeds, but linking the output from it fails with the following error:
```
undefined reference to `memcpy'
```

[rust-lang/wg-cargo-std-aware#53](https://github.com/rust-lang/wg-cargo-std-aware/issues/53) seems to be related.
