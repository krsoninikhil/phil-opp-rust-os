use core::fmt::Write;

use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    // not create mutex like vga_buffer to ensure seria_port is initialized before first use
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = SerialPort::new(0x3F8);  // 0x3F8 is standard port number for fist serial interface
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    // `write_fmt` or `write_str` can be directly used as `Write`
    // trait is already implemented for `SerialPort`
    SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");  // use expect instead of unwrap to handle panic also
}

#[macro_export]
macro_rules! serial_print {
    ($($args:tt)*) => ($crate::serial::_print(format_args!($($args)*)));
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}
