use crate::wally::img::Color;

use super::Palette;

impl Palette {
    pub(crate) fn nord() -> Self {
        Palette {
            foreground: Color::from_u8(0xd8, 0xde, 0xe9), // #d8dee9
            accent: Color::from_u8(0x3f, 0x47, 0x58),     // #3f4758
            base: Color::from_u8(0x2e, 0x34, 0x40),       // #2e3440
            black: Color::from_u8(0x3b, 0x42, 0x52),      // #3b4252
            red: Color::from_u8(0xbf, 0x61, 0x6a),        // #bf616a
            yellow: Color::from_u8(0xeb, 0xcb, 0x8b),     // #ebcb8b
            green: Color::from_u8(0xa3, 0xbe, 0x8c),      // #a3be8c
            cyan: Color::from_u8(0x88, 0xc0, 0xd0),       // #88c0d0
            blue: Color::from_u8(0x81, 0xa1, 0xc1),       // #81a1c1
            magenta: Color::from_u8(0xb4, 0x8e, 0xad),    // #b48ead
            white: Color::from_u8(0xe5, 0xe9, 0xf0),      // #e5e9f0
        }
    }
}
