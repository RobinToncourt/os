#![no_std] // Don't link the Rust standart library.
#![no_main] // Disable all Rust-level entry points.
#![feature(custom_test_frameworks)]
#![feature(ascii_char)]
#![test_runner(os::test_utils::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[cfg(not(test))]
use os::eprintln;
use os::{println, eprint, colored_println};
use os::vga_buffer::GREEN_ON_BLACK;

#[cfg(test)]
use os::test_utils::test_panic_handler;

// This function is the entry point, since the linker looks for a function
// named `_start` by default.
// Used to setup up before the main or the test_main.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    os::init();

    println!("System booted.");
    if cfg!(test) {
        #[cfg(test)]
        test_main();
    } else {
        main();
    }

    eprint!("Endless loop...");
    #[allow(clippy::empty_loop)]
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    eprintln!("Panic!");
    eprintln!("{info}");
    loop {}
}

fn main() {
    println!("Le cœur déçu mais l'âme plutôt naïve, Louÿs rêva de crapaüter en canoë au delà des îles, près du mälström où brûlent les novæ.");
    println!("À, Â, È, Ê, Ë, Î, Ï, Ô, Œ, œ, Ù, Û, Ÿ");
    println!("{}", os::vga_buffer::ALL_CODE_PAGE437_CHARACTER);

    colored_println!(GREEN_ON_BLACK, "bonjour en vert!");
}
