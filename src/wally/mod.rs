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
use themes::catppuccin::CatppuccinFlavor;
use themes::rose_pine::RosePineFlavor;
use themes::Themes;

pub fn run(mut args: WallyCLI) {
    if args.swap {
        swap(&mut args.width, &mut args.height);
    };

    match &args.command {
        Commands::Dots { palette } => match palette {
            Themes::CatppuccinFrappe => mk_dots(&args, CatppuccinFlavor::frappe()),
            Themes::CatppuccinLatte => mk_dots(&args, CatppuccinFlavor::latte()),
            Themes::CatppuccinMacchiato => mk_dots(&args, CatppuccinFlavor::macchiato()),
            Themes::CatppuccinMocha => mk_dots(&args, CatppuccinFlavor::mocha()),
            Themes::RosePineDawn => mk_dots(&args, RosePineFlavor::dawn()),
            Themes::RosePineDefault => mk_dots(&args, RosePineFlavor::default()),
            Themes::RosePineMoon => mk_dots(&args, RosePineFlavor::moon()),
        },
    };
}
