use clap::{ArgAction, crate_authors, crate_version};
use clap::{Parser, Subcommand};

use crate::wally::consts::{CLI_STYLE, DEFAULT_DOT_SIZE, DEFAULT_STEPS};

pub(crate) mod args;
pub(crate) mod error;
pub(crate) mod parse;
pub(crate) mod term;

use args::WallArgs;
use parse::{parse_dot_size, parse_steps};

#[derive(Clone, Debug, Subcommand)]
pub(crate) enum WallyCommands {
    /// Create a wallpaper of randomly-generated dots.
    Dots {
        /// Radius of generated dots.
        #[arg(
            short,
            long,
            help_heading = "Wallpaper Options",
            default_value_t = DEFAULT_DOT_SIZE,
            value_parser = parse_dot_size,
        )]
        dot_size: f32,

        /// Density of generated dots.
        #[arg(
            short,
            long,
            help_heading = "Wallpaper Options",
            default_value_t = DEFAULT_STEPS,
            value_parser = parse_steps,
        )]
        steps: u32,

        #[command(flatten)]
        wall_args: WallArgs,
    },

    /// List available color palettes.
    Themes,
}

#[derive(Parser, Debug)]
#[command(
    arg_required_else_help = true,
    author = crate_authors!(),
    version = crate_version!(),
    about = None,
    long_about = None,
    styles = CLI_STYLE,
)]
pub struct WallyCli {
    /// Force color output.
    #[arg(
        long,
        help_heading = "CLI Options",
        global = true,
        conflicts_with = "no_color",
        action = ArgAction::SetTrue,
    )]
    pub(crate) color: bool,

    /// Disable color output.
    #[arg(
        long,
        help_heading = "CLI Options",
        global = true,
        conflicts_with = "color",
        action = ArgAction::SetTrue,
    )]
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
}
