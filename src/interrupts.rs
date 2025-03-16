use lazy_static::lazy_static;
use pc_keyboard::{layouts, HandleControl, Keyboard, ScancodeSet1};
use pic8259::ChainedPics;
use spin;
use x86_64::instructions::port::Port;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::coquille::COQUILLE;
use crate::gdt;
use crate::println;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(unsafe {
    // SAFETY: the ports are not used.
    ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)
});

static KEYBOARD: spin::Mutex<Keyboard<layouts::Azerty, ScancodeSet1>> = spin::Mutex::new(
    Keyboard::new(ScancodeSet1::new(), layouts::Azerty, HandleControl::Ignore),
);

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            // SAFETY: the index is not used by another exception.
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt[InterruptIndex::Timer.into_usize()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.into_usize()].set_handler_fn(keyboard_interrupt_handler);
        idt
    };

}

#[derive(Debug, Clone)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn into_u8(self) -> u8 {
        self as u8
    }

    fn into_usize(self) -> usize {
        usize::from(self.into_u8())
    }
}

pub fn init_idt() {
    println!("Initializing idt.");
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    unsafe {
        // SAFETY: timer interrupt handled.
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.into_u8());
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe {
        // SAFETY: keyboard data port is 0x60.
        port.read()
    };

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            COQUILLE.lock().push_key(key);
        }
    }

    unsafe {
        // SAFETY: keyboard interrupt handled.
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.into_u8());
    }
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

#[cfg(test)]
mod interrupts_tests {
    #[test_case]
    fn test_breakpoint_exception() {
        x86_64::instructions::interrupts::int3();
    }
}
