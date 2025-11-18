use clap::Parser;

use wally::{WallyCLI, run};

fn main() {
    run(WallyCLI::parse());
}
