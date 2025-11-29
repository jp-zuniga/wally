use crate::wally::img::Color;

use super::ColorPalette;

const GRUV_COLOR_COUNT: usize = 9;

#[derive(Clone, Copy, Debug)]
pub(crate) struct GruvboxFlavor {
    background: Color,
    foreground: Color,
    white: Color,
    red: Color,
    yellow: Color,
    green: Color,
    cyan: Color,
    blue: Color,
    magenta: Color,
}

impl ColorPalette for GruvboxFlavor {
    fn len(&self) -> usize {
        GRUV_COLOR_COUNT
    }

    fn background(&self) -> Color {
        self.background
    }

    fn get_color(&self, idx: usize) -> Color {
        match idx {
            0 => self.background,
            1 => self.foreground,
            2 => self.white,
            3 => self.red,
            4 => self.yellow,
            5 => self.green,
            6 => self.cyan,
            7 => self.blue,
            8 => self.magenta,
            _ => unreachable!(),
        }
    }
}

impl GruvboxFlavor {
    pub(crate) fn dark() -> Self {
        GruvboxFlavor {
            background: Color::from_u8(0x28, 0x28, 0x28), // #282828
            foreground: Color::from_u8(0xeb, 0xdb, 0xb2), // #ebdbb2
            white: Color::from_u8(0xa8, 0x99, 0x84),      // #a89984
            red: Color::from_u8(0xcc, 0x24, 0x1d),        // #cc241d
            yellow: Color::from_u8(0xd7, 0x99, 0x21),     // #d79921
            green: Color::from_u8(0x98, 0x97, 0x1a),      // #98971a
            cyan: Color::from_u8(0x68, 0x9d, 0x6a),       // #689d6a
            blue: Color::from_u8(0x45, 0x85, 0x88),       // #458588
            magenta: Color::from_u8(0xb1, 0x62, 0x86),    // #b16286
        }
    }

    pub(crate) fn light() -> Self {
        GruvboxFlavor {
            background: Color::from_u8(0xfb, 0xf1, 0xc7), // #fbf1c7
            foreground: Color::from_u8(0x3c, 0x38, 0x36), // #3c3836
            white: Color::from_u8(0x7c, 0x6f, 0x64),      // #7c6f64
            red: Color::from_u8(0xcc, 0x24, 0x1d),        // #cc241d
            yellow: Color::from_u8(0xd7, 0x99, 0x21),     // #d79921
            green: Color::from_u8(0x98, 0x97, 0x1a),      // #98971a
            cyan: Color::from_u8(0x68, 0x9d, 0x6a),       // #689d6a
            blue: Color::from_u8(0x45, 0x85, 0x88),       // #458588
            magenta: Color::from_u8(0xb1, 0x62, 0x86),    // #b16286
        }
    }
}
