use core::fmt;

use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

use crate::code_page_437::UTF_8_TO_CODE_PAGE_437_MAP;
use crate::stack_string::StackString;

const VGA_TEXT_BUFFER_ADDRESS: usize = 0xb8000;
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[allow(dead_code)]
pub const DEFAULT_COLOR_CODE: ColorCode = ColorCode::new(Color::White, Color::Black);
#[allow(dead_code)]
pub const WHITE_ON_BLACK: ColorCode = ColorCode::new(Color::White, Color::Black);
#[allow(dead_code)]
pub const RED_ON_BLACK: ColorCode = ColorCode::new(Color::Red, Color::Black);
#[allow(dead_code)]
pub const RED_ON_WHITE: ColorCode = ColorCode::new(Color::Red, Color::White);
#[allow(dead_code)]
pub const GREEN_ON_BLACK: ColorCode = ColorCode::new(Color::Green, Color::Black);
#[allow(dead_code)]
pub const BLUE_ON_BLACK: ColorCode = ColorCode::new(Color::Blue, Color::Black);

lazy_static! {
    static ref BLANK_CHAR: ScreenChar = ScreenChar {
        ascii_character: b' ',
        color_code: DEFAULT_COLOR_CODE,
    };

    pub static ref VGA_BUFFER_WRITER: Mutex<VgaBufferWriter> = Mutex::new(VgaBufferWriter {
        column_position: 0,
        buffer: unsafe {
            // SAFETY: the VGA text buffer is at this address.
            &mut *(VGA_TEXT_BUFFER_ADDRESS as *mut Buffer)
        },
    });
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    #[must_use]
    pub const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode(((background as u8) << 4) | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

/// A structure representing the VGA text buffer.
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct VgaBufferWriter {
    column_position: usize,
    buffer: &'static mut Buffer,
}

impl VgaBufferWriter {
    /// Uses code page 437.
    fn write_string_color(&mut self, s: &str, color_code: ColorCode) {
        for c in s.chars() {
            let b = if c.is_ascii() {
                c as u8
            } else if c == 'œ' {
                // œ does not exist in code page 437.
                self.write_byte(b'o', color_code);
                b'e'
            } else if c == 'Œ' {
                self.write_byte(b'O', color_code);
                b'E'
            } else {
                // 0xfe(■): default character.
                *UTF_8_TO_CODE_PAGE_437_MAP.get(&c).unwrap_or(&0xfe)
            };

            self.write_byte(b, color_code);
        }
    }

    pub fn write_byte(&mut self, byte: u8, color_code: ColorCode) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    #[must_use]
    pub fn get_line(&self) -> StackString<BUFFER_WIDTH> {
        let mut result = StackString::<BUFFER_WIDTH>::default();

        let row = BUFFER_HEIGHT - 1;
        for screen_char in &self.buffer.chars[row] {
            let _ = result.push(char::from(screen_char.read().ascii_character));
        }

        result
    }

    pub fn remove_last(&mut self) {
        if self.column_position > 0 {
            self.column_position -= 1;

            let row = BUFFER_HEIGHT - 1;
            let col = self.column_position;

            self.buffer.chars[row][col].write(*BLANK_CHAR);
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(*BLANK_CHAR);
        }
    }
}

impl fmt::Write for VgaBufferWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string_color(s, DEFAULT_COLOR_CODE);
        Ok(())
    }
}

struct ColoredStandardOutput(ColorCode);
impl fmt::Write for ColoredStandardOutput {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        VGA_BUFFER_WRITER.lock().write_string_color(s, self.0);
        Ok(())
    }
}

/// # Panics
///
/// Can't panic.
pub fn vga_buffer_colored_print(color_code: ColorCode, args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        { ColoredStandardOutput(color_code) }
            .write_fmt(args)
            .unwrap();
    });
}

/// Print to the standard output in white with black background.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::vga_buffer_colored_print($crate::vga_buffer::DEFAULT_COLOR_CODE, format_args!($($arg)*)));
}

/// Print to the standard output in white with black background,
/// appending a newline.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// Print to the standard output in red with white background.
#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => ($crate::vga_buffer::vga_buffer_colored_print($crate::vga_buffer::RED_ON_WHITE, format_args!($($arg)*)));
}

/// Print to the standard output in red with white background,
/// appending a newline.
#[macro_export]
macro_rules! eprintln {
    () => ($crate::eprint!("\n"));
    ($($arg:tt)*) => ($crate::eprint!("{}\n", format_args!($($arg)*)));
}

/// Print to the standard output with custom color.
#[macro_export]
macro_rules! colored_print {
    ($color_code:tt, $($arg:tt)*) => ($crate::vga_buffer::vga_buffer_colored_print($color_code, format_args!($($arg)*)));
}

/// Print to the standard output with custom color, appending a newline.
#[macro_export]
macro_rules! colored_println {
    () => ($crate::colored_print!("\n"));
    ($color_code:tt, $($arg:tt)*) => ($crate::colored_print!($color_code, "{}\n", format_args!($($arg)*)));
}

#[cfg(test)]
mod test_vga_buffer {
    use super::*;
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    #[test_case]
    fn test_println_simple() {
        println!("test_println_simple output");
    }

    #[test_case]
    fn test_println_many() {
        for _ in 0..200 {
            println!("test_println_simple output");
        }
    }

    #[test_case]
    fn test_println_output() {
        let s = "Some test string taht fits on a single line.";
        interrupts::without_interrupts(|| {
            let mut writer = VGA_BUFFER_WRITER.lock();
            writeln!(writer, "\n{s}").expect("writeln failed");
            for (i, c) in s.chars().enumerate() {
                let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
                assert_eq!(char::from(screen_char.ascii_character), c);
            }
        });
    }
}
