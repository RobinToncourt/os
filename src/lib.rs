#![no_std]
#![feature(abi_x86_interrupt)]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_utils::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod collections;
pub mod gdt;
pub mod interrupts;
pub mod serial;
pub mod stack_string;
pub mod vga_buffer;

pub mod test_utils;

use x86_64::instructions::port::Port;

const ISA_DEBUG_EXIT_IOBASE: u16 = 0xF4;

// QEMU exit status: (value << 1) | 1
// 1 is QEMU failed to run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    let mut port = Port::new(ISA_DEBUG_EXIT_IOBASE);
    unsafe {
        // SAFETY: it is the port and size specified in Cargo.toml.
        port.write(exit_code as u32);
    }
}

pub fn init() {
    interrupts::init_idt();
    gdt::init();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}
