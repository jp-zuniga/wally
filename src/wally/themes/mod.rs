pub(crate) mod catppuccin;
pub(crate) mod rose_pine;

use clap::ValueEnum;

use crate::wally::img::Color;

#[derive(Clone, Copy, Debug, ValueEnum)]
#[clap(rename_all = "kebab_case")]
pub(crate) enum Themes {
    RosePineDefault,
    RosePineDawn,
    RosePineMoon,
    CatppuccinFrappe,
    CatppuccinLatte,
    CatppuccinMacchiato,
    CatppuccinMocha,
}

pub(crate) trait ThemeFlavor {
    fn len(&self) -> usize;

    fn background(&self) -> Color;

    fn get_color(&self, idx: usize) -> Color;
}
