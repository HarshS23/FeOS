#![no_std] // dont link the rust standard library
#![no_main] // disable all rust level entry points
#![feature(custom_test_frameworks)]
#![test_runner(FeOSMK1::test_runner)]
#![reexport_test_harness_main = "test_main"]


/*
=========================================================================
|                            Compilation Command                        |
=========================================================================
        Compulation code for mac os : cargo rustc -- -C link-args="-e __start -static -nostartfiles"


        This command runs the current code on QEMU:
        cargo bootimage
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


// use core::{arch::x86_64, panic::PanicInfo};
use core::panic::PanicInfo;
use FeOSMK1::println;
// x86_64::instructions::interrupts;





#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World {}", "!");

    FeOSMK1::init();
    
    // invoking the breakpoint exception 
    //x86_64::instructions::interrupts::int3();

    // triggering a page fault 
//     unsafe {
//         *(0xddcebaff as *mut u8) = 42;
//     };
    

    // setting up a tripple fault 
//     fn stack_overflow(){
//         stack_overflow(); // for each recursionn, the return address is pushed 
//     }

    // triggering a stack overflow
    //stack_overflow();

    #[cfg(test)]
    test_main();

    println!("It did not crash");

    loop { x86_64::instructions::hlt(); }

    //panic!("some panic message");
    //loop {} //- this is unreachable after the panic happens 
    

}


#[cfg(not(test))]  // for not test 
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
        println!("{}",info);
        loop{}
}


#[cfg(test)] // for test
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
        FeOSMK1::test_panic_handler(info)
}


/*
================================================================
                        Double Faults
================================================================
*/

