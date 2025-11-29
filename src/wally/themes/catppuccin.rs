use crate::wally::img::Color;

use super::Palette;

impl Palette {
    pub(crate) fn cat_frappe() -> Self {
        Palette {
            foreground: Color::from_u8(0xc6, 0xd0, 0xf5), // #c6d0f5
            accent: Color::from_u8(0x44, 0x49, 0x5d),     // #44495d
            base: Color::from_u8(0x30, 0x34, 0x46),       // #303446
            black: Color::from_u8(0x51, 0x57, 0x6d),      // #51576d
            red: Color::from_u8(0xe7, 0x82, 0x84),        // #e78284
            green: Color::from_u8(0xa6, 0xd1, 0x89),      // #a6d189
            yellow: Color::from_u8(0xe5, 0xc8, 0x90),     // #e5c890
            blue: Color::from_u8(0x8c, 0xaa, 0xee),       // #8caaee
            magenta: Color::from_u8(0xf4, 0xb8, 0xe4),    // #f4b8e4
            cyan: Color::from_u8(0x81, 0xc8, 0xbe),       // #81c8be
            white: Color::from_u8(0xa5, 0xad, 0xce),      // #a5adce
        }
    }

    pub(crate) fn cat_latte() -> Self {
        Palette {
            foreground: Color::from_u8(0x4c, 0x4f, 0x69), // #4c4f69
            accent: Color::from_u8(0xef, 0xf1, 0xf5),     // #eff1f5
            base: Color::from_u8(0xd8, 0xda, 0xe1),       // #d8dae1
            black: Color::from_u8(0x5c, 0x5f, 0x77),      // #5c5f77
            red: Color::from_u8(0xd2, 0x0f, 0x39),        // #d20f39
            green: Color::from_u8(0x40, 0xa0, 0x2b),      // #40a02b
            yellow: Color::from_u8(0xdf, 0x8e, 0x1d),     // #df8e1d
            blue: Color::from_u8(0x1e, 0x66, 0xf5),       // #1e66f5
            magenta: Color::from_u8(0xea, 0x76, 0xcb),    // #ea76cb
            cyan: Color::from_u8(0x17, 0x92, 0x99),       // #179299
            white: Color::from_u8(0xac, 0xb0, 0xbe),      // #acb0be
        }
    }

    pub(crate) fn cat_macchiato() -> Self {
        Palette {
            foreground: Color::from_u8(0xca, 0xd3, 0xf5), // #cad3f5
            accent: Color::from_u8(0x3a, 0x3e, 0x53),     // #3a3e53
            base: Color::from_u8(0x24, 0x27, 0x3a),       // #24273a
            black: Color::from_u8(0x49, 0x4d, 0x64),      // #494d64
            red: Color::from_u8(0xed, 0x87, 0x96),        // #ed8796
            green: Color::from_u8(0xa6, 0xda, 0x95),      // #a6da95
            yellow: Color::from_u8(0xee, 0xd4, 0x9f),     // #eed49f
            blue: Color::from_u8(0x8a, 0xad, 0xf4),       // #8aadf4
            magenta: Color::from_u8(0xf5, 0xbd, 0xe6),    // #f5bde6
            cyan: Color::from_u8(0x8b, 0xd5, 0xca),       // #8bd5ca
            white: Color::from_u8(0xa5, 0xad, 0xcb),      // #a5adcb
        }
    }

    pub(crate) fn cat_mocha() -> Self {
        Palette {
            foreground: Color::from_u8(0xcd, 0xd6, 0xf4), // #cdd6f4
            accent: Color::from_u8(0x35, 0x37, 0x48),     // #353748
            base: Color::from_u8(0x1e, 0x1e, 0x2e),       // #1e1e2e
            black: Color::from_u8(0x45, 0x47, 0x5a),      // #45475a
            red: Color::from_u8(0xf3, 0x8b, 0xa8),        // #f38ba8
            green: Color::from_u8(0xa6, 0xe3, 0xa1),      // #a6e3a1
            yellow: Color::from_u8(0xf9, 0xe2, 0xaf),     // #f9e2af
            blue: Color::from_u8(0x89, 0xb4, 0xfa),       // #89b4fa
            magenta: Color::from_u8(0xf5, 0xc2, 0xe7),    // #f5c2e7
            cyan: Color::from_u8(0x94, 0xe2, 0xd5),       // #94e2d5
            white: Color::from_u8(0xa6, 0xad, 0xc8),      // #a6adc8
        }
    }
}
