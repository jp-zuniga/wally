use crate::wally::img::Color;

use super::Palette;

impl Palette {
    pub(crate) fn sol_dark() -> Self {
        Palette {
            foreground: Color::from_u8(0x83, 0x94, 0x96), // #839496
            accent: Color::from_u8(0x07, 0x36, 0x42),     // #073642
            base: Color::from_u8(0x00, 0x2b, 0x36),       // #002b36
            black: Color::from_u8(0x07, 0x36, 0x42),      // #073642
            red: Color::from_u8(0xdc, 0x32, 0x2f),        // #dc322f
            yellow: Color::from_u8(0xb5, 0x89, 0x00),     // #b58900
            green: Color::from_u8(0x85, 0x99, 0x00),      // #859900
            cyan: Color::from_u8(0x2a, 0xa1, 0x98),       // #2aa198
            blue: Color::from_u8(0x26, 0x8b, 0xd2),       // #268bd2
            magenta: Color::from_u8(0xd3, 0x36, 0x82),    // #d33682
            white: Color::from_u8(0xee, 0xe8, 0xd5),      // #eee8d5
        }
    }

    pub(crate) fn sol_light() -> Self {
        Palette {
            foreground: Color::from_u8(0x65, 0x7b, 0x83), // #657b83
            accent: Color::from_u8(0xee, 0xe8, 0xd5),     // #eee8d5
            base: Color::from_u8(0xfd, 0xf6, 0xe3),       // #fdf6e3
            black: Color::from_u8(0x07, 0x36, 0x42),      // #073642
            red: Color::from_u8(0xdc, 0x32, 0x2f),        // #dc322f
            yellow: Color::from_u8(0xb5, 0x89, 0x00),     // #b58900
            green: Color::from_u8(0x85, 0x99, 0x00),      // #859900
            cyan: Color::from_u8(0x2a, 0xa1, 0x98),       // #2aa198
            blue: Color::from_u8(0x26, 0x8b, 0xd2),       // #268bd2
            magenta: Color::from_u8(0xd3, 0x36, 0x82),    // #d33682
            white: Color::from_u8(0x58, 0x6e, 0x75),      // #586e75
        }
    }
}
