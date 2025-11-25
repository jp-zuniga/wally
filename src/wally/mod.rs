use std::ffi::OsString;

use clap::{CommandFactory, FromArgMatches};

pub(crate) mod cli;
mod consts;
mod dots;
mod draw;
mod error;
mod img;
mod noise;
mod parse;
mod term;
mod themes;
mod utils;

use cli::{WallyCLI, WallyCommands};
use dots::mk_dots;
use error::exit_with_clap_error;
use term::set_color_output;
use utils::{check_bounds, detect_color_choice};

pub fn init_cli() -> WallyCLI {
    let argv: Vec<OsString> = std::env::args_os().collect();

    let matches = WallyCLI::command()
        .color(detect_color_choice(&argv))
        .try_get_matches_from(&argv)
        .unwrap_or_else(exit_with_clap_error);

    let args =
        WallyCLI::from_arg_matches(&matches).unwrap_or_else(exit_with_clap_error);

    set_color_output(args.colorize());

    args
}

pub fn run_cli(mut args: WallyCLI) {
    if args.swap {
        std::mem::swap(&mut args.width, &mut args.height);
    }

    match args.command {
        WallyCommands::Dots { dot_size, steps } => {
            check_bounds(&args, steps);
            mk_dots(&args, dot_size, steps, &*args.mk_palette())
        },
        WallyCommands::Themes => {
            themes::print_palettes();
        },
    };
}
