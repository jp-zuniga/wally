use crate::wally::img::Color;

use super::Palette;

impl Palette {
    pub(crate) fn noctis() -> Self {
        Palette {
            foreground: Color::from_u8(0xb2, 0xca, 0xcd), // #b2cacd
            accent: Color::from_u8(0x08, 0x3d, 0x44),     // #083d44
            base: Color::from_u8(0x03, 0x19, 0x1b),       // #03191b
            black: Color::from_u8(0x32, 0x4a, 0x4d),      // #324a4d
            red: Color::from_u8(0xe6, 0x65, 0x33),        // #e66533
            green: Color::from_u8(0x49, 0xe9, 0xa6),      // #49e9a6
            yellow: Color::from_u8(0xe4, 0xb7, 0x81),     // #e4b781
            blue: Color::from_u8(0x49, 0xac, 0xe9),       // #49ace9
            magenta: Color::from_u8(0xdf, 0x76, 0x9b),    // #df769b
            cyan: Color::from_u8(0x49, 0xd6, 0xe9),       // #49d6e9
            white: Color::from_u8(0xb2, 0xca, 0xcd),      // #b2cacd
        }
    }

    pub(crate) fn noctis_sereno() -> Self {
        Palette {
            foreground: Color::from_u8(0xb2, 0xca, 0xcd), // #b2cacd
            accent: Color::from_u8(0x09, 0x40, 0x47),     // #094047
            base: Color::from_u8(0x04, 0x1d, 0x20),       // #041d20
            black: Color::from_u8(0x32, 0x4a, 0x4d),      // #324a4d
            red: Color::from_u8(0xe6, 0x65, 0x33),        // #e66533
            green: Color::from_u8(0x49, 0xe9, 0xa6),      // #49e9a6
            yellow: Color::from_u8(0xe4, 0xb7, 0x81),     // #e4b781
            blue: Color::from_u8(0x49, 0xac, 0xe9),       // #49ace9
            magenta: Color::from_u8(0xdf, 0x76, 0x9b),    // #df769b
            cyan: Color::from_u8(0x49, 0xd6, 0xe9),       // #49d6e9
            white: Color::from_u8(0xb2, 0xca, 0xcd),      // #b2cacd
        }
    }

    pub(crate) fn noctis_minimus() -> Self {
        Palette {
            foreground: Color::from_u8(0xc5, 0xcd, 0xd3), // #c5cdd3
            accent: Color::from_u8(0x21, 0x35, 0x41),     // #213541
            base: Color::from_u8(0x0e, 0x19, 0x20),       // #0e1920
            black: Color::from_u8(0x18, 0x2a, 0x35),      // #182a35
            red: Color::from_u8(0xc0, 0x88, 0x72),        // #c08872
            green: Color::from_u8(0x72, 0xc0, 0x9f),      // #72c09f
            yellow: Color::from_u8(0xc8, 0xa9, 0x84),     // #c8a984
            blue: Color::from_u8(0x61, 0x96, 0xb8),       // #6196b8
            magenta: Color::from_u8(0xc2, 0x80, 0x97),    // #c28097
            cyan: Color::from_u8(0x72, 0xb7, 0xc0),       // #72b7c0
            white: Color::from_u8(0xc5, 0xcd, 0xd3),      // #c5cdd3
        }
    }

    pub(crate) fn noctis_obscuro() -> Self {
        Palette {
            foreground: Color::from_u8(0xb2, 0xca, 0xcd), // #b2cacd
            accent: Color::from_u8(0x07, 0x34, 0x3a),     // #07343a
            base: Color::from_u8(0x02, 0x0c, 0x0e),       // #020c0e
            black: Color::from_u8(0x32, 0x4a, 0x4d),      // #324a4d
            red: Color::from_u8(0xe6, 0x65, 0x33),        // #e66533
            green: Color::from_u8(0x49, 0xe9, 0xa6),      // #49e9a6
            yellow: Color::from_u8(0xe4, 0xb7, 0x81),     // #e4b781
            blue: Color::from_u8(0x49, 0xac, 0xe9),       // #49ace9
            magenta: Color::from_u8(0xdf, 0x76, 0x9b),    // #df769b
            cyan: Color::from_u8(0x49, 0xd6, 0xe9),       // #49d6e9
            white: Color::from_u8(0xb2, 0xca, 0xcd),      // #b2cacd
        }
    }

