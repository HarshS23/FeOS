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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
// setting up Qemu exit codes 
pub enum QEMUExitCode{
        Success = 0x10,
        Failed = 0x11,
}

pub fn exit_qemu(exit_code: QEMUExitCode){
        use x86_64::instructions::port::Port;

        unsafe{
                let mut port = Port::new(0xf4); // creating a new port at oxf4
                port.write(exit_code as u32);

        }
}



// making a custom testing frame work 

// modules --> like #include in C/C++
mod vga_buffer;
mod serial;


use core::panic::PanicInfo;

#[cfg(not(test))]  // for not test 
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
        println!("{}",info);
        loop{}
}


#[cfg(test)] // for test
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
        serial_println!("FAILED \n");
        serial_println!("Error Type: {}\n", info);
        exit_qemu(QEMUExitCode::Failed);
        loop{}
}


#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
        //println!("Running Tests: {}", tests.len());
        serial_println!("Running tests: {}", tests.len());
        for test in tests{
                test.run();
        }

        // the qemu exit code 
        exit_qemu(QEMUExitCode::Success);
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("This is a floating point number: {}", 5.12980);
    #[cfg(test)]
    test_main();
    //panic!("some panic message");
    loop {} //- this is unreachable after the panic happens 
}



//  dont want to have multiple serial_printline everywhere 
// so its better to make it automatic
pub trait Testable {
        fn run(&self) -> ();

}

impl<T> Testable for T 
where T: Fn(), 
{
        fn run(&self) -> () {
            serial_print!(" \n{} --> ", core::any::type_name::<T>()); // invoke the funciton name 
            self(); // check if it panics or not 
            serial_println!("RESULT: TEST PASSED");  // prints okay if it did not panic
        }

}


#[test_case]
fn trivial_asseration(){
        // println!("TEST 1| TEST RESULT -->");
        // assert_eq!(1,1);
        // println!("PASS")

        //serial_print!("Triv assersation\n");
        assert_eq!(1,1);
        //serial_println!(" PASS");
}