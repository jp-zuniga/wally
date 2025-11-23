use clap::builder::Styles;
use clap::builder::styling::AnsiColor;

pub(crate) const DEFAULT_WIDTH: u32 = 1920;
pub(crate) const DEFAULT_HEIGHT: u32 = 1080;

pub(crate) const DEFAULT_PADDING: u32 = 200;

pub(crate) const DEFAULT_DOT_SIZE: f32 = 40.0;
pub(crate) const DEFAULT_STEPS: u32 = 80;

pub(crate) const DEFAULT_NAME: &str = "wally";

pub(crate) const WALLY_STYLE: Styles = Styles::styled()
    .header(AnsiColor::Yellow.on_default())
    .literal(AnsiColor::Green.on_default())
    .placeholder(AnsiColor::Blue.on_default());
