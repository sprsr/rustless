//vga_buffer.rs
// Writes to the VGA are volatile
use volatile::Volatile;
// To Format macros to print different types
use core::fmt;
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
// allow(dead_code) prevents warnings for unused enums.
// Deriving Debug, Clone, Copy, ... we enable copy semantics.
// Enum to specify color.  [repr(u8)] defines enums as u8.
pub enum Color {
    Black       = 0,
    Blue        = 1,
    Green       = 2,
    Cyan        = 3,
    Red         = 4,
    Magenta     = 5,
    Brown       = 6,
    LightGray   = 7,
    DarkGray    = 8,
    LightBlue  = 9,
    LightGreen  = 10,
    LightCyan   = 11,
    LightRed    = 12,
    Pink        = 13,
    Yellow      = 14,
    White       = 15,
}

// ColorCode contains full color byte, contains foreground and bg color. 
// // copy semantics and transparent to ensure exact data layout of u8.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

//Copy Semantics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Ensures struct is laid out exactly as in C
#[repr(C)]
struct ScreenChar {
        ascii_character: u8,
        color_code: ColorCode,
}
// Buffer Height and Width
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
// Volatile writes 
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
        column_position: usize, 
        color_code: ColorCode,
        buffer: &'static mut Buffer,
}

// Funciton to write a single ASCII byte
impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = BUFFER_HEIGHT -1;
                let col = self.column_position;
                let color_code = self.color_code;
                // Using the write method to guarantee that the compiler never optimizes away
                // write 
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position +=1;
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

    fn clear_row(&mut self, row: usize) {/* TODO */}

    // Function to write a full string
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }
}

// Function to print something
pub fn print_string() {
    use core::fmt::Write;
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_string("Welcome to The Rust Kernel");
    write!(writer, "Testing number implementation : {}", 1.0/3.0);
}

//Formatting macros so we can print different types
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
