use crate::wally::img::Color;

use super::ColorPalette;

const SOL_COLOR_COUNT: usize = 11;

#[derive(Clone, Copy, Debug)]
pub(crate) struct SolarizedFlavor {
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

impl ColorPalette for SolarizedFlavor {
    fn len(&self) -> usize {
        SOL_COLOR_COUNT
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

impl SolarizedFlavor {
    pub fn dark() -> SolarizedFlavor {
        SolarizedFlavor {
            foreground: Color::from_u8(0x83, 0x94, 0x96), // #839496
            background: Color::from_u8(0x00, 0x2b, 0x36), // #002b36
            accent: Color::from_u8(0x07, 0x36, 0x42),     // #073642
            black: Color::from_u8(0x07, 0x36, 0x42),      // #073642
            red: Color::from_u8(0xdc, 0x32, 0x2f),        // #dc322f
            green: Color::from_u8(0x85, 0x99, 0x00),      // #859900
            yellow: Color::from_u8(0xb5, 0x89, 0x00),     // #b58900
            blue: Color::from_u8(0x26, 0x8b, 0xd2),       // #268bd2
            magenta: Color::from_u8(0xd3, 0x36, 0x82),    // #d33682
            cyan: Color::from_u8(0x2a, 0xa1, 0x98),       // #2aa198
            white: Color::from_u8(0xee, 0xe8, 0xd5),      // #eee8d5
        }
    }

    pub fn light() -> SolarizedFlavor {
        SolarizedFlavor {
            foreground: Color::from_u8(0x65, 0x7b, 0x83), // #657b83
            background: Color::from_u8(0xfd, 0xf6, 0xe3), // #fdf6e3
            accent: Color::from_u8(0x58, 0x6e, 0x75),     // #586e75
            black: Color::from_u8(0x07, 0x36, 0x42),      // #073642
            red: Color::from_u8(0xdc, 0x32, 0x2f),        // #dc322f
            green: Color::from_u8(0x85, 0x99, 0x00),      // #859900
            yellow: Color::from_u8(0xb5, 0x89, 0x00),     // #b58900
            blue: Color::from_u8(0x26, 0x8b, 0xd2),       // #268bd2
            magenta: Color::from_u8(0xd3, 0x36, 0x82),    // #d33682
            cyan: Color::from_u8(0x2a, 0xa1, 0x98),       // #2aa198
            white: Color::from_u8(0xee, 0xe8, 0xd5),      // #eee8d5
        }
    }
}
