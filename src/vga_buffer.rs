use core::fmt;

use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

const VGA_TEXT_BUFFER_ADDRESS: usize = 0xb8000;
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

const ALL_CODE_PAGE437_CHARACTER: &str = "☺☻♥♦♣♠•◘○◙♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼ !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~⌂ÇüéâäàåçêëèïîìÄÅÉæÆôöòûùÿÖÜ¢£¥₧ƒáíóúñÑªº¿⌐¬½¼¡«»░▒▓│┤╡╢╖╕╣║╗╝╜╛┐└┴┬├─┼╞╟╚╔╩╦╠═╬╧╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞φε∩≡±≥≤⌠⌡÷≈°∙·√ⁿ²■";

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
                self.write_byte('o' as u8, color_code);
                'e' as u8
            } else if c == 'Œ' {
                self.write_byte('O' as u8, color_code);
                'E' as u8
            } else {
                match c {
                    '☺' => 0x01,
                    '☻' => 0x02,
                    '♥' => 0x03,
                    '♦' => 0x04,
                    '♣' => 0x05,
                    '♠' => 0x06,
                    '•' => 0x07,
                    '◘' => 0x08,
                    '○' => 0x09,
                    '◙' => 0x0a,
                    '♂' => 0x0b,
                    '♀' => 0x0c,
                    '♪' => 0x0d,
                    '♫' => 0x0e,
                    '☼' => 0x0f,
                    '►' => 0x10,
                    '◄' => 0x11,
                    '↕' => 0x12,
                    '‼' => 0x13,
                    '¶' => 0x14,
                    '§' => 0x15,
                    '▬' => 0x16,
                    '↨' => 0x17,
                    '↑' => 0x18,
                    '↓' => 0x19,
                    '→' => 0x1a,
                    '←' => 0x1b,
                    '∟' => 0x1c,
                    '↔' => 0x1d,
                    '▲' => 0x1e,
                    '▼' => 0x1f,
                    '⌂' => 0x7f,
                    'Ç' => 0x80,
                    'ü' => 0x81,
                    'é' => 0x82,
                    'â' => 0x83,
                    'ä' => 0x84,
                    'à' => 0x85,
                    'å' => 0x86,
                    'ç' => 0x87,
                    'ê' => 0x88,
                    'ë' => 0x89,
                    'è' => 0x8a,
                    'ï' => 0x8b,
                    'î' => 0x8c,
                    'ì' => 0x8d,
                    'Ä' => 0x8e,
                    'Å' => 0x8f,
                    'É' => 0x90,
                    'æ' => 0x91,
                    'Æ' => 0x92,
                    'ô' => 0x93,
                    'ö' => 0x94,
                    'ò' => 0x95,
                    'û' => 0x96,
                    'ù' => 0x97,
                    'ÿ' => 0x98,
                    'Ö' => 0x99,
                    'Ü' => 0x9a,
                    '¢' => 0x9b,
                    '£' => 0x9c,
                    '¥' => 0x9d,
                    '₧' => 0x9e,
                    'ƒ' => 0x9f,
                    'á' => 0xa0,
                    'í' => 0xa1,
                    'ó' => 0xa2,
                    'ú' => 0xa3,
                    'ñ' => 0xa4,
                    'Ñ' => 0xa5,
                    'ª' => 0xa6,
                    'º' => 0xa7,
                    '¿' => 0xa8,
                    '⌐' => 0xa9,
                    '¬' => 0xaa,
                    '½' => 0xab,
                    '¼' => 0xac,
                    '¡' => 0xad,
                    '«' => 0xae,
                    '»' => 0xaf,
                    '░' => 0xb0,
                    '▒' => 0xb1,
                    '▓' => 0xb2,
                    '│' => 0xb3,
                    '┤' => 0xb4,
                    '╡' => 0xb5,
                    '╢' => 0xb6,
                    '╖' => 0xb7,
                    '╕' => 0xb8,
                    '╣' => 0xb9,
                    '║' => 0xba,
                    '╗' => 0xbb,
                    '╝' => 0xbc,
                    '╜' => 0xbd,
                    '╛' => 0xbe,
                    '┐' => 0xbf,
                    '└' => 0xc0,
                    '┴' => 0xc1,
                    '┬' => 0xc2,
                    '├' => 0xc3,
                    '─' => 0xc4,
                    '┼' => 0xc5,
                    '╞' => 0xc6,
                    '╟' => 0xc7,
                    '╚' => 0xc8,
                    '╔' => 0xc9,
                    '╩' => 0xca,
                    '╦' => 0xcb,
                    '╠' => 0xcc,
                    '═' => 0xcd,
                    '╬' => 0xce,
                    '╧' => 0xcf,
                    '╨' => 0xd0,
                    '╤' => 0xd1,
                    '╥' => 0xd2,
                    '╙' => 0xd3,
                    '╘' => 0xd4,
                    '╒' => 0xd5,
                    '╓' => 0xd6,
                    '╫' => 0xd7,
                    '╪' => 0xd8,
                    '┘' => 0xd9,
                    '┌' => 0xda,
                    '█' => 0xdb,
                    '▄' => 0xdc,
                    '▌' => 0xdd,
                    '▐' => 0xde,
                    '▀' => 0xdf,
                    'α' => 0xe0,
                    'ß' => 0xe1,
                    'Γ' => 0xe2,
                    'π' => 0xe3,
                    'Σ' => 0xe4,
                    'σ' => 0xe5,
                    'µ' => 0xe6,
                    'τ' => 0xe7,
                    'Φ' => 0xe8,
                    'Θ' => 0xe9,
                    'Ω' => 0xea,
                    'δ' => 0xeb,
                    '∞' => 0xec,
                    'φ' => 0xed,
                    'ε' => 0xee,
                    '∩' => 0xef,
                    '≡' => 0xf0,
                    '±' => 0xf1,
                    '≥' => 0xf2,
                    '≤' => 0xf3,
                    '⌠' => 0xf4,
                    '⌡' => 0xf5,
                    '÷' => 0xf6,
                    '≈' => 0xf7,
                    '°' => 0xf8,
                    '∙' => 0xf9,
                    '·' => 0xfa,
                    '√' => 0xfb,
                    'ⁿ' => 0xfc,
                    '²' => 0xfd,
                    '■' => 0xfe,
                    _ => 0xfe,
                }
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

/// # Panics
///
/// Can't panic.
pub fn vga_buffer_print(args: fmt::Arguments) {
    use core::fmt::Write;
    { StandardOutput }.write_fmt(args).unwrap();
}

/// Print to the standard output in white with black background.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::vga_buffer_print(format_args!($($arg)*)));
}

/// Print to the standard output in white with black background,
/// appending a newline.
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

/// # Panics
///
/// Can't panic.
pub fn vga_buffer_eprint(args: fmt::Arguments) {
    use core::fmt::Write;
    { StandardError }.write_fmt(args).unwrap();
}

/// Print to the standard output in red with white background.
#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => ($crate::vga_buffer::vga_buffer_eprint(format_args!($($arg)*)));
}

/// Print to the standard output in red with white background,
/// appending a newline.
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

/// # Panics
///
/// Can't panic.
pub fn vga_buffer_colored_print(color_code: ColorCode, args: fmt::Arguments) {
    use core::fmt::Write;
    { ColoredStandardOutput(color_code) }
        .write_fmt(args)
        .unwrap();
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
        println!("{s}");
        for (i, c) in s.chars().enumerate() {
            let screen_char = VGA_BUFFER_WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    }
}
