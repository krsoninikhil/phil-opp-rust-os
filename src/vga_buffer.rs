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
                self.buffer.chars[row][self.column_position] = ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code
                };
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        // TODO
    }
}


#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
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


pub fn write_vga(text: &str) {
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };
    writer.write_string(text);
}
