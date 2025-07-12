#![no_std] // Don't link the Rust standart library.
#![no_main] // Disable all Rust-level entry points.
#![feature(custom_test_frameworks)]
#![feature(ascii_char)]
#![test_runner(os::test_utils::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::boxed::Box;
use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};
use os::allocator;
use x86_64::VirtAddr;

#[cfg(not(test))]
use os::eprintln;
use os::memory;
use os::vga_buffer::GREEN_ON_BLACK;
use os::{colored_println, println};

#[cfg(test)]
use os::test_utils::test_panic_handler;

entry_point!(kernel_main);

// This function is the entry point.
// Used to setup up before the main or the test_main.
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    os::init();

    println!("System booted.");
    if cfg!(test) {
        #[cfg(test)]
        test_main();
    } else {
        main(boot_info);
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

use alloc::{rc::Rc, vec, vec::Vec};

fn main(boot_info: &'static BootInfo) {
    colored_println!(GREEN_ON_BLACK, "bonjour en vert!");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe {
        // SAFETY: complete physical memory is mapped to virtual memory
        // at the provided offset, also it is called once here.
        memory::init(phys_mem_offset)
    };
    let mut frame_allocator = unsafe {
        // SAFETY: `memory_map` is provided by the boot info.
        memory::BootInfoFrameAllocator::new(&boot_info.memory_map)
    };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initalization failed");

    // Allocate a number on the heap.
    let heap_value = Box::new(42);
    println!("heap_value {heap_value} at {heap_value:p}");

    // Create a dynamically sized vector.
    let mut vec = Vec::<usize>::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // Create a reference counter vector -> will be freed when count reaches 0.
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = Rc::clone(&reference_counted);

    println!(
        "current reference count is {}.",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(reference_counted);
    println!(
        "reference count is {} now.",
        Rc::strong_count(&cloned_reference)
    );
}
