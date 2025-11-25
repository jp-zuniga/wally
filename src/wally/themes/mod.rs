use clap::ValueEnum;
use colored::Colorize;

use crate::wally::img::Color;

pub(crate) mod catppuccin;
pub(crate) mod dracula;
pub(crate) mod gruvbox;
pub(crate) mod nord;
pub(crate) mod rosepine;
pub(crate) mod solarized;
pub(crate) mod tokyonight;

pub(crate) trait ColorPalette {
    fn len(&self) -> usize;

    fn background(&self) -> Color;

    fn get_color(&self, idx: usize) -> Color;
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(crate) enum Themes {
    Alucard,
    Dracula,
    CatppuccinFrappe,
    CatppuccinLatte,
    CatppuccinMacchiato,
    CatppuccinMocha,
    GruvboxDark,
    GruvboxLight,
    Nord,
    RosePineDefault,
    RosePineDawn,
    RosePineMoon,
    SolarizedDark,
    SolarizedLight,
    TokyoNightDefault,
    TokyoNightDay,
    TokyoNightMoon,
    TokyoNightStorm,
}

impl Themes {
    pub(crate) fn get_variants() -> Vec<String> {
        Themes::value_variants()
            .iter()
            .filter_map(|v| v.to_possible_value().map(|pv| pv.get_name().to_string()))
            .collect()
    }
}

pub(crate) fn print_palettes() {
    println!();
    println!("{}", "Available color palettes:".purple().bold());

    for name in Themes::get_variants() {
        println!("  {} {}", "-".purple().bold(), name.green().italic());
    }
}
