#![no_std] // Don't link the Rust standard library.
#![no_main] // Disable all Rust-level entry points.
#![feature(custom_test_frameworks)]
#![feature(ascii_char)]
#![test_runner(os::test_utils::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};

#[cfg(not(test))]
use os::eprintln;
use os::vga_buffer::GREEN_ON_BLACK;
use os::{colored_println, println};
use os::task::keyboard;

#[cfg(test)]
use os::test_utils::test_panic_handler;

entry_point!(kernel_main);

// This function is the entry point.
// Used to set up before the main or the test_main.
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    os::init(boot_info);

    println!("System booted.");
    if cfg!(test) {
        #[cfg(test)]
        test_main();
    } else {
        main();
    }

    os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    eprintln!("Panic!\n{info}");
    os::hlt_loop();
}

use os::task::{Task, executor::Executor};

fn main() {
    colored_println!(GREEN_ON_BLACK, "Bonjour en vert !");

    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}
