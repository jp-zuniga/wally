use clap::{ArgAction, ColorChoice, crate_authors, crate_version};
use clap::{Parser, Subcommand};

use super::consts::{
    DEFAULT_DOT_SIZE, DEFAULT_HEIGHT, DEFAULT_NAME, DEFAULT_PADDING, DEFAULT_STEPS,
    DEFAULT_WIDTH, WALLY_STYLE,
};

use super::img::WallFormats;
use super::themes::Themes;
use super::utils::{
    parse_dot_size, parse_file_name, parse_height, parse_padding, parse_steps,
    parse_width,
};

#[derive(Clone, Copy, Debug, Subcommand)]
pub(crate) enum Commands {
    /// Create a wallpaper of randomly-generated dots.
    Dots {
        /// Radius of generated dots.
        #[arg(short, long, default_value_t = DEFAULT_DOT_SIZE, value_parser = parse_dot_size)]
        dot_size: f32,

        /// Density of generated dots.
        #[arg(short, long, default_value_t = DEFAULT_STEPS, value_parser = parse_steps)]
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
        value_parser = parse_file_name,
    )]
    pub(crate) name: String,

    /// Color palette of generated wallpaper.
    #[arg(
        short,
        long,
        value_enum,
        global = true,
        default_value_t = Themes::RosePineMoon,
    )]
    pub(crate) palette: Themes,

    /// Output format of generated wallpaper.
    #[arg(short, long, value_enum, global = true, default_value_t = WallFormats::Png)]
    pub(crate) format: WallFormats,

    /// Pixels of padding around wallpaper borders.
    #[arg(long, global = true, default_value_t = DEFAULT_PADDING, value_parser = parse_padding)]
    pub(crate) padding: u32,

    /// Width of generated wallpaper.
    #[arg(short = 'W', long, global = true, default_value_t = DEFAULT_WIDTH, value_parser = parse_width)]
    pub(crate) width: u32,

    /// Height of generated wallpaper.
    #[arg(short = 'H', long, global = true, default_value_t = DEFAULT_HEIGHT, value_parser = parse_height)]
    pub(crate) height: u32,

    /// Swap width and height.
    #[arg(long, global = true, action = ArgAction::SetTrue)]
    pub(crate) swap: bool,

    /// Command to execute.
    #[command(subcommand)]
    pub(crate) command: Commands,
}
