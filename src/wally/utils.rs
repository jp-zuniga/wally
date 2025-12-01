use std::ffi::OsString;
use std::path::{Path, PathBuf};

use clap::ColorChoice;

use super::cli::args::WallArgs;
use super::cli::error::{
    exit_with_error, mk_big_padding_error_msg, mk_big_steps_error_msg,
    mk_unknown_extension_msg, print_warning,
};
use super::img::WallFormats;

pub(crate) fn check_bounds(args: &WallArgs, steps: u32) {
    if args.padding * 2 >= args.width || args.padding * 2 >= args.height {
        exit_with_error(
            1,
            &mk_big_padding_error_msg(args.padding, args.height, args.width),
        )
    }

    if steps >= args.height || steps >= args.width {
        exit_with_error(1, &mk_big_steps_error_msg(steps, args.height, args.width));
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
    let path = Path::new(file);

    let abs_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        match std::env::current_dir() {
            Ok(cwd) => cwd.join(path),
            Err(_) => path.to_path_buf(),
        }
    };

    abs_path.display().to_string()
}

pub(crate) fn resolve_output_file(raw_name: &str, format: WallFormats) -> String {
    let mut path = PathBuf::from(raw_name);

    let extension = path.extension();

    if extension.is_none() {
        path.set_extension(format.as_str());
        return path.to_string_lossy().into_owned();
    }

    let ext_str = extension
        .unwrap()
        .to_string_lossy()
        .to_string()
        .to_ascii_lowercase();

    if WallFormats::from_str(&ext_str).is_err() {
        print_warning(&mk_unknown_extension_msg(&ext_str, format.as_str()));
        path.set_extension(WallFormats::default().as_str());
    }

    path.to_string_lossy().to_string()
}
