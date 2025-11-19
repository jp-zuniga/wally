use clap::ValueEnum;

use crate::wally::img::Color;

#[derive(Clone, Copy, Debug, ValueEnum)]
#[clap(rename_all = "kebab_case")]
pub(crate) enum Theme {
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

    // fn get_colors(&self) -> Vec<Color> {
    //     (0..self.len()).map(|i| self.get_color(i)).collect()
    // }
}
