use uart_16550::{Config, Uart16550Tty, backend::PioBackend};
use spin::Mutex;
use lazy_static::lazy_static;

// serial is basically a comunication interface in which we can send 
// data transfer in and out sequentially one bit at a time 
// It is easy to program and QEMU can redirect the 
// bytes sent over serial to the hosts standard output or a file.

// use lazy_static and spin lock to create a static writer instance 
// by using lazy_static, it make sure that its only initlized once 
lazy_static! {
    pub static ref SERIAL1: Mutex<Uart16550Tty<PioBackend>> = Mutex::new(unsafe {
        Uart16550Tty::new_port(0x3F8, Config::default())
            .expect("failed to initialize UART")
    });
}

// did not write this code needed help 
// now making every port easily readable 
#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments){
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).expect("Printing to serial failed (serial.rs)");
}

// print to host through serial interface 
#[macro_export]
macro_rules! serial_print { 
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}

