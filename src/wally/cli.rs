use clap::{ArgAction, crate_authors, crate_version};
use clap::{Parser, Subcommand};

use super::consts::{
    DEFAULT_DOT_SIZE, DEFAULT_HEIGHT, DEFAULT_NAME, DEFAULT_PADDING, DEFAULT_STEPS, DEFAULT_WIDTH,
};

use super::img::WallFormats;
use super::themes::Themes;
use super::utils::{parse_file_arg, parse_float};

#[derive(Clone, Copy, Debug, Subcommand)]
pub(crate) enum Commands {
    /// Generate a wallpaper of randomly-generated dots.
    Dots {
        /// Size of circles generated.
        #[arg(short, long, default_value_t = DEFAULT_DOT_SIZE, value_parser = parse_float)]
        dot_size: f32,

        /// Controls the density of generated dots.
        /// Lower generates denser, tight patterns; high creates a spread-out grid.
        #[arg(short, long, default_value_t = DEFAULT_STEPS)]
        steps: u32,

        /// Color palette for generated wallpaper.
        #[arg(short, long, value_enum, default_value_t = Themes::RosePineMoon)]
        palette: Themes,
    },
}

#[derive(Parser, Debug)]
#[command(
    author = crate_authors!(),
    version = crate_version!(),
    about = None,
    long_about = None,
    arg_required_else_help = true,
)]
pub struct WallyCLI {
    /// Name of generated wallpaper.
    #[arg(short, long, default_value_t = DEFAULT_NAME.to_string(), value_parser = parse_file_arg)]
    pub(crate) name: String,

    /// Image format wallpaper will be saved as.
    #[arg(short, long, default_value_t = WallFormats::Png, value_enum)]
    pub(crate) format: WallFormats,

    /// Width of wallpaper.
    #[arg(short = 'W', long, default_value_t = DEFAULT_WIDTH)]
    pub(crate) width: u32,

    /// Height of wallpaper.
    #[arg(short = 'H', long, default_value_t = DEFAULT_HEIGHT)]
    pub(crate) height: u32,

    /// Amount of padding to add to wallpaper's borders.
    #[arg(short, long, default_value_t = DEFAULT_PADDING)]
    pub(crate) padding: u32,

    /// Whether to swap width and height values to create a vertical wallpaper.
    #[arg(long, action = ArgAction::SetTrue)]
    pub(crate) swap: bool,

    /// Random seed for reproducible output.
    /// If omitted, a random seed is used.
    #[arg(long)]
    pub(crate) seed: Option<u64>,

    /// Command to execute.
    #[command(subcommand)]
    pub(crate) command: Commands,
}
