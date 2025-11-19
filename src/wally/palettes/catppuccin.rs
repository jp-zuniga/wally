use clap::ValueEnum;

use crate::wally::img::Color;

use super::theme::ThemeFlavor;

pub(crate) const CAT_COLOR_COUNT: usize = 25;

#[derive(Clone, Copy, Debug, ValueEnum)]
#[clap(rename_all = "kebab_case")]
pub(crate) enum CatppuccinFlavorNames {
    Frappe,
    Latte,
    Macchiato,
    Mocha,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct CatppuccinFlavor {
    pub(crate) rosewater: Color,
    pub(crate) flamingo: Color,
    pub(crate) pink: Color,
    pub(crate) mauve: Color,
    pub(crate) red: Color,
    pub(crate) maroon: Color,
    pub(crate) peach: Color,
    pub(crate) yellow: Color,
    pub(crate) green: Color,
    pub(crate) teal: Color,
    pub(crate) sky: Color,
    pub(crate) sapphire: Color,
    pub(crate) blue: Color,
    pub(crate) lavender: Color,
    pub(crate) text: Color,
    pub(crate) subtext0: Color,
    pub(crate) subtext1: Color,
    pub(crate) overlay0: Color,
    pub(crate) overlay1: Color,
    pub(crate) overlay2: Color,
    pub(crate) surface0: Color,
    pub(crate) surface1: Color,
    pub(crate) surface2: Color,
    pub(crate) base: Color,
    pub(crate) mantle: Color,
    pub(crate) crust: Color,
}

impl ThemeFlavor for CatppuccinFlavor {
    fn len(&self) -> usize {
        CAT_COLOR_COUNT
    }

    fn background(&self) -> Color {
        self.base
    }

    fn get_color(&self, idx: usize) -> Color {
        match idx {
            0 => self.rosewater,
            1 => self.flamingo,
            2 => self.pink,
            3 => self.mauve,
            4 => self.red,
            5 => self.maroon,
            6 => self.peach,
            7 => self.yellow,
            8 => self.green,
            9 => self.teal,
            10 => self.sky,
            11 => self.sapphire,
            12 => self.blue,
            13 => self.lavender,
            14 => self.text,
            15 => self.subtext0,
            16 => self.subtext1,
            17 => self.overlay0,
            18 => self.overlay1,
            19 => self.overlay2,
            20 => self.surface0,
            21 => self.surface1,
            22 => self.surface2,
            23 => self.base,
            24 => self.mantle,
            25 => self.crust,
            _ => unreachable!(),
        }
    }
}

impl CatppuccinFlavor {
    pub(crate) fn frappe() -> CatppuccinFlavor {
        CatppuccinFlavor {
            rosewater: Color::from_u8(0xf2, 0xd5, 0xcf), // #f2d5cf
            flamingo: Color::from_u8(0xee, 0xbe, 0xbe),  // #eebebe
            pink: Color::from_u8(0xf4, 0xb8, 0xe4),      // #f4b8e4
            mauve: Color::from_u8(0xca, 0x9e, 0xe6),     // #ca9ee6
            red: Color::from_u8(0xe7, 0x82, 0x84),       // #e78284
            maroon: Color::from_u8(0xea, 0x99, 0x9c),    // #ea999c
            peach: Color::from_u8(0xef, 0x9f, 0x76),     // #ef9f76
            yellow: Color::from_u8(0xe5, 0xc8, 0x90),    // #e5c890
            green: Color::from_u8(0xa6, 0xd1, 0x89),     // #a6d189
            teal: Color::from_u8(0x81, 0xc8, 0xbe),      // #81c8be
            sky: Color::from_u8(0x99, 0xd1, 0xdb),       // #99d1db
            sapphire: Color::from_u8(0x85, 0xc1, 0xdc),  // #85c1dc
            blue: Color::from_u8(0x8c, 0xaa, 0xee),      // #8caaee
            lavender: Color::from_u8(0xba, 0xbb, 0xf1),  // #babbf1
            text: Color::from_u8(0xc6, 0xd0, 0xf5),      // #c6d0f5
            subtext0: Color::from_u8(0xa5, 0xad, 0xce),  // #a5adce
            subtext1: Color::from_u8(0xb5, 0xbf, 0xe2),  // #b5bfe2
            overlay0: Color::from_u8(0x73, 0x79, 0x94),  // #737994
            overlay1: Color::from_u8(0x83, 0x8b, 0xa7),  // #838ba7
            overlay2: Color::from_u8(0x94, 0x9c, 0xbb),  // #949cbb
            surface0: Color::from_u8(0x41, 0x45, 0x59),  // #414559
            surface1: Color::from_u8(0x51, 0x57, 0x6d),  // #51576d
            surface2: Color::from_u8(0x62, 0x68, 0x80),  // #626880
            base: Color::from_u8(0x30, 0x34, 0x46),      // #303446
            mantle: Color::from_u8(0x29, 0x2c, 0x3c),    // #292c3c
            crust: Color::from_u8(0x23, 0x26, 0x34),     // #232634
        }
    }

