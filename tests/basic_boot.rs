#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_utils::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use os::println;
use os::test_utils::test_panic_handler;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info);
}

#[test_case]
fn test_prinln() {
    println!("test_prinln output");
}
