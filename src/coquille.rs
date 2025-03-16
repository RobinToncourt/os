use lazy_static::lazy_static;
use pc_keyboard::DecodedKey;
use spin::Mutex;

use crate::stack_string::StackString;
use crate::vga_buffer::VGA_BUFFER_WRITER;
use crate::{print, println};

const COMMAND_MAX_LENGTH: usize = 256;

lazy_static! {
    pub static ref COQUILLE: Mutex<Coquille> = Mutex::new(Coquille::default());
}

#[derive(Default)]
pub struct Coquille {
    current_characters: StackString<COMMAND_MAX_LENGTH>,
}

impl Coquille {
    pub fn push_key(&mut self, key: DecodedKey) {
        match key {
            DecodedKey::Unicode(character) => match character {
                '\u{8}' => {
                    VGA_BUFFER_WRITER.lock().backspace();
                    let _ = self.current_characters.pop();
                }
                '\n' => {
                    print!("\n");
                    exec_command(self.current_characters.get_data());
                    self.current_characters.clear();
                }
                _ => {
                    if self.current_characters.push(character).is_err() {
                        println!("limit");
                    }
                    print!("{}", character);
                }
            },
            DecodedKey::RawKey(_key_code) => {}
        }
    }
}

fn exec_command(command: &[char]) {
    let (length, binding) = char_slice_to_utf8_slice(command);
    let command: &str = str::from_utf8(&binding[..length]).expect("not valid utf-8");

    println!("'{command}'");
}

fn char_slice_to_utf8_slice(char_slice: &[char]) -> (usize, [u8; COMMAND_MAX_LENGTH * 4]) {
    let mut utf8_bytes = [0u8; COMMAND_MAX_LENGTH * 4];

    let mut i = 0;
    for c in char_slice {
        let _ = c.encode_utf8(&mut utf8_bytes[i..]);
        i += get_slice_first_zero(&mut utf8_bytes[i..]);
    }

    (i, utf8_bytes)
}

fn get_slice_first_zero(slice: &[u8]) -> usize {
    let mut result = 0;
    let mut iter = slice.iter();

    while let Some(byte) = iter.next() {
        if *byte == 0 {
            break;
        } else {
            result += 1;
        }
    }

    result
}
