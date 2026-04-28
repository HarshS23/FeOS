/*
=========================================================================
|                            Compilation Command                        |
=========================================================================
        Compulation code for mac os : cargo rustc -- -C link-args="-e __start -static -nostartfiles"


        This command runs the current code on QEMU:
        qemu-system-x86_64 -drive format=raw,file=target/x86_64-FeOSMK1/debug/bootimage-FeOSMK1.bin

*/


/*

Things to implement 
    1) panic handler 
    2) disabling Unwinding 
    3) Start attribute 
    
*/ 


/*
=========================================================================
|                            Panic Handler                              |
=========================================================================
        - The compiler invokes this function if a panic occurs 
arguments: 
        - the PanicInfo parameter is file and line where the panic happened

return: 
        - the function should never return so we use the !
        - ! means that this function will never return 
        
*/


#![no_std] // dont link the rust standard library

#![no_main] // disable all rust level entry points

// modules --> like #include in C/C++
mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    // This function is the entry point, since the linker looks for a 
    // function  named _start by default
    loop{}
}

//static HELLO: &[u8] = b"Hello World!";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! { // no return 
    let vga_buffer = 0xb8000 as *mut u8;

    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }


    loop {}
}

// cargo build --target thumbv7em-none-eabihf
// we use a custom target that describes the x86_64 bit architecture