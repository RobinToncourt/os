use core::fmt;
use core::fmt::Write;

use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

const SERIAL_INTERFACE_FIRST_PORT: u16 = 0x3f8;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe {
            // SAFETY: standard port number for the first serial interface.
            SerialPort::new(SERIAL_INTERFACE_FIRST_PORT)
        };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

pub struct Red(pub &'static str);
impl fmt::Display for Red {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[91m")?;
        write!(f, "{}", self.0)?;
        write!(f, "\x1B[0m")?;
        Ok(())
    }
}

pub struct Green(pub &'static str);
impl fmt::Display for Green {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[32m")?;
        write!(f, "{}", self.0)?;
        write!(f, "\x1B[0m")?;
        Ok(())
    }
}

/// # Panics
///
/// Can panic if the serial port is invalid
/// or if the program doesn't have the necessary rights.
#[allow(dead_code)]
pub fn uart_16550_print(args: ::core::fmt::Arguments) {
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        SERIAL1
            .lock()
            .write_fmt(args)
            .expect("Printing to serial failed.");
    });
}

#[macro_export]
macro_rules! serial_print {
    ($($args:tt)*) => {
        $crate::serial::uart_16550_print(format_args!($($args)*))
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}
