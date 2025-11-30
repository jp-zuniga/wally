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

pub(crate) fn run_cli(cli: WallyCli) {
    match cli.command {
        WallyCommands::Dots {
            mut wall_args,
            dot_size,
            steps,
        } => {
            if wall_args.swap {
                std::mem::swap(&mut wall_args.width, &mut wall_args.height);
            }

            check_bounds(&wall_args, steps);
            mk_dots(&wall_args, dot_size, steps, wall_args.mk_palette())
        },
        WallyCommands::Themes => print_palettes(),
    };
}
