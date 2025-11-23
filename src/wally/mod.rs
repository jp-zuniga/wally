pub(crate) mod cli;
mod consts;
mod dots;
mod draw;
mod img;
mod noise;
mod themes;
mod utils;

use std::mem::swap;

use cli::{Commands, WallyCLI};
use dots::mk_dots;
use themes::Themes;
use themes::catppuccin::CatppuccinFlavor;
use themes::dracula::DraculaFlavor;
use themes::rosepine::RosePineFlavor;

pub fn run(mut args: WallyCLI) {
    if args.swap {
        swap(&mut args.width, &mut args.height);
    }

    let (dot_size, steps) = match args.command {
        Commands::Dots { dot_size, steps } => (dot_size, steps),
    };

    match args.palette {
        Themes::Dracula => mk_dots(&args, dot_size, steps, DraculaFlavor::default()),
        Themes::Alucard => mk_dots(&args, dot_size, steps, DraculaFlavor::alucard()),
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
        Themes::RosePineDawn => mk_dots(&args, dot_size, steps, RosePineFlavor::dawn()),
        Themes::RosePineDefault => {
            mk_dots(&args, dot_size, steps, RosePineFlavor::default())
        }
        Themes::RosePineMoon => mk_dots(&args, dot_size, steps, RosePineFlavor::moon()),
    };
}
