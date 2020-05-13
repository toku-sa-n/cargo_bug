build:
	cargo build -Zbuild-std=core,alloc
	ld -nostdlib target/cargo_settings/debug/librust_bug.a
