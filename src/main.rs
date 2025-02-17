#![no_std] // Don't link the Rust standart library.
#![no_main] // Disable all Rust-level entry points.
#![feature(custom_test_frameworks)]
#![feature(ascii_char)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use os::{colored_print, eprint, eprintln, println};

use os::vga_buffer::{DEFAULT_COLOR_CODE, RED_ON_WHITE};

// This function is the entry point, since the linker looks for a function
// named `_start` by default.
// Used to setup up before the main or the test_main.
#[no_mangle]
pub extern "C" fn _start() -> ! {
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
    os::test_panic_handler(info)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    eprintln!("{info}");
    println!("fin de l'erreur");
    loop {}
}

fn main() {
    println!("Le cœur déçu mais l'âme plutôt naïve, Louÿs rêva de crapaüter en canoë au delà des îles, près du mälström où brûlent les novæ.");
    println!("À, Â, È, Ê, Ë, Î, Ï, Ô, Œ, œ, Ù, Û, Ÿ");
}
