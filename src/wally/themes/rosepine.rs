use crate::wally::img::Color;

use super::Palette;

impl Palette {
    pub(crate) fn rose_dawn() -> Self {
        Palette {
            foreground: Color::from_u8(0x57, 0x52, 0x79), // #575279
            accent: Color::from_u8(0xf3, 0xee, 0xea),     // #f3eeea
            base: Color::from_u8(0xfa, 0xf4, 0xed),       // #faf4ed
            black: Color::from_u8(0xf2, 0xe9, 0xe1),      // #f2e9e1
            red: Color::from_u8(0xb4, 0x63, 0x7a),        // #b4637a
            green: Color::from_u8(0x28, 0x69, 0x83),      // #286983
            yellow: Color::from_u8(0xea, 0x9d, 0x34),     // #ea9d34
            blue: Color::from_u8(0x56, 0x94, 0x9f),       // #56949f
            magenta: Color::from_u8(0x90, 0x7a, 0xa9),    // #907aa9
            cyan: Color::from_u8(0xd7, 0x82, 0x7e),       // #d7827e
            white: Color::from_u8(0x57, 0x52, 0x79),      // #575279
        }
    }

    pub(crate) fn rose_moon() -> Self {
        Palette {
            foreground: Color::from_u8(0xe0, 0xde, 0xf4), // #e0def4
            accent: Color::from_u8(0x2a, 0x27, 0x3f),     // #2a273f
            base: Color::from_u8(0x23, 0x21, 0x36),       // #232136
            black: Color::from_u8(0x39, 0x35, 0x52),      // #393552
            red: Color::from_u8(0xeb, 0x6f, 0x92),        // #eb6f92
            green: Color::from_u8(0x3e, 0x8f, 0xb0),      // #3e8fb0
            yellow: Color::from_u8(0xf6, 0xc1, 0x77),     // #f6c177
            blue: Color::from_u8(0x9c, 0xcf, 0xd8),       // #9ccfd8
            magenta: Color::from_u8(0xc4, 0xa7, 0xe7),    // #c4a7e7
            cyan: Color::from_u8(0xea, 0x9a, 0x97),       // #ea9a97
            white: Color::from_u8(0xe0, 0xde, 0xf4),      // #e0def4
        }
    }

    pub(crate) fn rose_pine() -> Self {
        Palette {
            foreground: Color::from_u8(0xe0, 0xde, 0xf4), // #e0def4
            accent: Color::from_u8(0x1f, 0x1d, 0x2e),     // #1f1d2e
            base: Color::from_u8(0x19, 0x17, 0x24),       // #191724
            black: Color::from_u8(0x26, 0x23, 0x3a),      // #26233a
            red: Color::from_u8(0xeb, 0x6f, 0x92),        // #eb6f92
            green: Color::from_u8(0x31, 0x74, 0x8f),      // #31748f
            yellow: Color::from_u8(0xf6, 0xc1, 0x77),     // #f6c177
            blue: Color::from_u8(0x9c, 0xcf, 0xd8),       // #9ccfd8
            magenta: Color::from_u8(0xc4, 0xa7, 0xe7),    // #c4a7e7
            cyan: Color::from_u8(0xeb, 0xbc, 0xba),       // #ebbcba
            white: Color::from_u8(0xe0, 0xde, 0xf4),      // #e0def4
        }
    }
}
