use crate::wally::img::Color;

use super::Palette;

impl Palette {
    pub(crate) fn dracula() -> Self {
        Palette {
            foreground: Color::from_u8(0xf8, 0xf8, 0xf2), // #f8f8f2
            accent: Color::from_u8(0x44, 0x47, 0x5a),     // #44475a
            base: Color::from_u8(0x28, 0x2a, 0x36),       // #282a36
            black: Color::from_u8(0x21, 0x22, 0x2c),      // #21222c
            red: Color::from_u8(0xff, 0x55, 0x55),        // #ff5555
            yellow: Color::from_u8(0xf1, 0xfa, 0x8c),     // #f1fa8c
            green: Color::from_u8(0x50, 0xfa, 0x7b),      // #50fa7b
            cyan: Color::from_u8(0x8b, 0xe9, 0xfd),       // #8be9fd
            blue: Color::from_u8(0xbd, 0x93, 0xf9),       // #bd93f9
            magenta: Color::from_u8(0xff, 0x79, 0xc6),    // #ff79c6
            white: Color::from_u8(0xf8, 0xf8, 0xf2),      // #f8f8f2
        }
    }

    pub(crate) fn alucard() -> Self {
        Palette {
            foreground: Color::from_u8(0x6c, 0x66, 0x4b), // #6c664b
            accent: Color::from_u8(0xff, 0xfb, 0xeb),     // #fffbeb
            base: Color::from_u8(0xcf, 0xcf, 0xde),       // #cfcfde
            black: Color::from_u8(0xf8, 0xf8, 0xf2),      // #f8f8f2
            red: Color::from_u8(0xcb, 0x3a, 0x2a),        // #cb3a2a
            yellow: Color::from_u8(0x84, 0x6e, 0x15),     // #846e15
            green: Color::from_u8(0x14, 0x71, 0x0a),      // #14710a
            cyan: Color::from_u8(0x03, 0x6a, 0x96),       // #036a96
            blue: Color::from_u8(0x64, 0x4a, 0xc9),       // #644ac9
            magenta: Color::from_u8(0xa3, 0x14, 0x4d),    // #a3144d
            white: Color::from_u8(0x6c, 0x66, 0x4b),      // #6c664b
        }
    }
}
