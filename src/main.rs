#![no_std] // Don't link the Rust standart library.
#![no_main] // Disable all Rust-level entry points.
#![feature(custom_test_frameworks)]
#![feature(ascii_char)]
#![test_runner(os::test_utils::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};
use x86_64::{
    structures::paging::{Page, Translate},
    VirtAddr,
};

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

fn main(boot_info: &'static BootInfo) {
    colored_println!(GREEN_ON_BLACK, "bonjour en vert!");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe {
        // SAFETY: complete physical memory is mapped to virtual memory
        // at the provided offset, also it is called once here.
        memory::init(phys_mem_offset)
    };
    let mut frame_allocator = unsafe { memory::BootInfoFrameAllocator::new(&boot_info.memory_map) };

    // Map an unused page.
    let page = Page::containing_address(VirtAddr::new(0x0dea_dbea_f000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // Write the string `New!` to the screen through the new mapping.
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe {
        page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e);
    };

    let addresses = [
        // The identity-mapped vga buffer page.
        0xb8000,
        // Some code page.
        0x0020_1008,
        // Some stack page.
        0x0100_0020_1a10,
        // Virtual address mapped to physical address 0.
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate(virt);
        println!("{virt:?} -> {phys:?}");
    }
}
