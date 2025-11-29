use clap::ValueEnum;
use colored::Colorize;

use crate::wally::img::Color;

mod catppuccin;
mod dracula;
mod everforest;
mod gruvbox;
mod moonfly;
mod noctis;
mod nord;
mod rosepine;
mod solarized;
mod tokyonight;

const COLOR_COUNT: usize = 11;

pub(crate) fn print_palettes() {
    println!();
    println!("{}", "Available color palettes:".purple().bold());

    for name in WallyPalettes::get_variants() {
        println!("  {} {}", "-".purple().bold(), name.green().italic());
    }
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(crate) enum WallyPalettes {
    Alucard,
    CatppuccinFrappe,
    CatppuccinLatte,
    CatppuccinMacchiato,
    CatppuccinMocha,
    Dracula,
    EverforestDark,
    EverforestLight,
    GruvboxDark,
    GruvboxLight,
    Moonfly,
    Noctis,
    NoctisSereno,
    NoctisMinimus,
    NoctisObscuro,
    NoctisAzureus,
    NoctisUva,
    NoctisViola,
    NoctisBordo,
    NoctisHibernus,
    NoctisLilac,
    NoctisLux,
    Nord,
    RosePine,
    RosePineDawn,
    RosePineMoon,
    SolarizedDark,
    SolarizedLight,
    TokyoNightDefault,
    TokyoNightDay,
    TokyoNightMoon,
    TokyoNightStorm,
}

impl WallyPalettes {
    pub(crate) fn get_variants() -> Vec<String> {
        WallyPalettes::value_variants()
            .iter()
            .filter_map(|v| v.to_possible_value().map(|pv| pv.get_name().to_string()))
            .collect()
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Palette {
    foreground: Color,
    accent: Color,
    base: Color,
    black: Color,
    red: Color,
    yellow: Color,
    green: Color,
    cyan: Color,
    blue: Color,
    magenta: Color,
    white: Color,
}

impl Palette {
    pub(crate) fn len(&self) -> usize {
        COLOR_COUNT
    }

    pub(crate) fn background(&self) -> Color {
        self.base
    }

    pub(crate) fn get_color(&self, idx: usize) -> Color {
        match idx {
            0 => self.foreground,
            1 => self.accent,
            2 => self.base,
            3 => self.black,
            4 => self.red,
            5 => self.yellow,
            6 => self.green,
            7 => self.cyan,
            8 => self.blue,
            9 => self.magenta,
            10 => self.white,
            _ => unreachable!(),
        }
    }
}
