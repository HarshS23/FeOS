#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

//use x86_64::instructions::interrupts;

pub mod vga_buffer;
pub mod serial;
pub mod interrupts;

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



pub fn test_runner(tests: &[&dyn Testable]) {
        //println!("Running Tests: {}", tests.len());
        serial_println!("Running tests: {}", tests.len());
        for test in tests{
                test.run();
        }

        // the qemu exit code 
        exit_qemu(QEMUExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> !{
        serial_println!("FAILED \n");
        serial_println!("Error Type: {}\n", info);
        exit_qemu(QEMUExitCode::Failed);
        loop{}
}


#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}


/*

moving qemu exit codes into lib.rs 

*/

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


// Interrupts section 
pub fn init(){
    interrupts::init_idt();
}