    pub(crate) fn noctis_azureus() -> Self {
        Palette {
            foreground: Color::from_u8(0xbe, 0xcf, 0xda), // #becfda
            accent: Color::from_u8(0x0b, 0x3a, 0x58),     // #0b3a58
            base: Color::from_u8(0x05, 0x1b, 0x29),       // #051b29
            black: Color::from_u8(0x28, 0x35, 0x3e),      // #28353e
            red: Color::from_u8(0xe6, 0x65, 0x33),        // #e66533
            green: Color::from_u8(0x49, 0xe9, 0xa6),      // #49e9a6
            yellow: Color::from_u8(0xe4, 0xb7, 0x81),     // #e4b781
            blue: Color::from_u8(0x49, 0xac, 0xe9),       // #49ace9
            magenta: Color::from_u8(0xdf, 0x76, 0x9b),    // #df769b
            cyan: Color::from_u8(0x49, 0xd6, 0xe9),       // #49d6e9
            white: Color::from_u8(0xae, 0xc3, 0xd0),      // #aec3d0
        }
    }

    pub(crate) fn noctis_uva() -> Self {
        Palette {
            foreground: Color::from_u8(0xc5, 0xc2, 0xd6), // #c5c2d6
            accent: Color::from_u8(0x39, 0x35, 0x58),     // #393558
            base: Color::from_u8(0x1f, 0x1d, 0x30),       // #1f1d30
            black: Color::from_u8(0x30, 0x2f, 0x3d),      // #302f3d
            red: Color::from_u8(0xe6, 0x65, 0x33),        // #e66533
            green: Color::from_u8(0x49, 0xe9, 0xa6),      // #49e9a6
            yellow: Color::from_u8(0xe4, 0xb7, 0x81),     // #e4b781
            blue: Color::from_u8(0x49, 0xac, 0xe9),       // #49ace9
            magenta: Color::from_u8(0xdf, 0x76, 0x9b),    // #df769b
            cyan: Color::from_u8(0x49, 0xd6, 0xe9),       // #49d6e9
            white: Color::from_u8(0xb6, 0xb3, 0xcc),      // #b6b3cc
        }
    }

    pub(crate) fn noctis_viola() -> Self {
        Palette {
            foreground: Color::from_u8(0xcc, 0xbf, 0xd9), // #ccbfd9
            accent: Color::from_u8(0x48, 0x35, 0x5b),     // #48355b
            base: Color::from_u8(0x29, 0x1d, 0x35),       // #291d35
            black: Color::from_u8(0x36, 0x2f, 0x3d),      // #362f3d
            red: Color::from_u8(0xe6, 0x65, 0x33),        // #e66533
            green: Color::from_u8(0x49, 0xe9, 0xa6),      // #49e9a6
            yellow: Color::from_u8(0xe4, 0xb7, 0x81),     // #e4b781
            blue: Color::from_u8(0x49, 0xac, 0xe9),       // #49ace9
            magenta: Color::from_u8(0xdf, 0x76, 0x9b),    // #df769b
            cyan: Color::from_u8(0x49, 0xd6, 0xe9),       // #49d6e9
            white: Color::from_u8(0xbf, 0xaf, 0xcf),      // #bfafcf
        }
    }

