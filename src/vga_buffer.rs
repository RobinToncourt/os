use core::fmt;

use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

use crate::collections::double_array_map::DoubleArrayMap;

const VGA_TEXT_BUFFER_ADDRESS: usize = 0xb8000;
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

// ◙ is new line.
#[allow(dead_code)]
pub const ALL_CODE_PAGE437_CHARACTER: &str = "☺☻♥♦♣♠•◘○♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼ !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~⌂ÇüéâäàåçêëèïîìÄÅÉæÆôöòûùÿÖÜ¢£¥₧ƒáíóúñÑªº¿⌐¬½¼¡«»░▒▓│┤╡╢╖╕╣║╗╝╜╛┐└┴┬├─┼╞╟╚╔╩╦╠═╬╧╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞φε∩≡±≥≤⌠⌡÷≈°∙·√ⁿ²■";

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

    static ref UTF_8_TO_CODE_PAGE_437_MAP: DoubleArrayMap<168, char, u8> = {
        let mut map = DoubleArrayMap::new();
        let _ = map.insert('☺', 0x01);
        let _ = map.insert('☻', 0x02);
        let _ = map.insert('♥', 0x03);
        let _ = map.insert('♦', 0x04);
        let _ = map.insert('♣', 0x05);
        let _ = map.insert('♠', 0x06);
        let _ = map.insert('•', 0x07);
        let _ = map.insert('◘', 0x08);
        let _ = map.insert('○', 0x09);
        let _ = map.insert('♂', 0x0b);
        let _ = map.insert('♀', 0x0c);
        let _ = map.insert('♪', 0x0d);
        let _ = map.insert('♫', 0x0e);
        let _ = map.insert('☼', 0x0f);
        let _ = map.insert('►', 0x10);
        let _ = map.insert('◄', 0x11);
        let _ = map.insert('↕', 0x12);
        let _ = map.insert('‼', 0x13);
        let _ = map.insert('¶', 0x14);
        let _ = map.insert('§', 0x15);
        let _ = map.insert('▬', 0x16);
        let _ = map.insert('↨', 0x17);
        let _ = map.insert('↑', 0x18);
        let _ = map.insert('↓', 0x19);
        let _ = map.insert('→', 0x1a);
        let _ = map.insert('←', 0x1b);
        let _ = map.insert('∟', 0x1c);
        let _ = map.insert('↔', 0x1d);
        let _ = map.insert('▲', 0x1e);
        let _ = map.insert('▼', 0x1f);
        let _ = map.insert('⌂', 0x7f);
        let _ = map.insert('Ç', 0x80);
        let _ = map.insert('ü', 0x81);
        let _ = map.insert('é', 0x82);
        let _ = map.insert('â', 0x83);
        let _ = map.insert('ä', 0x84);
        let _ = map.insert('à', 0x85);
        let _ = map.insert('å', 0x86);
        let _ = map.insert('ç', 0x87);
        let _ = map.insert('ê', 0x88);
        let _ = map.insert('ë', 0x89);
        let _ = map.insert('è', 0x8a);
        let _ = map.insert('ï', 0x8b);
        let _ = map.insert('î', 0x8c);
        let _ = map.insert('ì', 0x8d);
        let _ = map.insert('Ä', 0x8e);
        let _ = map.insert('Å', 0x8f);
        let _ = map.insert('É', 0x90);
        let _ = map.insert('æ', 0x91);
        let _ = map.insert('Æ', 0x92);
        let _ = map.insert('ô', 0x93);
        let _ = map.insert('ö', 0x94);
        let _ = map.insert('ò', 0x95);
        let _ = map.insert('û', 0x96);
        let _ = map.insert('ù', 0x97);
        let _ = map.insert('ÿ', 0x98);
        let _ = map.insert('Ö', 0x99);
        let _ = map.insert('Ü', 0x9a);
        let _ = map.insert('¢', 0x9b);
        let _ = map.insert('£', 0x9c);
        let _ = map.insert('¥', 0x9d);
        let _ = map.insert('₧', 0x9e);
        let _ = map.insert('ƒ', 0x9f);
        let _ = map.insert('á', 0xa0);
        let _ = map.insert('í', 0xa1);
        let _ = map.insert('ó', 0xa2);
        let _ = map.insert('ú', 0xa3);
        let _ = map.insert('ñ', 0xa4);
        let _ = map.insert('Ñ', 0xa5);
        let _ = map.insert('ª', 0xa6);
        let _ = map.insert('º', 0xa7);
        let _ = map.insert('¿', 0xa8);
        let _ = map.insert('⌐', 0xa9);
        let _ = map.insert('¬', 0xaa);
        let _ = map.insert('½', 0xab);
        let _ = map.insert('¼', 0xac);
        let _ = map.insert('¡', 0xad);
        let _ = map.insert('«', 0xae);
        let _ = map.insert('»', 0xaf);
        let _ = map.insert('░', 0xb0);
        let _ = map.insert('▒', 0xb1);
        let _ = map.insert('▓', 0xb2);
        let _ = map.insert('│', 0xb3);
        let _ = map.insert('┤', 0xb4);
        let _ = map.insert('╡', 0xb5);
        let _ = map.insert('╢', 0xb6);
        let _ = map.insert('╖', 0xb7);
        let _ = map.insert('╕', 0xb8);
        let _ = map.insert('╣', 0xb9);
        let _ = map.insert('║', 0xba);
        let _ = map.insert('╗', 0xbb);
        let _ = map.insert('╝', 0xbc);
        let _ = map.insert('╜', 0xbd);
        let _ = map.insert('╛', 0xbe);
        let _ = map.insert('┐', 0xbf);
        let _ = map.insert('└', 0xc0);
        let _ = map.insert('┴', 0xc1);
        let _ = map.insert('┬', 0xc2);
        let _ = map.insert('├', 0xc3);
        let _ = map.insert('─', 0xc4);
        let _ = map.insert('┼', 0xc5);
        let _ = map.insert('╞', 0xc6);
        let _ = map.insert('╟', 0xc7);
        let _ = map.insert('╚', 0xc8);
        let _ = map.insert('╔', 0xc9);
        let _ = map.insert('╩', 0xca);
        let _ = map.insert('╦', 0xcb);
        let _ = map.insert('╠', 0xcc);
        let _ = map.insert('═', 0xcd);
        let _ = map.insert('╬', 0xce);
        let _ = map.insert('╧', 0xcf);
        let _ = map.insert('╨', 0xd0);
        let _ = map.insert('╤', 0xd1);
        let _ = map.insert('╥', 0xd2);
        let _ = map.insert('╙', 0xd3);
        let _ = map.insert('╘', 0xd4);
        let _ = map.insert('╒', 0xd5);
        let _ = map.insert('╓', 0xd6);
        let _ = map.insert('╫', 0xd7);
        let _ = map.insert('╪', 0xd8);
        let _ = map.insert('┘', 0xd9);
        let _ = map.insert('┌', 0xda);
        let _ = map.insert('█', 0xdb);
        let _ = map.insert('▄', 0xdc);
        let _ = map.insert('▌', 0xdd);
        let _ = map.insert('▐', 0xde);
        let _ = map.insert('▀', 0xdf);
        let _ = map.insert('α', 0xe0);
        let _ = map.insert('ß', 0xe1);
        let _ = map.insert('Γ', 0xe2);
        let _ = map.insert('π', 0xe3);
        let _ = map.insert('Σ', 0xe4);
        let _ = map.insert('σ', 0xe5);
        let _ = map.insert('µ', 0xe6);
        let _ = map.insert('τ', 0xe7);
        let _ = map.insert('Φ', 0xe8);
        let _ = map.insert('Θ', 0xe9);
        let _ = map.insert('Ω', 0xea);
        let _ = map.insert('δ', 0xeb);
        let _ = map.insert('∞', 0xec);
        let _ = map.insert('φ', 0xed);
        let _ = map.insert('ε', 0xee);
        let _ = map.insert('∩', 0xef);
        let _ = map.insert('≡', 0xf0);
        let _ = map.insert('±', 0xf1);
        let _ = map.insert('≥', 0xf2);
        let _ = map.insert('≤', 0xf3);
        let _ = map.insert('⌠', 0xf4);
        let _ = map.insert('⌡', 0xf5);
        let _ = map.insert('÷', 0xf6);
        let _ = map.insert('≈', 0xf7);
        let _ = map.insert('°', 0xf8);
        let _ = map.insert('∙', 0xf9);
        let _ = map.insert('·', 0xfa);
        let _ = map.insert('√', 0xfb);
        let _ = map.insert('ⁿ', 0xfc);
        let _ = map.insert('²', 0xfd);

        let _ = map.insert('À', 0x41);
        let _ = map.insert('Â', 0x41);
        let _ = map.insert('È', 0x45);
        let _ = map.insert('Ê', 0x45);
        let _ = map.insert('Ë', 0x45);
        let _ = map.insert('Î', 0x49);
        let _ = map.insert('Ï', 0x49);
        let _ = map.insert('Ô', 0x4f);
        let _ = map.insert('Ù', 0x55);
        let _ = map.insert('Û', 0x55);
        let _ = map.insert('Ÿ', 0x59);
        map
    };
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
                // ■: default character.
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
