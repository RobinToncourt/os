#![no_std] // Don't link the Rust standart library.
#![no_main] // Disable all Rust-level entry points.

mod vga_buffer;

use core::panic::PanicInfo;
use vga_buffer::GREEN_ON_BLACK;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    eprintln!("{info}");
    println!("fin de l'erreur");
    loop {}
}

// This function is the entry point, since the linker looks for a function
// named `_start` by default.
#[no_mangle] // Don't mangle the name of this funciton.
pub extern "C" fn _start() -> ! {
    println!("AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz");
    println!("&,?;.:/!*%^$<=>{{[()]}}'\"\\");
    println!("0123456789");

    colored_println!(GREEN_ON_BLACK, "te{}t", "s");

    panic!("Une erreur en rouge sur du blanc!");

    loop {}
}
