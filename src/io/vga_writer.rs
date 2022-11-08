use core::fmt::Write;
use volatile::Volatile;

const VGA_BUFFER_POINTER: *mut u8 = 0xb8000 as *mut u8;

const LINES: usize = 25;
const COLUMNS: usize = 80;
const EMPTY_ROW: &'static [ScreenChar; COLUMNS] = &[ScreenChar::default_space(); COLUMNS];

const DEFAULT_FOREGROUND_COLOR: Color = Color::White;
const DEFAULT_BACKGROUND_COLOR: Color = Color::Black;

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

pub struct VgaWriter {
    current_line: usize,
    current_col: usize,
    inner: &'static mut VgaInner,
}

impl VgaWriter {
    pub fn new() -> VgaWriter {
        let inner = unsafe { &mut *(VGA_BUFFER_POINTER as *mut VgaInner) };
        VgaWriter {
            current_line: 0,
            current_col: 0,
            inner,
        }
    }

    pub fn push_bytes(&mut self, bytes: &[u8], foreground: Color, background: Color) {
        for byte in bytes {
            self.push_byte(*byte, foreground, background);
        }
    }

    pub fn push_byte(&mut self, byte: u8, foreground: Color, background: Color) {
        let byte = match byte {
            0x20..=0x7e => byte,
            // printable ASCII byte or newline
            b'\n' => return self.new_line(),
            // not part of printable ASCII range
            _ => 0xfe,
        };

        if self.current_col == COLUMNS {
            self.new_line();
        }

        self.inner.buf[self.current_line][self.current_col] = ScreenChar {
            ascii_character: byte,
            color_code: ColorCode::new(foreground, background),
        };

        self.current_col += 1;
    }

    fn new_line(&mut self) {
        if self.current_line == LINES - 1 {
            self.shift_lines_up();
        } else {
            self.current_line += 1;
        }

        self.current_col = 0;
    }

    fn shift_lines_up(&mut self) {
        for i in 0..(LINES - 1) {
            let (dst, src) = self.inner.buf.split_at_mut(i + 1);
            // `Volatile` tells the compiler that the write has side effects and should not be optimized away.
            Volatile::new(dst[i].as_mut_slice()).copy_from_slice(src[0].as_slice());
        }

        self.inner.buf[LINES - 1].copy_from_slice(EMPTY_ROW);
    }
}

impl Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        Ok(self.push_bytes(
            s.as_bytes(),
            DEFAULT_FOREGROUND_COLOR,
            DEFAULT_BACKGROUND_COLOR,
        ))
    }
}

#[repr(transparent)]
struct VgaInner {
    buf: [[ScreenChar; COLUMNS]; LINES],
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

/// `repc(C)` guarantees that the struct’s fields are laid out exactly like in a C struct
/// and thus guarantees the correct field ordering.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

impl ScreenChar {
    const fn default_space() -> ScreenChar {
        ScreenChar {
            ascii_character: b' ',
            color_code: ColorCode::new(DEFAULT_FOREGROUND_COLOR, DEFAULT_BACKGROUND_COLOR),
        }
    }
}
