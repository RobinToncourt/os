#![no_std] // Don't link the Rust standart library.
#![no_main] // Disable all Rust-level entry points.
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use os::{colored_println, eprintln, println};

use os::vga_buffer::GREEN_ON_BLACK;

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

    eprintln!("Endless loop...");
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
    println!("AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz");
    println!("&,?;.:/!*%^$<=>{{[()]}}'\"\\");
    println!("0123456789");

    colored_println!(GREEN_ON_BLACK, "{}", (0x10 << 1) | 1);

    panic!("Une erreur en rouge sur du blanc!");
}
