#![no_std]
#![no_main]



use core::panic::{self, PanicInfo};
use FeOSMK1::{QEMUExitCode, exit_qemu, serial_println};
use FeOSMK1::serial_print;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("OKAY");
    exit_qemu(QEMUExitCode::Success);
    loop {}

}


fn should_fail(){
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("TEST DID NOT PANIC");
    exit_qemu(QEMUExitCode::Failed);
    loop {}
}


/*
                            Making the test 
*/

