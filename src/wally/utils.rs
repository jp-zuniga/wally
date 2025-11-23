use std::env::current_dir;
use std::ffi::OsString;
use std::path::Path;

use clap::ColorChoice;

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
