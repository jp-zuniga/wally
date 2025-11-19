pub(crate) mod cli;
mod draw;
mod img;
mod noise;
mod palettes;
mod utils;
mod wall;

use std::mem::swap;

use cli::{Commands, WallyCLI};
use palettes::catppuccin::CatppuccinFlavor;
use palettes::rose_pine::RosePineFlavor;
use palettes::theme::Theme;
use wall::mk_wall;

pub fn run(mut args: WallyCLI) {
    if args.swap {
        swap(&mut args.width, &mut args.height);
    };

    match &args.command {
        Commands::Dots { palette } => match palette {
            Theme::CatppuccinFrappe => mk_wall(&args, CatppuccinFlavor::frappe()),
            Theme::CatppuccinLatte => mk_wall(&args, CatppuccinFlavor::latte()),
            Theme::CatppuccinMacchiato => mk_wall(&args, CatppuccinFlavor::macchiato()),
            Theme::CatppuccinMocha => mk_wall(&args, CatppuccinFlavor::mocha()),
            Theme::RosePineDawn => mk_wall(&args, RosePineFlavor::dawn()),
            Theme::RosePineDefault => mk_wall(&args, RosePineFlavor::default()),
            Theme::RosePineMoon => mk_wall(&args, RosePineFlavor::moon()),
        },
    };
}
