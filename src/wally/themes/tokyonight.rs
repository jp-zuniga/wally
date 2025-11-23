use crate::wally::img::Color;

use super::ColorPalette;

pub(crate) const TOKYO_COLOR_COUNT: usize = 11;

#[derive(Clone, Copy, Debug)]
pub(crate) struct TokyoNightFlavor {
    background: Color,
    foreground: Color,
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

impl ColorPalette for TokyoNightFlavor {
    fn len(&self) -> usize {
        TOKYO_COLOR_COUNT
    }

    fn background(&self) -> Color {
        self.background
    }

    fn get_color(&self, idx: usize) -> Color {
        match idx {
            0 => self.background,
            1 => self.foreground,
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

impl TokyoNightFlavor {
    pub fn default() -> TokyoNightFlavor {
        TokyoNightFlavor {
            background: Color::from_u8(0x1a, 0x1b, 0x26), // #1a1b26
            foreground: Color::from_u8(0xc0, 0xca, 0xf5), // #c0caf5
            accent: Color::from_u8(0x28, 0x34, 0x57),     // #283457
            black: Color::from_u8(0x15, 0x16, 0x1e),      // #15161e
            red: Color::from_u8(0xf7, 0x76, 0x8e),        // #f7768e
            green: Color::from_u8(0x9e, 0xce, 0x6a),      // #9ece6a
            yellow: Color::from_u8(0xe0, 0xaf, 0x68),     // #e0af68
            blue: Color::from_u8(0x7a, 0xa2, 0xf7),       // #7aa2f7
            magenta: Color::from_u8(0xbb, 0x9a, 0xf7),    // #bb9af7
            cyan: Color::from_u8(0x7d, 0xcf, 0xff),       // #7dcfff
            white: Color::from_u8(0xa9, 0xb1, 0xd6),      // #a9b1d6
        }
    }

    pub fn day() -> TokyoNightFlavor {
        TokyoNightFlavor {
            background: Color::from_u8(0xe1, 0xe2, 0xe7), // #e1e2e7
            foreground: Color::from_u8(0x37, 0x60, 0xbf), // #3760bf
            accent: Color::from_u8(0xb7, 0xc1, 0xe3),     // #b7c1e3
            black: Color::from_u8(0xb4, 0xb5, 0xb9),      // #b4b5b9
            red: Color::from_u8(0xf5, 0x2a, 0x65),        // #f52a65
            green: Color::from_u8(0x58, 0x75, 0x39),      // #587539
            yellow: Color::from_u8(0x8c, 0x6c, 0x3e),     // #8c6c3e
            blue: Color::from_u8(0x2e, 0x7d, 0xe9),       // #2e7de9
            magenta: Color::from_u8(0x98, 0x54, 0xf1),    // #9854f1
            cyan: Color::from_u8(0x00, 0x71, 0x97),       // #007197
            white: Color::from_u8(0x61, 0x72, 0xb0),      // #6172b0
        }
    }

    pub fn moon() -> TokyoNightFlavor {
        TokyoNightFlavor {
            background: Color::from_u8(0x22, 0x24, 0x36), // #222436
            foreground: Color::from_u8(0xc8, 0xd3, 0xf5), // #c8d3f5
            accent: Color::from_u8(0x2d, 0x3f, 0x76),     // #2d3f76
            black: Color::from_u8(0x1b, 0x1d, 0x2b),      // #1b1d2b
            red: Color::from_u8(0xff, 0x75, 0x7f),        // #ff757f
            green: Color::from_u8(0xc3, 0xe8, 0x8d),      // #c3e88d
            yellow: Color::from_u8(0xff, 0xc7, 0x77),     // #ffc777
            blue: Color::from_u8(0x82, 0xaa, 0xff),       // #82aaff
            magenta: Color::from_u8(0xc0, 0x99, 0xff),    // #c099ff
            cyan: Color::from_u8(0x86, 0xe1, 0xfc),       // #86e1fc
            white: Color::from_u8(0x82, 0x8b, 0xb8),      // #828bb8
        }
    }

    pub fn storm() -> TokyoNightFlavor {
        TokyoNightFlavor {
            background: Color::from_u8(0x24, 0x28, 0x3b), // #24283b
            foreground: Color::from_u8(0xc0, 0xca, 0xf5), // #c0caf5
            accent: Color::from_u8(0x2e, 0x3c, 0x64),     // #2e3c64
            black: Color::from_u8(0x1d, 0x20, 0x2f),      // #1d202f
            red: Color::from_u8(0xf7, 0x76, 0x8e),        // #f7768e
            green: Color::from_u8(0x9e, 0xce, 0x6a),      // #9ece6a
            yellow: Color::from_u8(0xe0, 0xaf, 0x68),     // #e0af68
            blue: Color::from_u8(0x7a, 0xa2, 0xf7),       // #7aa2f7
            magenta: Color::from_u8(0xbb, 0x9a, 0xf7),    // #bb9af7
            cyan: Color::from_u8(0x7d, 0xcf, 0xff),       // #7dcfff
            white: Color::from_u8(0xa9, 0xb1, 0xd6),      // #a9b1d6
        }
    }
}
