use crate::wally::img::Color;

use super::ColorPalette;

pub(crate) const ROSE_COLOR_COUNT: usize = 15;

#[derive(Clone, Copy, Debug)]
pub(crate) struct RosePineFlavor {
    pub(crate) base: Color,
    pub(crate) surface: Color,
    pub(crate) overlay: Color,
    pub(crate) muted: Color,
    pub(crate) subtle: Color,
    pub(crate) text: Color,
    pub(crate) love: Color,
    pub(crate) gold: Color,
    pub(crate) rose: Color,
    pub(crate) pine: Color,
    pub(crate) foam: Color,
    pub(crate) iris: Color,
    pub(crate) h_low: Color,
    pub(crate) h_med: Color,
    pub(crate) h_high: Color,
}

impl ColorPalette for RosePineFlavor {
    fn len(&self) -> usize {
        ROSE_COLOR_COUNT
    }

    fn background(&self) -> Color {
        self.base
    }

    fn get_color(&self, idx: usize) -> Color {
        match idx {
            0 => self.base,
            1 => self.surface,
            2 => self.overlay,
            3 => self.muted,
            4 => self.subtle,
            5 => self.text,
            6 => self.love,
            7 => self.gold,
            8 => self.rose,
            9 => self.pine,
            10 => self.foam,
            11 => self.iris,
            12 => self.h_low,
            13 => self.h_med,
            14 => self.h_high,
            _ => unreachable!(),
        }
    }
}

impl RosePineFlavor {
    pub(crate) fn default() -> RosePineFlavor {
        RosePineFlavor {
            base: Color::from_u8(0x19, 0x17, 0x24),    // #191724
            surface: Color::from_u8(0x1f, 0x1d, 0x2e), // #1f1d2e
            overlay: Color::from_u8(0x26, 0x23, 0x3a), // #26233a
            muted: Color::from_u8(0x6e, 0x6a, 0x86),   // #6e6a86
            subtle: Color::from_u8(0x90, 0x8c, 0xaa),  // #908caa
            text: Color::from_u8(0xe0, 0xde, 0xf4),    // #e0def4
            love: Color::from_u8(0xeb, 0x6f, 0x92),    // #eb6f92
            gold: Color::from_u8(0xf6, 0xc1, 0x77),    // #f6c177
            rose: Color::from_u8(0xeb, 0xbc, 0xba),    // #ebbcba
            pine: Color::from_u8(0x31, 0x74, 0x8f),    // #31748f
            foam: Color::from_u8(0x9c, 0xcf, 0xd8),    // #9ccfd8
            iris: Color::from_u8(0xc4, 0xa7, 0xe7),    // #c4a7e7
            h_low: Color::from_u8(0x21, 0x20, 0x2e),   // #21202e
            h_med: Color::from_u8(0x40, 0x3d, 0x52),   // #403d52
            h_high: Color::from_u8(0x52, 0x4f, 0x67),  // #524f67
        }
    }

    pub(crate) fn dawn() -> RosePineFlavor {
        RosePineFlavor {
            base: Color::from_u8(0xfa, 0xf4, 0xed),    // #faf4ed
            surface: Color::from_u8(0xff, 0xfa, 0xf3), // #fffaf3
            overlay: Color::from_u8(0xf2, 0xe9, 0xe1), // #f2e9e1
            muted: Color::from_u8(0x98, 0x93, 0xa5),   // #9893a5
            subtle: Color::from_u8(0x79, 0x75, 0x93),  // #797593
            text: Color::from_u8(0x57, 0x52, 0x79),    // #575279
            love: Color::from_u8(0xb4, 0x63, 0x7a),    // #b4637a
            gold: Color::from_u8(0xea, 0x9d, 0x34),    // #ea9d34
            rose: Color::from_u8(0xd7, 0x82, 0x7e),    // #d7827e
            pine: Color::from_u8(0x28, 0x69, 0x83),    // #286983
            foam: Color::from_u8(0x56, 0x94, 0x9f),    // #56949f
            iris: Color::from_u8(0x90, 0x7a, 0xa9),    // #907aa9
            h_low: Color::from_u8(0xf4, 0xed, 0xe8),   // #f4ede8
            h_med: Color::from_u8(0xdf, 0xda, 0xd9),   // #dfdad9
            h_high: Color::from_u8(0xce, 0xca, 0xcd),  // #cecacd
        }
    }

    pub(crate) fn moon() -> RosePineFlavor {
        RosePineFlavor {
            base: Color::from_u8(0x23, 0x21, 0x36),    // #232136
            surface: Color::from_u8(0x2a, 0x27, 0x3f), // #2a273f
            overlay: Color::from_u8(0x39, 0x35, 0x52), // #393552
            muted: Color::from_u8(0x6e, 0x6a, 0x86),   // #6e6a86
            subtle: Color::from_u8(0x90, 0x8c, 0xaa),  // #908caa
            text: Color::from_u8(0xe0, 0xde, 0xf4),    // #e0def4
            love: Color::from_u8(0xeb, 0x6f, 0x92),    // #eb6f92
            gold: Color::from_u8(0xf6, 0xc1, 0x77),    // #f6c177
            rose: Color::from_u8(0xea, 0x9a, 0x97),    // #ea9a97
            pine: Color::from_u8(0x3e, 0x8f, 0xb0),    // #3e8fb0
            foam: Color::from_u8(0x9c, 0xcf, 0xd8),    // #9ccfd8
            iris: Color::from_u8(0xc4, 0xa7, 0xe7),    // #c4a7e7
            h_low: Color::from_u8(0x2a, 0x28, 0x3e),   // #2a283e
            h_med: Color::from_u8(0x44, 0x41, 0x5a),   // #44415a
            h_high: Color::from_u8(0x56, 0x52, 0x6e),  // #56526e
        }
    }
}
