#![no_std]  // Don't link the Rust standart library.
#![no_main] // Disable all Rust-level entry points.

use core::panic::PanicInfo;

#[no_mangle] // Don't mangle the name of this funciton.
pub extern "C" fn _start() -> ! {
    // This function is the entry point, since the linker looks for a function
    // named `_start` by default.
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
