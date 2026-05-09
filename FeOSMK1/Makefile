make: bootimage build 

bootimage:
	cargo bootimage

run: 
	cargo bootimage
	cargo build 
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-FeOSMK1/debug/bootimage-FeOSMK1.bin

test:
	cargo test


terminate:
	pkill qemu-system-x86_64

clean:
	rm -rf target
	# test