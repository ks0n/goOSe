TARGET ?= 'NOTARGETSELECTED'


check:
	cargo check --target $(TARGET) -Zbuild-std=core,alloc 
build:
	cargo build --target $(TARGET) -Zbuild-std=core,alloc
run:
	cargo run --target $(TARGET) -Zbuild-std=core,alloc
test:
	cargo test --target $(TARGET) -Zbuild-std=core,alloc
clean:
	cargo clean
