//vga_buffer.rs

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
    Light Blue  = 9,
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
        ColorCode((background as u*) << 4 | (foreground as u8))
    }
}

//Copy Semantics
#[derive(Debug, Clone, Copy, PartialEq, Eq]
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
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
        column_position: usize, 
        color_code: ColorCode,
        buffer: &'static mut Buffer,
}

// Funciton to write a single ASCII byte
