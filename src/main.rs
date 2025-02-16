#![no_std] // Don't link the Rust standart library.
#![no_main] // Disable all Rust-level entry points.
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;
mod vga_buffer;

// Core imports.
use core::panic::PanicInfo;

// Crates imports.
use x86_64::instructions::port::Port;

// Project imports.
use vga_buffer::GREEN_ON_BLACK;

const ISA_DEBUG_EXIT_IOBASE: u16 = 0xF4;

// QEMU exit status: (value << 1) | 1
// 1 is QEMU fails to run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    let mut port = Port::new(ISA_DEBUG_EXIT_IOBASE);
    unsafe {
        // SAFETY: it is the port and size specified in Cargo.toml.
        port.write(exit_code as u32);
    }
}

// This function is the entry point, since the linker looks for a function
// named `_start` by default.
// Used to setup up before the main or the test_main.
#[no_mangle] // Don't mangle the name of this funciton.
pub extern "C" fn _start() -> ! {
    println!("System booted.");
    if cfg!(test) {
        #[cfg(test)]
        test_main();
    } else {
        main();
    }

    eprintln!("Endless loop...");
    #[allow(clippy::empty_loop)]
    loop {}
}

/// This function is called on panic in non test config.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    eprintln!("{info}");
    println!("fin de l'erreur");
    loop {}
}

// This is the main.
fn main() {
    println!("AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz");
    println!("&,?;.:/!*%^$<=>{{[()]}}'\"\\");
    println!("0123456789");

    colored_println!(GREEN_ON_BLACK, "{}", (0x10 << 1) | 1);

    panic!("Une erreur en rouge sur du blanc!");
}

#[cfg(test)]
mod test_os {
    use super::*;

    /// This function is called on panic in test config.
    #[panic_handler]
    fn panic(info: &PanicInfo) -> ! {
        serial_println!("[failed]\n");
        serial_println!("Error: {}\n", info);
        exit_qemu(QemuExitCode::Failed);
        loop {}
    }

    pub trait Testable {
        fn run(&self);
    }

    impl<T> Testable for T
    where
        T: Fn(),
    {
        fn run(&self) {
            serial_print!("{}...\t", core::any::type_name::<T>());
            self();
            serial_println!("[ok]");
        }
    }

    // This is the main for the tests.
    pub fn test_runner(tests: &[&dyn Testable]) {
        serial_println!("Running {} tests...", tests.len());
        for test in tests {
            test.run();
        }
        exit_qemu(QemuExitCode::Success);
    }
}
