use clap::{ArgAction, Args};

use crate::wally::consts::{
    DEFAULT_HEIGHT, DEFAULT_NAME, DEFAULT_PADDING, DEFAULT_WIDTH,
};
use crate::wally::img::WallFormats;
use crate::wally::themes::{Palette, WallyPalettes};

use super::parse::{parse_file_name, parse_height, parse_padding, parse_width};

#[derive(Args, Clone, Debug)]
pub(crate) struct WallArgs {
    /// Name of generated wallpaper.
    #[arg(
        short,
        long,
        help_heading = "Wallpaper Options",
        default_value_t = DEFAULT_NAME.to_string(),
        value_parser = parse_file_name,
    )]
    pub(crate) name: String,

    /// Output format of generated wallpaper.
    #[arg(
        short,
        long,
        hide_possible_values = true,
        help_heading = "Wallpaper Options",
        value_enum,
        default_value_t = WallFormats::default(),
    )]
    pub(crate) format: WallFormats,

    /// Color palette of generated wallpaper.
    #[arg(
        short,
        long,
        help_heading = "Wallpaper Options",
        value_enum,
        hide_possible_values = true,
        default_value_t = WallyPalettes::RosePineMoon,
    )]
    pub(crate) palette: WallyPalettes,

    /// Width of generated wallpaper.
    #[arg(
        short = 'W',
        long,
        help_heading = "Wallpaper Options",
        default_value_t = DEFAULT_WIDTH,
        value_parser = parse_width,
    )]
    pub(crate) width: u32,

    /// Height of generated wallpaper.
    #[arg(
        short = 'H',
        long,
        help_heading = "Wallpaper Options",
        default_value_t = DEFAULT_HEIGHT,
        value_parser = parse_height,
    )]
    pub(crate) height: u32,

    /// Padding around wallpaper borders.
    #[arg(
        short = 'P',
        long,
        help_heading = "Wallpaper Options",
        default_value_t = DEFAULT_PADDING,
        value_parser = parse_padding,
    )]
    pub(crate) padding: u32,

    /// Swap width and height.
    #[arg(long, help_heading = "Wallpaper Options", action = ArgAction::SetTrue)]
    pub(crate) swap: bool,

    /// Optional seed for reproducibility.
    #[arg(long, help_heading = "Wallpaper Options")]
    pub(crate) seed: Option<u64>,
}

impl WallArgs {
    pub(crate) fn mk_palette(&self) -> Palette {
        match self.palette {
            WallyPalettes::Alucard => Palette::alucard(),
            WallyPalettes::CatppuccinFrappe => Palette::cat_frappe(),
            WallyPalettes::CatppuccinLatte => Palette::cat_latte(),
            WallyPalettes::CatppuccinMacchiato => Palette::cat_macchiato(),
            WallyPalettes::CatppuccinMocha => Palette::cat_mocha(),
            WallyPalettes::Dracula => Palette::dracula(),
            WallyPalettes::EverforestDark => Palette::evf_dark(),
            WallyPalettes::EverforestLight => Palette::evf_light(),
            WallyPalettes::GruvboxDark => Palette::gruv_dark(),
            WallyPalettes::GruvboxLight => Palette::gruv_light(),
            WallyPalettes::Moonfly => Palette::moonfly(),
            WallyPalettes::Noctis => Palette::noctis(),
            WallyPalettes::NoctisSereno => Palette::noctis_sereno(),
            WallyPalettes::NoctisMinimus => Palette::noctis_minimus(),
            WallyPalettes::NoctisObscuro => Palette::noctis_obscuro(),
            WallyPalettes::NoctisAzureus => Palette::noctis_azureus(),
            WallyPalettes::NoctisUva => Palette::noctis_uva(),
            WallyPalettes::NoctisViola => Palette::noctis_viola(),
            WallyPalettes::NoctisBordo => Palette::noctis_bordo(),
            WallyPalettes::NoctisHibernus => Palette::noctis_hibernus(),
            WallyPalettes::NoctisLilac => Palette::noctis_lilac(),
            WallyPalettes::NoctisLux => Palette::noctis_lux(),
            WallyPalettes::Nord => Palette::nord(),
            WallyPalettes::RosePineDawn => Palette::rose_dawn(),
            WallyPalettes::RosePine => Palette::rose_pine(),
            WallyPalettes::RosePineMoon => Palette::rose_moon(),
            WallyPalettes::SolarizedDark => Palette::sol_dark(),
            WallyPalettes::SolarizedLight => Palette::sol_light(),
            WallyPalettes::TokyoNightDefault => Palette::tokyo_night(),
            WallyPalettes::TokyoNightDay => Palette::tokyo_day(),
            WallyPalettes::TokyoNightMoon => Palette::tokyo_moon(),
            WallyPalettes::TokyoNightStorm => Palette::tokyo_storm(),
        }
    }
}
