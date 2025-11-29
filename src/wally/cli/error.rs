use colored::Colorize;

use clap::error::{ContextKind, ErrorKind};
use clap::{Error as ClapError, crate_version};
use image::ImageError;

use super::parse::Dimensions;
use crate::wally::consts::{MAX_HEIGHT, MAX_WIDTH, MIN_HEIGHT, MIN_STEPS, MIN_WIDTH};

#[derive(Debug, Default)]
struct ErrorContext {
    arg: Option<String>,
    value: Option<String>,
    custom_msg: Option<String>,
}

pub(crate) fn exit_with_clap_error<T>(err: ClapError) -> T {
    match err.kind() {
        ErrorKind::DisplayHelp
        | ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand => {
            println!();
            err.print().expect("what could go wrong writing to stdout?");
            std::process::exit(0);
        },
        ErrorKind::DisplayVersion => {
            println!();
            print!("{} {}", "wally".purple(), crate_version!().purple().bold());
            std::process::exit(0);
        },
        _ => {},
    }

    let msg = format_clap_error(&err);
    exit_with_error(err.exit_code(), msg);
}

pub(crate) fn exit_with_error(code: i32, msg: String) -> ! {
    eprintln!();
    eprintln!("{}", "Oh, no!".red().bold());
    eprintln!("{msg}");
    std::process::exit(code);
}

pub(crate) fn print_warning(msg: String) {
    eprintln!();
    eprintln!("{}", "Heads up!".yellow().bold());
    eprintln!("{msg}");
}

// -------------------------------------------------------------------------------------

fn format_clap_error(err: &ClapError) -> String {
    let ctx = extract_error_context(err);

    match err.kind() {
        ErrorKind::InvalidValue | ErrorKind::ValueValidation => {
            if let (Some(arg), Some(value)) = (ctx.arg, ctx.value) {
                mk_custom_error_msg(arg, value, ctx.custom_msg)
            } else if let Some(msg) = ctx.custom_msg {
                mk_unexpected_error_msg(msg)
            } else {
                mk_generic_parse_error()
            }
        },
        ErrorKind::UnknownArgument => {
            if let Some(arg) = ctx.arg {
                mk_unexpected_error_msg(arg)
            } else {
                mk_generic_parse_error()
            }
        },
        ErrorKind::InvalidSubcommand => {
            if let Some(sub) = ctx.arg {
                mk_invalid_subcommand_msg(sub)
            } else {
                mk_generic_parse_error()
            }
        },
        ErrorKind::MissingRequiredArgument => {
            if let Some(arg) = ctx.arg {
                mk_missing_arg_msg(arg)
            } else {
                mk_generic_parse_error()
            }
        },
        ErrorKind::MissingSubcommand => mk_missing_subcommand_msg(),
        ErrorKind::ArgumentConflict => mk_argument_conflict_msg(),
        _ => mk_fallback_error(err),
    }
}

fn extract_error_context(err: &ClapError) -> ErrorContext {
    let mut ctx = ErrorContext::default();

    for (kind, s) in err.context() {
        match kind {
            ContextKind::InvalidArg | ContextKind::InvalidSubcommand => {
                ctx.arg = Some(s.to_string());
            },
            ContextKind::InvalidValue => {
                ctx.value = Some(s.to_string());
            },
            ContextKind::Custom => {
                ctx.custom_msg = Some(s.to_string());
            },
            _ => {},
        }
    }

    ctx
}

// -------------------------------------------------------------------------------------

fn mk_generic_parse_error() -> String {
    format!(
        "{}\n{} {} {}",
        "There was an error parsing some of the arguments passed.".yellow(),
        "Run".blue().italic(),
        "wally help".green().bold().italic(),
        "to see usage information.".blue().italic(),
    )
}

fn mk_fallback_error(err: &ClapError) -> String {
    let kind = format!("{:?}", err.kind()).to_lowercase().replace('_', " ");
    format!(
        "{} {}\n{} {} {}",
        "Something went wrong while parsing the command line:".yellow(),
        kind.red().bold(),
        "Run".blue().italic(),
        "wally help".green().bold().italic(),
        "to see usage information.".blue().italic(),
    )
}

// -------------------------------------------------------------------------------------

