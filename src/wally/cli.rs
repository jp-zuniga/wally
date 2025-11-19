use clap::{ArgAction, crate_authors, crate_version};
use clap::{Parser, Subcommand};

use super::palettes::theme::Theme;
use super::img::ImgFormats;
use super::utils::{parse_float, parse_str};

pub(crate) const DEFAULT_DOT_SIZE: f64 = 40.0;
pub(crate) const DEFAULT_FILE_NAME: &str = "dots";
pub(crate) const DEFAULT_HEIGHT: u32 = 1080;
pub(crate) const DEFAULT_PADDING: u32 = 200;
pub(crate) const DEFAULT_STEPS: u32 = 80;
pub(crate) const DEFAULT_WIDTH: u32 = 1920;

#[derive(Clone, Copy, Debug, Subcommand)]
pub(crate) enum Commands {
    /// Generate a wallpaper of randomly-generated dots.
    Dots {
        /// Color palette for generated wallpaper.
        #[arg(short, long, value_enum, default_value_t = Theme::RosePineMoon)]
        palette: Theme,
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
    /// Name of generated image.
    #[arg(short, long, default_value_t = DEFAULT_FILE_NAME.to_string(), value_parser = parse_str)]
    pub(crate) file_name: String,

    /// Image format for wallpaper.
    #[arg(short = 'F', long, default_value_t = ImgFormats::Png, value_enum)]
    pub(crate) format: ImgFormats,

    /// Width of wallpaper.
    #[arg(short = 'W', long, default_value_t = DEFAULT_WIDTH)]
    pub(crate) width: u32,

    /// Height of wallpaper.
    #[arg(short = 'H', long, default_value_t = DEFAULT_HEIGHT)]
    pub(crate) height: u32,

    /// Amount of steps forward per iteration when generating wallpaper.
    // /// Smaller values create more tightly packed circles,
    // /// while larger values create spread-out patterns.
    #[arg(short, long, default_value_t = DEFAULT_STEPS)]
    pub(crate) steps: u32,

    #[arg(short, long, default_value_t = DEFAULT_PADDING)]
    pub(crate) padding: u32,

    /// Size of circles generated.
    #[arg(short, long, default_value_t = DEFAULT_DOT_SIZE, value_parser = parse_float)]
    pub(crate) dot_size: f64,

    /// Whether to always generate a full grid of circles.
    #[arg(long, action = ArgAction::SetTrue)]
    pub(crate) full_grid: bool,

    /// Whether to swap width and height values to create a vertical wallpaper.
    #[arg(long, action = ArgAction::SetTrue)]
    pub(crate) swap: bool,

    /// Command to execute.
    #[command(subcommand)]
    pub(crate) command: Commands,
}