    pub(crate) fn latte() -> CatppuccinFlavor {
        CatppuccinFlavor {
            rosewater: Color::from_u8(0xdc, 0x8a, 0x78), // #dc8a78
            flamingo: Color::from_u8(0xdd, 0x78, 0x78),  // #dd7878
            pink: Color::from_u8(0xea, 0x76, 0xcb),      // #ea76cb
            mauve: Color::from_u8(0x88, 0x39, 0xef),     // #8839ef
            red: Color::from_u8(0xd2, 0x0f, 0x39),       // #d20f39
            maroon: Color::from_u8(0xe6, 0x45, 0x53),    // #e64553
            peach: Color::from_u8(0xfe, 0x64, 0x0b),     // #fe640b
            yellow: Color::from_u8(0xdf, 0x8e, 0x1d),    // #df8e1d
            green: Color::from_u8(0x40, 0xa0, 0x2b),     // #40a02b
            teal: Color::from_u8(0x17, 0x92, 0x99),      // #179299
            sky: Color::from_u8(0x04, 0xa5, 0xe5),       // #04a5e5
            sapphire: Color::from_u8(0x20, 0x9f, 0xb5),  // #209fb5
            blue: Color::from_u8(0x1e, 0x66, 0xf5),      // #1e66f5
            lavender: Color::from_u8(0x72, 0x87, 0xfd),  // #7287fd
            text: Color::from_u8(0x4c, 0x4f, 0x69),      // #4c4f69
            subtext0: Color::from_u8(0x6c, 0x6f, 0x85),  // #6c6f85
            subtext1: Color::from_u8(0x5c, 0x5f, 0x77),  // #5c5f77
            overlay0: Color::from_u8(0x9c, 0xa0, 0xb0),  // #9ca0b0
            overlay1: Color::from_u8(0x8c, 0x8f, 0xa1),  // #8c8fa1
            overlay2: Color::from_u8(0x7c, 0x7f, 0x93),  // #7c7f93
            surface0: Color::from_u8(0xcc, 0xd0, 0xda),  // #ccd0da
            surface1: Color::from_u8(0xbc, 0xc0, 0xcc),  // #bcc0cc
            surface2: Color::from_u8(0xac, 0xb0, 0xbe),  // #acb0be
            base: Color::from_u8(0xef, 0xf1, 0xf5),      // #eff1f5
            mantle: Color::from_u8(0xe6, 0xe9, 0xef),    // #e6e9ef
            crust: Color::from_u8(0xdc, 0xe0, 0xe8),     // #dce0e8
        }
    }

