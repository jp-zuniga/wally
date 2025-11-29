use std::ffi::OsString;

use clap::{CommandFactory, FromArgMatches};

mod cli;
mod consts;
mod dots;
mod draw;
mod img;
mod noise;
mod themes;
mod utils;

use cli::error::exit_with_clap_error;
use cli::term::set_color_output;
use cli::{WallyCli, WallyCommands};
use dots::mk_dots;
use themes::print_palettes;
use utils::{check_bounds, detect_color_choice};

pub(crate) fn init_cli() -> WallyCli {
    let argv: Vec<OsString> = std::env::args_os().collect();

    let matches = WallyCli::command()
        .color(detect_color_choice(&argv))
        .try_get_matches_from(&argv)
        .unwrap_or_else(exit_with_clap_error);

    let args =
        WallyCli::from_arg_matches(&matches).unwrap_or_else(exit_with_clap_error);

    set_color_output(args.colorize());

    args
}

pub(crate) fn run_cli(mut args: WallyCli) {
    if args.swap {
        std::mem::swap(&mut args.width, &mut args.height);
    }

    match args.command {
        WallyCommands::Dots { dot_size, steps } => {
            check_bounds(&args, steps);
            mk_dots(&args, dot_size, steps, args.mk_palette())
        },
        WallyCommands::Themes => {
            print_palettes();
        },
    };
}
