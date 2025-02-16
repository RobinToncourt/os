#![no_std]  // Don't link the Rust standart library.
#![no_main] // Disable all Rust-level entry points.

mod vga_buffer;

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    loop {}
}

// This function is the entry point, since the linker looks for a function
// named `_start` by default.
#[no_mangle] // Don't mangle the name of this funciton.
pub extern "C" fn _start() -> ! {
    let world = "World";
    println!("Hello {world}{}", "!");

    panic!("Panic test.");

    loop {}
}