    pub(crate) fn macchiato() -> CatppuccinFlavor {
        CatppuccinFlavor {
            rosewater: Color::from_u8(0xf4, 0xdb, 0xd6), // #f4dbd6
            flamingo: Color::from_u8(0xf0, 0xc6, 0xc6),  // #f0c6c6
            pink: Color::from_u8(0xf5, 0xbd, 0xe6),      // #f5bde6
            mauve: Color::from_u8(0xc6, 0xa0, 0xf6),     // #c6a0f6
            red: Color::from_u8(0xed, 0x87, 0x96),       // #ed8796
            maroon: Color::from_u8(0xee, 0x99, 0xa0),    // #ee99a0
            peach: Color::from_u8(0xf5, 0xa9, 0x7f),     // #f5a97f
            yellow: Color::from_u8(0xee, 0xd4, 0x9f),    // #eed49f
            green: Color::from_u8(0xa6, 0xda, 0x95),     // #a6da95
            teal: Color::from_u8(0x8b, 0xd5, 0xca),      // #8bd5ca
            sky: Color::from_u8(0x91, 0xd7, 0xe3),       // #91d7e3
            sapphire: Color::from_u8(0x7d, 0xc4, 0xe4),  // #7dc4e4
            blue: Color::from_u8(0x8a, 0xad, 0xf4),      // #8aadf4
            lavender: Color::from_u8(0xb7, 0xbd, 0xf8),  // #b7bdf8
            text: Color::from_u8(0xca, 0xd3, 0xf5),      // #cad3f5
            subtext0: Color::from_u8(0xa5, 0xad, 0xcb),  // #a5adcb
            subtext1: Color::from_u8(0xb8, 0xc0, 0xe0),  // #b8c0e0
            overlay0: Color::from_u8(0x6e, 0x73, 0x8d),  // #6e738d
            overlay1: Color::from_u8(0x80, 0x87, 0xa2),  // #8087a2
            overlay2: Color::from_u8(0x93, 0x9a, 0xb7),  // #939ab7
            surface0: Color::from_u8(0x36, 0x3a, 0x4f),  // #363a4f
            surface1: Color::from_u8(0x49, 0x4d, 0x64),  // #494d64
            surface2: Color::from_u8(0x5b, 0x60, 0x78),  // #5b6078
            base: Color::from_u8(0x24, 0x27, 0x3a),      // #24273a
            mantle: Color::from_u8(0x1e, 0x20, 0x30),    // #1e2030
            crust: Color::from_u8(0x18, 0x19, 0x26),     // #181926
        }
    }

    pub(crate) fn mocha() -> CatppuccinFlavor {
        CatppuccinFlavor {
            rosewater: Color::from_u8(0xf5, 0xe0, 0xdc), // #f5e0dc
            flamingo: Color::from_u8(0xf2, 0xcd, 0xcd),  // #f2cdcd
            pink: Color::from_u8(0xf5, 0xc2, 0xe7),      // #f5c2e7
            mauve: Color::from_u8(0xcb, 0xa6, 0xf7),     // #cba6f7
            red: Color::from_u8(0xf3, 0x8b, 0xa8),       // #f38ba8
            maroon: Color::from_u8(0xeb, 0xa0, 0xac),    // #eba0ac
            peach: Color::from_u8(0xfa, 0xb3, 0x87),     // #fab387
            yellow: Color::from_u8(0xf9, 0xe2, 0xaf),    // #f9e2af
            green: Color::from_u8(0xa6, 0xe3, 0xa1),     // #a6e3a1
            teal: Color::from_u8(0x94, 0xe2, 0xd5),      // #94e2d5
            sky: Color::from_u8(0x89, 0xdc, 0xeb),       // #89dceb
            sapphire: Color::from_u8(0x74, 0xc7, 0xec),  // #74c7ec
            blue: Color::from_u8(0x89, 0xb4, 0xfa),      // #89b4fa
            lavender: Color::from_u8(0xb4, 0xbe, 0xfe),  // #b4befe
            text: Color::from_u8(0xcd, 0xd6, 0xf4),      // #cdd6f4
            subtext0: Color::from_u8(0xa6, 0xad, 0xc8),  // #a6adc8
            subtext1: Color::from_u8(0xba, 0xc2, 0xde),  // #bac2de
            overlay0: Color::from_u8(0x6c, 0x70, 0x86),  // #6c7086
            overlay1: Color::from_u8(0x7f, 0x84, 0x9c),  // #7f849c
            overlay2: Color::from_u8(0x93, 0x99, 0xb2),  // #9399b2
            surface0: Color::from_u8(0x31, 0x32, 0x44),  // #313244
            surface1: Color::from_u8(0x45, 0x47, 0x5a),  // #45475a
            surface2: Color::from_u8(0x58, 0x5b, 0x70),  // #585b70
            base: Color::from_u8(0x1e, 0x1e, 0x2e),      // #1e1e2e
            mantle: Color::from_u8(0x18, 0x18, 0x25),    // #181825
            crust: Color::from_u8(0x11, 0x11, 0x1b),     // #11111b
        }
    }
}
