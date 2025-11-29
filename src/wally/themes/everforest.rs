use crate::wally::img::Color;

use super::Palette;

impl Palette {
    pub(crate) fn evf_dark() -> Self {
        Palette {
            foreground: Color::from_u8(0xd3, 0xc6, 0xaa), // #d3c6aa
            accent: Color::from_u8(0x41, 0x4b, 0x51),     // #414b51
            base: Color::from_u8(0x2d, 0x35, 0x3b),       // #2d353b
            black: Color::from_u8(0x34, 0x3f, 0x44),      // #343f44
            red: Color::from_u8(0xe6, 0x7e, 0x80),        // #e67e80
            yellow: Color::from_u8(0xa7, 0xc0, 0x80),     // #a7c080
            green: Color::from_u8(0xdb, 0xbc, 0x7f),      // #dbbc7f
            cyan: Color::from_u8(0x7f, 0xbb, 0xb3),       // #7fbbb3
            blue: Color::from_u8(0xd6, 0x99, 0xb6),       // #d699b6
            magenta: Color::from_u8(0x83, 0xc0, 0x92),    // #83c092
            white: Color::from_u8(0xd3, 0xc6, 0xaa),      // #d3c6aa
        }
    }

    pub(crate) fn evf_light() -> Self {
        Palette {
            foreground: Color::from_u8(0x5c, 0x6a, 0x72), // #5c6a72
            accent: Color::from_u8(0xfd, 0xf6, 0xe3),     // #fdf6e3
            base: Color::from_u8(0xef, 0xe9, 0xd5),       // #efe9d5
            black: Color::from_u8(0x5c, 0x6a, 0x72),      // #5c6a72
            red: Color::from_u8(0xf8, 0x55, 0x52),        // #f85552
            yellow: Color::from_u8(0x8d, 0xa1, 0x01),     // #8da101
            green: Color::from_u8(0xdf, 0xa0, 0x00),      // #dfa000
            cyan: Color::from_u8(0x3a, 0x94, 0xc5),       // #3a94c5
            blue: Color::from_u8(0xdf, 0x69, 0xba),       // #df69ba
            magenta: Color::from_u8(0x35, 0xa7, 0x7c),    // #35a77c
            white: Color::from_u8(0x93, 0x9f, 0x91),      // #939f91
        }
    }
}
