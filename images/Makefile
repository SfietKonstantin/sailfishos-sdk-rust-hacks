all: i486 armv7hl aarch64

i486:
	cd i486 && docker build . --tag sailfishos-sdk-rust-i486

armv7hl:
	cd armv7hl && docker build . --tag sailfishos-sdk-rust-armv7hl

aarch64:
	cd aarch64 && docker build . --tag sailfishos-sdk-rust-aarch64

.PHONY: i486 armv7hl aarch64
