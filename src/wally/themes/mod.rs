pub(crate) mod catppuccin;
pub(crate) mod dracula;
pub(crate) mod rosepine;

use clap::ValueEnum;

use crate::wally::img::Color;

#[derive(Clone, Copy, Debug, ValueEnum)]
#[clap(rename_all = "kebab_case")]
pub(crate) enum Themes {
    CatppuccinFrappe,
    CatppuccinLatte,
    CatppuccinMacchiato,
    CatppuccinMocha,
    Dracula,
    Alucard,
    RosePineDefault,
    RosePineDawn,
    RosePineMoon,
}

pub(crate) trait ColorPalette {
    fn len(&self) -> usize;

    fn background(&self) -> Color;

    fn get_color(&self, idx: usize) -> Color;
}
