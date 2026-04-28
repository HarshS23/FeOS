/*
=========================================================================
|                            Compilation Command                        |
=========================================================================
        Compulation code for mac os : cargo rustc -- -C link-args="-e __start -static -nostartfiles"

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

// disabling the standard rust library 
#![no_std] // dont link the rust standard library

#![no_main] // disable all rust level entry points

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    // This function is the entry point, since the linker looks for a 
    // function  named _start by default
    loop{}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! { // no return 
    loop {}
}

// cargo build --target thumbv7em-none-eabihf
// we use a custom target that describes the x86_64 bit architecture