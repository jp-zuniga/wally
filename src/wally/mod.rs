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
use themes::rosepine::RosePineFlavor;

pub fn run(mut args: WallyCLI) {
    if args.swap {
        swap(&mut args.width, &mut args.height);
    };

    match args.command {
        Commands::Dots { dot_size, steps } => match &args.palette {
            Themes::CatppuccinFrappe => {
                mk_dots(&args, CatppuccinFlavor::frappe(), dot_size, steps)
            }
            Themes::CatppuccinLatte => {
                mk_dots(&args, CatppuccinFlavor::latte(), dot_size, steps)
            }
            Themes::CatppuccinMacchiato => {
                mk_dots(&args, CatppuccinFlavor::macchiato(), dot_size, steps)
            }
            Themes::CatppuccinMocha => {
                mk_dots(&args, CatppuccinFlavor::mocha(), dot_size, steps)
            }
            Themes::RosePineDawn => {
                mk_dots(&args, RosePineFlavor::dawn(), dot_size, steps)
            }
            Themes::RosePineDefault => {
                mk_dots(&args, RosePineFlavor::default(), dot_size, steps)
            }
            Themes::RosePineMoon => {
                mk_dots(&args, RosePineFlavor::moon(), dot_size, steps)
            }
        },
    };
}
