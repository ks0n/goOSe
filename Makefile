check:
	cargo check -Zbuild-std=panic_abort
build:
	cargo build -Zbuild-std=panic_abort
run:
	cargo run -Zbuild-std=panic_abort
test:
	cargo test -Zbuild-std=panic_abort
clean:
	cargo clean
