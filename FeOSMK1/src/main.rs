#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
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


#![no_std] // dont link the rust standard library

#![no_main] // disable all rust level entry points

// making a custom testing frame work 

// modules --> like #include in C/C++
mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> !{
        println!("{}", info);
        loop{}
}


#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
        println!("Running Tests: {}", tests.len());
        for i in tests{
                i();
        }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("This is a floating point number: {}", 5.12980);
    #[cfg(test)]
    test_main();
    //panic!("some panic message");
    loop {} //- this is unreachable after the panic happens 
}

#[test_case]
fn trivial_asseration(){
        println!("TEST 1| TEST RESULT -->");
        assert_eq!(1,1);
        println!("PASS")
}

// #[test_case]
// fn fail_test(){
//         println!("TEST 2 | TEST RESULT ---> ");
//         assert_eq!(1,0);
//         println!("FAIL");
// }



//static HELLO: &[u8] = b"Hello World!"
// cargo build --target thumbv7em-none-eabihf
// we use a custom target that describes the x86_64 bit architecture

