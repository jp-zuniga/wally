use clap::{ArgAction, crate_authors, crate_version};
use clap::{Parser, Subcommand};

use super::consts::{
    DEFAULT_DOT_SIZE, DEFAULT_HEIGHT, DEFAULT_NAME, DEFAULT_PADDING, DEFAULT_STEPS,
    DEFAULT_WIDTH, WALLY_STYLE,
};

use super::img::WallFormats;
use super::parse::{
    parse_dot_size, parse_file_name, parse_height, parse_padding, parse_steps,
    parse_width,
};

use super::themes::catppuccin::CatppuccinFlavor;
use super::themes::dracula::DraculaFlavor;
use super::themes::gruvbox::GruvboxFlavor;
use super::themes::nord::Nord;
use super::themes::rosepine::RosePineFlavor;
use super::themes::solarized::SolarizedFlavor;
use super::themes::tokyonight::TokyoNightFlavor;
use super::themes::{ColorPalette, Themes};

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
    Themes,
}

#[derive(Parser, Debug)]
#[command(
    arg_required_else_help = true,
    styles = WALLY_STYLE,
    author = crate_authors!(),
    version = crate_version!(),
    about = None,
    long_about = None,
)]
pub struct WallyCLI {
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
    #[arg(short, long, value_enum, global = true, default_value_t = WallFormats::Png)]
    pub(crate) format: WallFormats,

    /// Color palette of generated wallpaper.
    #[arg(
        short,
        long,
        value_enum,
        global = true,
        hide_possible_values = true,
        default_value_t = Themes::RosePineMoon,
    )]
    pub(crate) palette: Themes,

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

impl WallyCLI {
    pub(crate) fn colorize(&self) -> Option<bool> {
        if self.color {
            Some(true)
        } else if self.no_color {
            Some(false)
        } else {
            None
        }
    }

    pub(crate) fn mk_palette(&self) -> Box<dyn ColorPalette> {
        match self.palette {
            Themes::Alucard => Box::new(DraculaFlavor::alucard()),
            Themes::Dracula => Box::new(DraculaFlavor::default()),
            Themes::CatppuccinFrappe => Box::new(CatppuccinFlavor::frappe()),
            Themes::CatppuccinLatte => Box::new(CatppuccinFlavor::latte()),
            Themes::CatppuccinMacchiato => Box::new(CatppuccinFlavor::macchiato()),
            Themes::CatppuccinMocha => Box::new(CatppuccinFlavor::mocha()),
            Themes::GruvboxDark => Box::new(GruvboxFlavor::dark()),
            Themes::GruvboxLight => Box::new(GruvboxFlavor::light()),
            Themes::Nord => Box::new(Nord::new()),
            Themes::RosePineDawn => Box::new(RosePineFlavor::dawn()),
            Themes::RosePineDefault => Box::new(RosePineFlavor::default()),
            Themes::RosePineMoon => Box::new(RosePineFlavor::moon()),
            Themes::SolarizedDark => Box::new(SolarizedFlavor::dark()),
            Themes::SolarizedLight => Box::new(SolarizedFlavor::light()),
            Themes::TokyoNightDefault => Box::new(TokyoNightFlavor::default()),
            Themes::TokyoNightDay => Box::new(TokyoNightFlavor::day()),
            Themes::TokyoNightMoon => Box::new(TokyoNightFlavor::moon()),
            Themes::TokyoNightStorm => Box::new(TokyoNightFlavor::storm()),
        }
    }
}