pub(crate) fn mk_big_padding_error_msg(
    padding: &u32,
    height: &u32,
    width: &u32,
) -> String {
    format!(
        "{} {} {} {} {} {} {}",
        "A".purple(),
        "--padding".green().bold(),
        "value of".purple(),
        format!("{}", padding).red().bold(),
        "is too large for a".purple(),
        format!("{}x{}", width, height).green().bold(),
        "image.".purple()
    )
}

pub(crate) fn mk_big_steps_error_msg(steps: &u32, height: &u32, width: &u32) -> String {
    format!(
        "{} {} {} {} {} {} {}",
        "A".purple(),
        "--steps".green().bold(),
        "value of".purple(),
        format!("{}", steps).red().bold(),
        "is too large for a".purple(),
        format!("{}x{}", width, height).green().bold(),
        "image.".purple()
    )
}

pub(crate) fn mk_dir_create_error_msg(dir: &str, err: &std::io::Error) -> String {
    format!(
        "{} {}{}\n{}",
        "Couldn't create directory".purple(),
        dir.blue().bold(),
        ".".purple(),
        err.to_string().red(),
    )
}

pub(crate) fn mk_unknown_extension_msg(ext_str: &str, target_ext: &str) -> String {
    format!(
        "{} {} {}\n{} {}{}",
        "The extension".purple(),
        format!(".{}", ext_str).red().bold(),
        "is not supported.".purple(),
        "Wally will save your wallpaper as a".blue().italic(),
        target_ext.blue().bold().italic(),
        " instead.".blue().italic(),
    )
}

// -------------------------------------------------------------------------------------

pub(crate) fn mk_custom_error_msg(
    arg: String,
    value: String,
    custom_msg: Option<String>,
) -> String {
    let arg_str = arg.as_str();

    if arg_str.contains("--palette") {
        return mk_palette_error_msg(value);
    }

    if arg_str.contains("--name") {
        return mk_name_error_msg(value);
    }

    if arg_str.contains("--width") || arg_str.contains("<WIDTH>") {
        return mk_dimensions_error_msg(Dimensions::Width, value);
    }

    if arg_str.contains("--height") || arg_str.contains("<HEIGHT>") {
        return mk_dimensions_error_msg(Dimensions::Height, value);
    }

    if arg_str.contains("--padding") {
        return mk_padding_error_msg(value);
    }

    if arg_str.contains("--dot-size") || arg_str.contains("<DOT_SIZE>") {
        return mk_dot_size_error_msg(value);
    }

    if arg_str.contains("--steps") || arg_str.contains("<STEPS>") {
        return mk_steps_error_msg(value);
    }

    if let Some(msg) = custom_msg {
        return mk_unexpected_error_msg(msg);
    }

    format!(
        "{} {} {}{}\n{} {} {}",
        value.red().bold(),
        "is not a valid value for".purple(),
        remove_arg_placeholder(arg_str).green().bold(),
        "!".purple(),
        "Run".blue().italic(),
        "wally help".green().bold().italic(),
        "to see all valid arguments and examples.".blue().italic(),
    )
}

fn mk_dimensions_error_msg(dim: Dimensions, value: String) -> String {
    match dim {
        Dimensions::Height => format!(
            "{} {} {}{}\n{} {} {} {}{}",
            value.red().bold(),
            "is not a valid value for".purple(),
            "--height".green().bold(),
            "!".purple(),
            // newline
            "Height must be an integer between".blue().italic(),
            MIN_HEIGHT.to_string().green().bold().italic(),
            "and".blue().italic(),
            MAX_HEIGHT.to_string().green().bold().italic(),
            ".".blue().italic(),
        ),
        Dimensions::Width => format!(
            "{} {} {}{}\n{} {} {} {}{}",
            value.red().bold(),
            "is not a valid value for".purple(),
            "--width".green().bold(),
            "!".purple(),
            // newline
            "Width must be an integer between".blue().italic(),
            MIN_WIDTH.to_string().green().bold().italic(),
            "and".blue().italic(),
            MAX_WIDTH.to_string().green().bold().italic(),
            ".".blue().italic(),
        ),
    }
}

fn mk_dot_size_error_msg(value: String) -> String {
    format!(
        "{} {} {}{}\n{} {}",
        value.red().bold(),
        "is not a valid value for".purple(),
        "--dot-size".green().bold(),
        "!".purple(),
        // newline
        "--dot-size".green().bold(),
        "must be a positive number.".blue().italic(),
    )
}

