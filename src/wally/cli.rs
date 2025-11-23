use clap::{ArgAction, ColorChoice, crate_authors, crate_version};
use clap::{Parser, Subcommand};

use super::consts::{
    DEFAULT_DOT_SIZE, DEFAULT_HEIGHT, DEFAULT_NAME, DEFAULT_PADDING, DEFAULT_STEPS,
    DEFAULT_WIDTH, WALLY_STYLE,
};

use super::img::WallFormats;
use super::themes::Themes;
use super::utils::{parse_file_arg, parse_float};

#[derive(Clone, Copy, Debug, Subcommand)]
pub(crate) enum Commands {
    /// Create a wallpaper of randomly-generated dots.
    Dots {
        /// Size of dots generated.
        #[arg(short, long, default_value_t = DEFAULT_DOT_SIZE, value_parser = parse_float)]
        dot_size: f32,

        /// Density of generated dots.
        #[arg(short, long, default_value_t = DEFAULT_STEPS)]
        steps: u32,
    },
}

#[derive(Parser, Debug)]
#[command(
    arg_required_else_help = true,
    color = ColorChoice::Auto,
    styles = WALLY_STYLE,
    author = crate_authors!(),
    version = crate_version!(),
    about = None,
    long_about = None,
)]
pub struct WallyCLI {
    /// Optional seed for reproducible wallpapers.
    #[arg(long, global = true)]
    pub(crate) seed: Option<u64>,

    /// Name of generated wallpaper.
    #[arg(
        short,
        long,
        global = true,
        default_value_t = DEFAULT_NAME.to_string(),
        value_parser = parse_file_arg,
    )]
    pub(crate) name: String,

    /// Color palette of generated wallpaper.
    #[arg(
        short,
        long,
        global = true,
        value_enum,
        default_value_t = Themes::RosePineMoon,
    )]
    pub(crate) palette: Themes,

    /// Output format of generated wallpaper.
    #[arg(short, long, global = true, default_value_t = WallFormats::Png, value_enum)]
    pub(crate) format: WallFormats,

    /// Pixels of padding around wallpaper borders.
    #[arg(long, global = true, default_value_t = DEFAULT_PADDING)]
    pub(crate) padding: u32,

    /// Width of generated wallpaper.
    #[arg(short = 'W', long, global = true, default_value_t = DEFAULT_WIDTH)]
    pub(crate) width: u32,

    /// Height of generated wallpaper.
    #[arg(short = 'H', long, global = true, default_value_t = DEFAULT_HEIGHT)]
    pub(crate) height: u32,

    /// Swap width and height to create a vertical wallpaper.
    #[arg(long, global = true, action = ArgAction::SetTrue)]
    pub(crate) swap: bool,

    /// Command to execute.
    #[command(subcommand)]
    pub(crate) command: Commands,
}
