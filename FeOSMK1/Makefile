make: bootimage build 

run:
	cargo build 
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-FeOSMK1/debug/bootimage-FeOSMK1.bin
	
bootimage:
	cargo bootimage

terminate:
	pkill qemu-system-x86_64

clean:
	rm -rf FeOSMK1/target/x86_64-FeOSMK1