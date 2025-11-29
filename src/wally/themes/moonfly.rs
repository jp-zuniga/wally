use crate::wally::img::Color;

use super::Palette;

impl Palette {
    pub(crate) fn moonfly() -> Self {
        Palette {
            foreground: Color::from_u8(0xbd, 0xbd, 0xbd), // #bdbdbd
            accent: Color::from_u8(0xb2, 0xce, 0xee),     // #b2ceee
            base: Color::from_u8(0x08, 0x08, 0x08),       // #080808
            black: Color::from_u8(0x32, 0x34, 0x37),      // #323437
            red: Color::from_u8(0xff, 0x54, 0x54),        // #ff5454
            green: Color::from_u8(0x8c, 0xc8, 0x5f),      // #8cc85f
            yellow: Color::from_u8(0xe3, 0xc7, 0x8a),     // #e3c78a
            blue: Color::from_u8(0x80, 0xa0, 0xff),       // #80a0ff
            magenta: Color::from_u8(0xcf, 0x87, 0xe8),    // #cf87e8
            cyan: Color::from_u8(0x79, 0xda, 0xc8),       // #79dac8
            white: Color::from_u8(0xc6, 0xc6, 0xc6),      // #c6c6c6
        }
    }
}
