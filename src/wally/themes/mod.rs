use clap::ValueEnum;

use crate::wally::img::Color;

pub(crate) mod catppuccin;
pub(crate) mod dracula;
pub(crate) mod gruvbox;
pub(crate) mod nord;
pub(crate) mod rosepine;
pub(crate) mod solarized;
pub(crate) mod tokyonight;

#[derive(Clone, Copy, Debug, ValueEnum)]
#[clap(rename_all = "kebab_case")]
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

pub(crate) trait ColorPalette {
    fn len(&self) -> usize;

    fn background(&self) -> Color;

    fn get_color(&self, idx: usize) -> Color;
}