    pub(crate) fn noctis_bordo() -> Self {
        Palette {
            foreground: Color::from_u8(0xcb, 0xbe, 0xc2), // #cbbec2
            accent: Color::from_u8(0x4d, 0x3c, 0x42),     // #4d3c42
            base: Color::from_u8(0x27, 0x20, 0x22),       // #272022
            black: Color::from_u8(0x47, 0x39, 0x3e),      // #47393e
            red: Color::from_u8(0xe6, 0x65, 0x33),        // #e66533
            green: Color::from_u8(0x49, 0xe9, 0xa6),      // #49e9a6
            yellow: Color::from_u8(0xe4, 0xb7, 0x81),     // #e4b781
            blue: Color::from_u8(0x49, 0xac, 0xe9),       // #49ace9
            magenta: Color::from_u8(0xdf, 0x76, 0x9b),    // #df769b
            cyan: Color::from_u8(0x49, 0xd6, 0xe9),       // #49d6e9
            white: Color::from_u8(0xb9, 0xac, 0xb0),      // #b9acb0
        }
    }

    pub(crate) fn noctis_hibernus() -> Self {
        Palette {
            foreground: Color::from_u8(0x00, 0x56, 0x61), // #005661
            accent: Color::from_u8(0xc9, 0xe8, 0xed),     // #c9e8ed
            base: Color::from_u8(0xe1, 0xee, 0xef),       // #e1eeef
            black: Color::from_u8(0x00, 0x3b, 0x42),      // #003b42
            red: Color::from_u8(0xe3, 0x4e, 0x1c),        // #e34e1c
            green: Color::from_u8(0x00, 0xb3, 0x68),      // #00b368
            yellow: Color::from_u8(0xf4, 0x97, 0x25),     // #f49725
            blue: Color::from_u8(0x00, 0x94, 0xf0),       // #0094f0
            magenta: Color::from_u8(0xff, 0x57, 0x92),    // #ff5792
            cyan: Color::from_u8(0x00, 0xbd, 0xd6),       // #00bdd6
            white: Color::from_u8(0x8c, 0xa6, 0xa6),      // #8ca6a6
        }
    }

    pub(crate) fn noctis_lilac() -> Self {
        Palette {
            foreground: Color::from_u8(0x0c, 0x00, 0x6b), // #0c006b
            accent: Color::from_u8(0xe9, 0xe7, 0xf3),     // #e9e7f3
            base: Color::from_u8(0xd0, 0xcc, 0xef),       // #d0ccef
            black: Color::from_u8(0x0c, 0x00, 0x6b),      // #0c006b
            red: Color::from_u8(0xe3, 0x4e, 0x1c),        // #e34e1c
            green: Color::from_u8(0x00, 0xb3, 0x68),      // #00b368
            yellow: Color::from_u8(0xf4, 0x97, 0x25),     // #f49725
            blue: Color::from_u8(0x00, 0x94, 0xf0),       // #0094f0
            magenta: Color::from_u8(0xff, 0x57, 0x92),    // #ff5792
            cyan: Color::from_u8(0x00, 0xbd, 0xd6),       // #00bdd6
            white: Color::from_u8(0x8c, 0xa6, 0xa6),      // #8ca6a6
        }
    }

    pub(crate) fn noctis_lux() -> Self {
        Palette {
            foreground: Color::from_u8(0x00, 0x56, 0x61), // #005661
            accent: Color::from_u8(0xf6, 0xed, 0xda),     // #f6edda
            base: Color::from_u8(0xd4, 0xe8, 0xe2),       // #d4e8e2
            black: Color::from_u8(0x00, 0x3b, 0x42),      // #003b42
            red: Color::from_u8(0xe3, 0x4e, 0x1c),        // #e34e1c
            green: Color::from_u8(0x00, 0xb3, 0x68),      // #00b368
            yellow: Color::from_u8(0xf4, 0x97, 0x25),     // #f49725
            blue: Color::from_u8(0x00, 0x94, 0xf0),       // #0094f0
            magenta: Color::from_u8(0xff, 0x57, 0x92),    // #ff5792
            cyan: Color::from_u8(0x00, 0xbd, 0xd6),       // #00bdd6
            white: Color::from_u8(0x8c, 0xa6, 0xa6),      // #8ca6a6
        }
    }
}
