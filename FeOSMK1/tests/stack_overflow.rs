#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
//use FeOSMK1::serial_println;
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::InterruptStackFrame;
use FeOSMK1::{serial_println, exit_qemu, QEMUExitCode};


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> !{
    serial_println!("STACK OVER FLOW | stack_overflow::stack_overflow --> \t");

    FeOSMK1::gdt::init();

    // initilize the test interupt deescriptor table 
    init_test_idt();
    // trigger a stack over flow 
    stack_overflow();


    panic!("Execution continued after stack Overflow ");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    FeOSMK1::test_panic_handler(info)
}

/*
            Making the test 

*/

#[allow(unconditional_recursion)]
fn stack_overflow(){
    stack_overflow(); // this will push the address of this function onto the stack
    volatile::Volatile::new(0).read();
}


extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame, _error_code: u64) -> !
{
    serial_println!("RESULT: TEST PASSED");
    exit_qemu(QEMUExitCode::Success);
    loop{}
}

lazy_static!{
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(FeOSMK1::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

pub fn init_test_idt(){
    TEST_IDT.load();
}