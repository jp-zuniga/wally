pub(crate) mod cli;
mod draw;
mod flavors;
mod img;
mod palettes;
mod utils;
mod wall;

use std::mem::swap;

use cli::{Commands, WallyCLI};
use palettes::PALETTES;
use wall::mk_wall;

pub fn run(mut args: WallyCLI) {
    match &args.command {
        Commands::Dots { palette } => {
            if args.swap {
                swap(&mut args.width, &mut args.height);
            };

            mk_wall(&args, &PALETTES.get_flavor(palette));
        }
    }
}