fn mk_name_error_msg(value: String) -> String {
    format!(
        "{} {}\n{}",
        value.red().bold(),
        "is not a valid file name!".purple(),
        "The name must be a valid path, \
         with a file extension matching one of the supported formats."
            .blue()
            .italic(),
    )
}

fn mk_padding_error_msg(value: String) -> String {
    format!(
        "{} {} {}{}\n{} {}",
        value.red().bold(),
        "is not a valid value for".purple(),
        "--padding".green().bold(),
        "!".purple(),
        // newline
        "--padding".green().bold(),
        "must be a non-negative integer, and less than half the height and width."
            .blue()
            .italic(),
    )
}

fn mk_palette_error_msg(value: String) -> String {
    format!(
        "{} {} {}\n{} {} {}",
        "The palette".purple(),
        value.red().bold(),
        "doesn't exist!".purple(),
        "Run".blue().italic(),
        "wally themes".green().bold().italic(),
        "to see all available color palettes.".blue().italic(),
    )
}

fn mk_steps_error_msg(value: String) -> String {
    format!(
        "{} {} {}{}\n{} {} {} {} {} {} {}{}",
        value.red().bold(),
        "is not a valid value for".purple(),
        "--steps".green().bold(),
        "!".purple(),
        // newline
        "--steps".green().bold().italic(),
        "must be a positive integer greater than or equal to"
            .blue()
            .italic(),
        format!("{}", MIN_STEPS).blue().bold().italic(),
        "and less than".blue().italic(),
        "--height".green().bold().italic(),
        "and".blue().italic(),
        "--width".green().bold().italic(),
        ".".blue().italic(),
    )
}

// -------------------------------------------------------------------------------------

fn mk_invalid_subcommand_msg(sub: String) -> String {
    format!(
        "{} {}{}\n{} {} {}",
        "Unknown subcommand".purple(),
        sub.red().bold(),
        "!".purple(),
        "Run".blue().italic(),
        "wally help".green().bold().italic(),
        "to see all available subcommands.".blue().italic(),
    )
}

fn mk_missing_arg_msg(arg: String) -> String {
    let pretty = arg.trim_matches(|c| c == '<' || c == '>');
    format!(
        "{} {}{}\n{} {} {}",
        "Missing required value for".purple(),
        pretty.green().bold(),
        "!".purple(),
        "Run".blue().italic(),
        "wally help".green().bold().italic(),
        "to see all available arguments, flags, and subcommands."
            .blue()
            .italic(),
    )
}

fn mk_missing_subcommand_msg() -> String {
    format!(
        "{}\n{} {} {}",
        "You might have forgotten a subcommand!".purple(),
        "Try".blue().italic(),
        "wally help <command>".green().bold().italic(),
        "to see what you missed.".blue().italic(),
    )
}

fn mk_argument_conflict_msg() -> String {
    format!(
        "{} {} {} {} {}\n{}",
        "You can't use".purple(),
        "--color".green().bold(),
        "and".purple(),
        "--no-color".green().bold(),
        "at the same time.".purple(),
        "Select one to be explicit, or omit both for automatic detection."
            .blue()
            .italic(),
    )
}

fn mk_unexpected_error_msg(value: String) -> String {
    format!(
        "{} {}{}\n{} {} {}",
        "Found unexpected argument".purple(),
        value.red().bold(),
        "!".purple(),
        "Run".blue().italic(),
        "wally help".green().bold().italic(),
        "to see all available arguments, flags, and subcommands."
            .blue()
            .italic(),
    )
}

// -------------------------------------------------------------------------------------

pub(crate) fn mk_write_error_msg(err: ImageError, file: &str) -> String {
    format!(
        "{} {}{}\n{}",
        "Failed to save your wallpaper at".purple(),
        file.blue().bold(),
        "!".purple(),
        err.to_string().red(),
    )
}

// -------------------------------------------------------------------------------------

fn remove_arg_placeholder(arg: &str) -> &str {
    arg.get(
        0..arg
            .find("<")
            .expect("arg should come in the form '--arg <ARG>'"),
    )
    .unwrap_or(arg)
    .trim()
}
