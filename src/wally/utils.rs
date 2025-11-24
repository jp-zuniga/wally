use std::env::current_dir;
use std::ffi::OsString;
use std::path::Path;

use clap::ColorChoice;
use colored::Colorize;

use super::cli::{Commands, WallyCLI};

pub(crate) fn check_bounds(args: &WallyCLI) {
    if args.padding * 2 >= args.width || args.padding * 2 >= args.height {
        print_error(format!(
            "{} {} {} {}{}",
            "The padding".yellow(),
            format!("({})", args.padding).red().bold(),
            "is too large for the given dimensions".yellow(),
            format!("({}x{})", args.width, args.height).red().bold(),
            ".".yellow(),
        ))
    }

    let steps = match args.command {
        Commands::Dots { steps, .. } => steps,
    };

    if steps >= args.height || steps >= args.width {
        print_error(format!(
            "{} {} {} {} {}",
            "A step value of".yellow(),
            format!("{}", steps).red().bold(),
            "is too large for a".yellow(),
            format!("{}x{}", args.width, args.height).red().bold(),
            "image.".yellow()
        ));
    }
}

pub(crate) fn detect_color_choice(argv: &[OsString]) -> ColorChoice {
    let mut flag: Option<bool> = None;

    for arg in argv.iter().skip(1) {
        if arg == "--" {
            break;
        }

        if arg == "--color" {
            flag = Some(true);
        } else if arg == "--no-color" {
            flag = Some(false);
        }
    }

    match flag {
        Some(true) => ColorChoice::Always,
        Some(false) => ColorChoice::Never,
        None => ColorChoice::Auto,
    }
}

pub(crate) fn get_absolute_path(file: &str) -> String {
    let path = Path::new(&file);

    let abs_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        match current_dir() {
            Ok(cwd) => cwd.join(path),
            Err(_) => path.to_path_buf(),
        }
    };

    abs_path.display().to_string()
}

pub(crate) fn print_error(error: String) {
    eprintln!();
    eprintln!("{}", "Oh no!".red().bold());
    eprintln!("{error}");

    std::process::exit(2);
}
