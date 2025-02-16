use core::fmt;

use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

const VGA_TEXT_BUFFER_ADDRESS: usize = 0xb8000;

pub const DEFAULT_COLOR_CODE: ColorCode = ColorCode::new(Color::White, Color::Black);
pub const WHITE_ON_BLACK: ColorCode = ColorCode::new(Color::White, Color::Black);
pub const RED_ON_BLACK: ColorCode = ColorCode::new(Color::Red, Color::Black);
pub const RED_ON_WHITE: ColorCode = ColorCode::new(Color::Red, Color::White);
pub const GREEN_ON_BLACK: ColorCode = ColorCode::new(Color::Green, Color::Black);
pub const BLUE_ON_BLACK: ColorCode = ColorCode::new(Color::Blue, Color::Black);

lazy_static! {
    static ref VGA_BUFFER_WRITER: Mutex<VgaBufferWriter> = Mutex::new(VgaBufferWriter {
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
    const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode(((background as u8) << 4) | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

/// A structure representing the VGA text buffer.
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

struct VgaBufferWriter {
    column_position: usize,
    buffer: &'static mut Buffer,
}

impl VgaBufferWriter {
    fn write_string_color(&mut self, s: &str, color_code: ColorCode) {
        for byte in s.bytes() {
            let b = match byte {
                // Printable ASCII byte or newline.
                0x20..=0x7e | b'\n' => byte,
                // Not part of the printable ASCII range.
                _ => 0xfe,
            };

            self.write_byte(b, color_code);
        }
    }

    fn write_byte(&mut self, byte: u8, color_code: ColorCode) {
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
            let blank = ScreenChar {
                ascii_character: b' ',
                color_code: DEFAULT_COLOR_CODE,
            };
            self.buffer.chars[row][col].write(blank);
        }
    }
}

struct StandardOutput;
impl fmt::Write for StandardOutput {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        VGA_BUFFER_WRITER
            .lock()
            .write_string_color(s, DEFAULT_COLOR_CODE);
        Ok(())
    }
}

#[doc(hidden)]
pub fn vga_buffer_print(args: fmt::Arguments) {
    use core::fmt::Write;
    { StandardOutput }.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::vga_buffer_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

struct StandardError;
impl fmt::Write for StandardError {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        VGA_BUFFER_WRITER.lock().write_string_color(s, RED_ON_WHITE);
        Ok(())
    }
}

#[doc(hidden)]
pub fn vga_buffer_eprint(args: fmt::Arguments) {
    use core::fmt::Write;
    { StandardError }.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => ($crate::vga_buffer::vga_buffer_eprint(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! eprintln {
    () => ($crate::eprint!("\n"));
    ($($arg:tt)*) => ($crate::eprint!("{}\n", format_args!($($arg)*)));
}

struct ColoredStandardOutput(ColorCode);
impl fmt::Write for ColoredStandardOutput {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        VGA_BUFFER_WRITER.lock().write_string_color(s, self.0);
        Ok(())
    }
}

#[doc(hidden)]
pub fn vga_buffer_colored_print(color_code: ColorCode, args: fmt::Arguments) {
    use core::fmt::Write;
    { ColoredStandardOutput(color_code) }
        .write_fmt(args)
        .unwrap();
}

#[macro_export]
macro_rules! colored_print {
    ($color_code:tt, $($arg:tt)*) => ($crate::vga_buffer::vga_buffer_colored_print($color_code, format_args!($($arg)*)));
}

#[macro_export]
macro_rules! colored_println {
    () => ($crate::colored_print!("\n"));
    ($color_code:tt, $($arg:tt)*) => ($crate::colored_print!($color_code, "{}\n", format_args!($($arg)*)));
}
