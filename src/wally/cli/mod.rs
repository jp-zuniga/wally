use clap::{ArgAction, crate_authors, crate_version};
use clap::{Parser, Subcommand};

pub(crate) mod error;
pub(crate) mod parse;
pub(crate) mod term;

use parse::{
    parse_dot_size, parse_file_name, parse_height, parse_padding, parse_steps,
    parse_width,
};

use crate::wally::consts::{
    CLI_STYLE, DEFAULT_DOT_SIZE, DEFAULT_HEIGHT, DEFAULT_NAME, DEFAULT_PADDING,
    DEFAULT_STEPS, DEFAULT_WIDTH,
};
use crate::wally::img::WallFormats;
use crate::wally::themes::{Palette, WallyPalettes};

#[derive(Clone, Copy, Debug, Subcommand)]
pub(crate) enum WallyCommands {
    /// Create a wallpaper of randomly-generated dots.
    Dots {
        /// Radius of generated dots.
        #[arg(short, long, default_value_t = DEFAULT_DOT_SIZE, value_parser = parse_dot_size)]
        dot_size: f32,

        /// Density of generated dots.
        #[arg(short, long, default_value_t = DEFAULT_STEPS, value_parser = parse_steps)]
        steps: u32,
    },

    /// List available color palettes.
    WallyPalettes,
}

#[derive(Parser, Debug)]
#[command(
    arg_required_else_help = true,
    styles = CLI_STYLE,
    author = crate_authors!(),
    version = crate_version!(),
    about = None,
    long_about = None,
)]
pub struct WallyCli {
    /// Optional seed for reproducibility.
    #[arg(long, global = true)]
    pub(crate) seed: Option<u64>,

    /// Name of generated wallpaper.
    #[arg(
        short,
        long,
        global = true,
        default_value_t = DEFAULT_NAME.to_string(),
        value_parser = parse_file_name,
    )]
    pub(crate) name: String,

    /// Output format of generated wallpaper.
    #[arg(short, long, value_enum, global = true, default_value_t = WallFormats::default())]
    pub(crate) format: WallFormats,

    /// Color palette of generated wallpaper.
    #[arg(
        short,
        long,
        value_enum,
        global = true,
        hide_possible_values = true,
        default_value_t = WallyPalettes::RosePineMoon,
    )]
    pub(crate) palette: WallyPalettes,

    /// Width of generated wallpaper.
    #[arg(short = 'W', long, global = true, default_value_t = DEFAULT_WIDTH, value_parser = parse_width)]
    pub(crate) width: u32,

    /// Height of generated wallpaper.
    #[arg(short = 'H', long, global = true, default_value_t = DEFAULT_HEIGHT, value_parser = parse_height)]
    pub(crate) height: u32,

    /// Padding around wallpaper borders.
    #[arg(long, global = true, default_value_t = DEFAULT_PADDING, value_parser = parse_padding)]
    pub(crate) padding: u32,

    /// Swap width and height.
    #[arg(long, global = true, action = ArgAction::SetTrue)]
    pub(crate) swap: bool,

    /// Force color output.
    #[arg(long, global = true, conflicts_with = "no_color", action = ArgAction::SetTrue)]
    pub(crate) color: bool,

    /// Disable color output.
    #[arg(long, global = true, conflicts_with = "color", action = ArgAction::SetTrue)]
    pub(crate) no_color: bool,

    /// Command to execute.
    #[command(subcommand)]
    pub(crate) command: WallyCommands,
}

impl WallyCli {
    pub(crate) fn colorize(&self) -> Option<bool> {
        if self.color {
            Some(true)
        } else if self.no_color {
            Some(false)
        } else {
            None
        }
    }

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
