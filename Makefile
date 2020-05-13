build:
	cargo build -Zbuild-std=core,alloc
	ld -nostdlib -T os.ld target/cargo_settings/debug/librust_bug.a
