use crate::wally::img::Color;

use super::Palette;

impl Palette {
    pub(crate) fn gruv_dark() -> Self {
        Palette {
            foreground: Color::from_u8(0xeb, 0xdb, 0xb2), // #ebdbb2
            accent: Color::from_u8(0x32, 0x30, 0x2f),     // #32302f
            base: Color::from_u8(0x28, 0x28, 0x28),       // #282828
            black: Color::from_u8(0x1d, 0x20, 0x21),      // #1d2021
            red: Color::from_u8(0xcc, 0x24, 0x1d),        // #cc241d
            yellow: Color::from_u8(0xd7, 0x99, 0x21),     // #d79921
            green: Color::from_u8(0x98, 0x97, 0x1a),      // #98971a
            cyan: Color::from_u8(0x68, 0x9d, 0x6a),       // #689d6a
            blue: Color::from_u8(0x45, 0x85, 0x88),       // #458588
            magenta: Color::from_u8(0xb1, 0x62, 0x86),    // #b16286
            white: Color::from_u8(0xa8, 0x99, 0x84),      // #a89984
        }
    }

    pub(crate) fn gruv_light() -> Self {
        Palette {
            foreground: Color::from_u8(0x3c, 0x38, 0x36), // #3c3836
            accent: Color::from_u8(0xfb, 0xf1, 0xc7),     // #fbf1c7
            base: Color::from_u8(0xf2, 0xe5, 0xbc),       // #f2e5bc
            black: Color::from_u8(0x92, 0x83, 0x74),      // #928374
            red: Color::from_u8(0xcc, 0x24, 0x1d),        // #cc241d
            yellow: Color::from_u8(0xd7, 0x99, 0x21),     // #d79921
            green: Color::from_u8(0x98, 0x97, 0x1a),      // #98971a
            cyan: Color::from_u8(0x68, 0x9d, 0x6a),       // #689d6a
            blue: Color::from_u8(0x45, 0x85, 0x88),       // #458588
            magenta: Color::from_u8(0xb1, 0x62, 0x86),    // #b16286
            white: Color::from_u8(0x7c, 0x6f, 0x64),      // #7c6f64
        }
    }
}
