pub(crate) mod cli;
mod consts;
mod dots;
mod draw;
mod img;
mod noise;
mod palettes;
mod utils;

use std::mem::swap;

use cli::{Commands, WallyCLI};
use dots::mk_dots;
use palettes::catppuccin::CatppuccinFlavor;
use palettes::rose_pine::RosePineFlavor;
use palettes::theme::Theme;

pub fn run(mut args: WallyCLI) {
    if args.swap {
        swap(&mut args.width, &mut args.height);
    };

    match &args.command {
        Commands::Dots { palette } => match palette {
            Theme::CatppuccinFrappe => mk_dots(&args, CatppuccinFlavor::frappe()),
            Theme::CatppuccinLatte => mk_dots(&args, CatppuccinFlavor::latte()),
            Theme::CatppuccinMacchiato => mk_dots(&args, CatppuccinFlavor::macchiato()),
            Theme::CatppuccinMocha => mk_dots(&args, CatppuccinFlavor::mocha()),
            Theme::RosePineDawn => mk_dots(&args, RosePineFlavor::dawn()),
            Theme::RosePineDefault => mk_dots(&args, RosePineFlavor::default()),
            Theme::RosePineMoon => mk_dots(&args, RosePineFlavor::moon()),
        },
    };
}
