use crate::wally::img::Color;

use super::ColorPalette;

pub(crate) const DRACULA_COLOR_COUNT: usize = 12;

#[derive(Clone, Copy, Debug)]
pub(crate) struct DraculaFlavor {
    background: Color,
    foreground: Color,
    line: Color,
    selection: Color,
    comment: Color,
    red: Color,
    orange: Color,
    yellow: Color,
    green: Color,
    cyan: Color,
    purple: Color,
    pink: Color,
}

impl ColorPalette for DraculaFlavor {
    fn len(&self) -> usize {
        DRACULA_COLOR_COUNT
    }

    fn background(&self) -> Color {
        self.background
    }

    fn get_color(&self, idx: usize) -> Color {
        match idx {
            0 => self.background,
            1 => self.foreground,
            2 => self.line,
            3 => self.selection,
            4 => self.comment,
            5 => self.red,
            6 => self.orange,
            7 => self.yellow,
            8 => self.green,
            9 => self.cyan,
            10 => self.purple,
            11 => self.pink,
            _ => unreachable!(),
        }
    }
}

impl DraculaFlavor {
    pub fn default() -> DraculaFlavor {
        DraculaFlavor {
            background: Color::from_u8(0x28, 0x2a, 0x36), // #282a36
            foreground: Color::from_u8(0xf8, 0xf8, 0xf2), // #f8f8f2
            line: Color::from_u8(0x62, 0x72, 0xa4),       // #6272a4
            selection: Color::from_u8(0x44, 0x47, 0x5a),  // #44475a
            comment: Color::from_u8(0x62, 0x72, 0xa4),    // #6272a4
            red: Color::from_u8(0xff, 0x55, 0x55),        // #ff5555
            orange: Color::from_u8(0xff, 0xb8, 0x6c),     // #ffb86c
            yellow: Color::from_u8(0xf1, 0xfa, 0x8c),     // #f1fa8c
            green: Color::from_u8(0x50, 0xfa, 0x7b),      // #50fa7b
            cyan: Color::from_u8(0x8b, 0xe9, 0xfd),       // #8be9fd
            purple: Color::from_u8(0xbd, 0x93, 0xf9),     // #bd93f9
            pink: Color::from_u8(0xff, 0x79, 0xc6),       // #ff79c6
        }
    }

    pub fn alucard() -> DraculaFlavor {
        DraculaFlavor {
            background: Color::from_u8(0xff, 0xfb, 0xeb), // #fffbeb
            foreground: Color::from_u8(0x6c, 0x66, 0x4b), // #6c664b
            line: Color::from_u8(0xcf, 0xcf, 0xde),       // #cfcfde
            selection: Color::from_u8(0x1f, 0x1f, 0x1f),  // #1f1f1f
            comment: Color::from_u8(0x6c, 0x66, 0x4b),    // #6c664b
            red: Color::from_u8(0xcb, 0x3a, 0x2a),        // #cb3a2a
            orange: Color::from_u8(0xa3, 0x4d, 0x14),     // #a34d14
            yellow: Color::from_u8(0x84, 0x6e, 0x15),     // #846e15
            green: Color::from_u8(0x14, 0x71, 0x0a),      // #14710a
            cyan: Color::from_u8(0x03, 0x6a, 0x96),       // #036a96
            purple: Color::from_u8(0x64, 0x4a, 0xc9),     // #644ac9
            pink: Color::from_u8(0xa3, 0x14, 0x4d),       // #a3144d
        }
    }
}
