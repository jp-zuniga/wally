use crate::wally::img::Color;

use super::ColorPalette;

const NORD_COLOR_COUNT: usize = 11;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Nord {
    foreground: Color,
    background: Color,
    accent: Color,
    black: Color,
    red: Color,
    green: Color,
    yellow: Color,
    blue: Color,
    magenta: Color,
    cyan: Color,
    white: Color,
}

impl ColorPalette for Nord {
    fn len(&self) -> usize {
        NORD_COLOR_COUNT
    }

    fn background(&self) -> Color {
        self.background
    }

    fn get_color(&self, idx: usize) -> Color {
        match idx {
            0 => self.foreground,
            1 => self.background,
            2 => self.accent,
            3 => self.black,
            4 => self.red,
            5 => self.green,
            6 => self.yellow,
            7 => self.blue,
            8 => self.magenta,
            9 => self.cyan,
            10 => self.white,
            _ => unreachable!(),
        }
    }
}

impl Nord {
    pub(crate) fn new() -> Self {
        Nord {
            foreground: Color::from_u8(0xd8, 0xde, 0xe9), // #d8dee9
            background: Color::from_u8(0x2e, 0x34, 0x40), // #2e3440
            accent: Color::from_u8(0x3f, 0x47, 0x58),     // #3f4758
            black: Color::from_u8(0x3b, 0x42, 0x52),      // #3b4252
            red: Color::from_u8(0xbf, 0x61, 0x6a),        // #bf616a
            green: Color::from_u8(0xa3, 0xbe, 0x8c),      // #a3be8c
            yellow: Color::from_u8(0xeb, 0xcb, 0x8b),     // #ebcb8b
            blue: Color::from_u8(0x81, 0xa1, 0xc1),       // #81a1c1
            magenta: Color::from_u8(0xb4, 0x8e, 0xad),    // #b48ead
            cyan: Color::from_u8(0x88, 0xc0, 0xd0),       // #88c0d0
            white: Color::from_u8(0xe5, 0xe9, 0xf0),      // #e5e9f0
        }
    }
}
