use clap::builder::Styles;
use clap::builder::styling::AnsiColor;

pub(crate) const BASE_ALPHA: f32 = 128.0 / 255.0;

pub(crate) const DEFAULT_WIDTH: u32 = 1920;
pub(crate) const MAX_WIDTH: u32 = 7680;
pub(crate) const MIN_WIDTH: u32 = 480;

pub(crate) const DEFAULT_HEIGHT: u32 = 1080;
pub(crate) const MAX_HEIGHT: u32 = 4320;
pub(crate) const MIN_HEIGHT: u32 = 270;

pub(crate) const DEFAULT_PADDING: u32 = 150;

pub(crate) const DEFAULT_DOT_SIZE: f32 = 40.0;
pub(crate) const DEFAULT_STEPS: u32 = 80;

pub(crate) const DEFAULT_NAME: &str = "wally";

pub(crate) const WALLY_STYLE: Styles = Styles::styled()
    .header(AnsiColor::Yellow.on_default())
    .literal(AnsiColor::Green.on_default())
    .placeholder(AnsiColor::Blue.on_default());
