use core::fmt;
use core::fmt::Write;

use volatile::Volatile;
use spin::Mutex;
use lazy_static::lazy_static;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;


pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // ASCII characters or new line
                0x20...0x7e | b'\n' => self.write_byte(byte),
                // non-printable chars
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            _ => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                self.buffer.chars[row][self.column_position].write(ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code
                });
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let char_cell = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(char_cell);
            }
        }
        self.clear_line(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_line(&mut self, row: usize) {
        let blank = ScreenChar {
                ascii_character: b' ',
                color_code: self.color_code
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}


#[repr(transparent)]
struct Buffer {
    // either implement volatile writes for ScreenChar or use Volatile
    // crate instead
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]  // represent as C, so fields are laid out in order
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(fg: Color, bg: Color) -> ColorCode {
        ColorCode((bg as u8) << 4 | (fg as u8))
    }
}


#[allow(dead_code)]  // supress warning for unused variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]  // store each variant as u8
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

lazy_static! {  // computes static object at runtime
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {  // mutex (spinlock), make object safely mutable
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[macro_export]  // to make macro available at crate root
macro_rules! println {
    () => ($crate::print!("\n"));  // use $crate to use this macro without importing `print` too
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

pub fn _print(args: fmt::Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
}
