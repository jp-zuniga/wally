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
use themes::Themes;
use themes::catppuccin::CatppuccinFlavor;
use themes::dracula::DraculaFlavor;
use themes::gruvbox::GruvboxFlavor;
use themes::rosepine::RosePineFlavor;
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

    let (dot_size, steps) = match args.command {
        Commands::Dots { dot_size, steps } => (dot_size, steps),
    };

    match args.palette {
        Themes::Alucard => mk_dots(&args, dot_size, steps, DraculaFlavor::alucard()),
        Themes::Dracula => mk_dots(&args, dot_size, steps, DraculaFlavor::default()),
        Themes::CatppuccinFrappe => {
            mk_dots(&args, dot_size, steps, CatppuccinFlavor::frappe())
        }
        Themes::CatppuccinLatte => {
            mk_dots(&args, dot_size, steps, CatppuccinFlavor::latte())
        }
        Themes::CatppuccinMacchiato => {
            mk_dots(&args, dot_size, steps, CatppuccinFlavor::macchiato())
        }
        Themes::CatppuccinMocha => {
            mk_dots(&args, dot_size, steps, CatppuccinFlavor::mocha())
        }
        Themes::GruvboxDark => mk_dots(&args, dot_size, steps, GruvboxFlavor::dark()),
        Themes::GruvboxLight => mk_dots(&args, dot_size, steps, GruvboxFlavor::light()),
        Themes::RosePineDawn => mk_dots(&args, dot_size, steps, RosePineFlavor::dawn()),
        Themes::RosePineDefault => {
            mk_dots(&args, dot_size, steps, RosePineFlavor::default())
        }
        Themes::RosePineMoon => mk_dots(&args, dot_size, steps, RosePineFlavor::moon()),
    };
}
