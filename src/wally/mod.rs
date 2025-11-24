use std::mem::swap;

use clap::{CommandFactory, FromArgMatches};

pub(crate) mod cli;
mod consts;
mod dots;
mod draw;
mod img;
mod noise;
mod parse;
mod term;
mod themes;
mod utils;

use cli::{Commands, WallyCLI};
use dots::mk_dots;
use term::set_color_output;
use utils::detect_color_choice;

pub fn init_cli() -> WallyCLI {
    let argv: Vec<std::ffi::OsString> = std::env::args_os().collect();
    let clap_color = detect_color_choice(&argv);

    let cmd = WallyCLI::command().color(clap_color);

    let matches = cmd.get_matches_from(argv);
    let args = WallyCLI::from_arg_matches(&matches).unwrap_or_else(|e| e.exit());

    set_color_output(args.colorize());

    args
}

pub fn run_cli(mut args: WallyCLI) {
    if args.swap {
        swap(&mut args.width, &mut args.height);
    }

    match args.command {
        Commands::Dots { dot_size, steps } => {
            mk_dots(&args, dot_size, steps, &*args.mk_palette())
        }
    };
}
