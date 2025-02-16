use core::fmt::Write;
use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe {
            // SAFETY: standard port number for the first serial interface.
            SerialPort::new(0x3F8)
        };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[allow(dead_code)]
pub fn uart_16550_print(args: ::core::fmt::Arguments) {
    SERIAL1
        .lock()
        .write_fmt(args)
        .expect("Printing to serial failed.");
}

/// Prints to the hast through the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($($args:tt)*) => {
        $crate::serial::uart_16550_print(format_args!($($args)*))
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}
