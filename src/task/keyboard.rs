use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;

use core::{pin::Pin, task::{Poll, Context}};
use futures_util::{stream::{Stream, StreamExt}, task::AtomicWaker};
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};

use crate::{println, print};
use crate::vga_buffer::VGA_BUFFER_WRITER;

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

static WAKER: AtomicWaker = AtomicWaker::new();

static KEYBOARD: spin::Mutex<Keyboard<layouts::Azerty, ScancodeSet1>> = spin::Mutex::new(
    Keyboard::new(ScancodeSet1::new(), layouts::Azerty, HandleControl::Ignore),
);

/// Called by the keyboard interrupt handler.
///
/// Must not block or allocate.
pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if queue.push(scancode).is_ok() {
            WAKER.wake();
        } else {
            println!("WARNING: scancode queue full; dropping keyboard input");
        }
    } else {
        println!("WARNING: scancode queue uninitialized");
    }
}

#[derive(Default)]
pub struct ScancodeStream {
    _private: (),
}

impl ScancodeStream {
    /// # Panics
    ///
    /// Can panic if `SCANCODE_QUEUE` is already initialized.
    pub fn new() -> Self {
        SCANCODE_QUEUE.try_init_once(|| ArrayQueue::new(100))
            .expect("ScancodeStream::new should only be called once");
        ScancodeStream { _private: () }
    }
}

impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        let queue = SCANCODE_QUEUE.try_get().expect("not initialized");

        // fast path
        if let Some(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode));
        }

        WAKER.register(cx.waker());
        match queue.pop() {
            Some(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            None => Poll::Pending,
        }
    }
}

pub async fn print_keypresses() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = KEYBOARD.lock();

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => match character {
                        '\u{8}' => {
                            VGA_BUFFER_WRITER.lock().remove_last();
                        }
                        '\n' => {
                            print!("\n");
                        }
                        _ => {
                            print!("{}", character);
                        }
                    },
                    DecodedKey::RawKey(_key_code) => {}
                }
            }
        }
    }
}
