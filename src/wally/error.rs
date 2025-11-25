use colored::Colorize;

use clap::Error as ClapError;
use clap::error::{ContextKind, ErrorKind};
use image::ImageError;

use super::consts::{MAX_HEIGHT, MAX_WIDTH, MIN_HEIGHT, MIN_WIDTH};
use super::parse::Dimensions;

#[derive(Debug, Default)]
struct ErrorContext {
    arg: Option<String>,
    value: Option<String>,
    custom_msg: Option<String>,
}

pub(crate) fn exit_with_clap_error<T>(err: ClapError) -> T {
    match err.kind() {
        ErrorKind::DisplayHelp
        | ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
        | ErrorKind::DisplayVersion => {
            println!();
            err.print().expect("what could go wrong writing to stdout?");
            std::process::exit(0);
        },
        _ => {},
    }

    let msg = if is_real_error(err.kind()) {
        let ctx = extract_error_context(&err);

        if let (Some(arg), Some(value)) = (ctx.arg, ctx.value) {
            mk_custom_error_msg(arg, value, ctx.custom_msg)
        } else {
            err.to_string()
        }
    } else {
        err.to_string()
    };

    exit_with_error(err.exit_code(), msg);
}

pub(crate) fn exit_with_error(code: i32, msg: String) -> ! {
    eprintln!();
    eprintln!("{}", "Oh, no!".red().bold());
    eprintln!("{msg}");

    std::process::exit(code);
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

fn is_real_error(kind: ErrorKind) -> bool {
    matches!(kind, ErrorKind::InvalidValue | ErrorKind::ValueValidation)
}

pub(crate) fn mk_big_padding_error_msg(
    padding: &u32,
    height: &u32,
    width: &u32,
) -> String {
    format!(
        "{} {} {} {} {} {} {}",
        "A".blue(),
        "--padding".green().bold().italic(),
        "value of".blue(),
        format!("{}", padding).red().bold(),
        "is too large for a".blue(),
        format!("{}x{}", width, height).green().bold(),
        "image.".blue()
    )
}

pub(crate) fn mk_big_steps_error_msg(steps: &u32, height: &u32, width: &u32) -> String {
    format!(
        "{} {} {} {} {} {} {}",
        "A".blue(),
        "--steps".green().bold().italic(),
        "value of".blue(),
        format!("{}", steps).red().bold(),
        "is too large for a".blue(),
        format!("{}x{}", width, height).green().bold(),
        "image.".blue()
    )
}

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
        "Invalid value {} for {}!\n{} {} {}",
        value.red().bold(),
        arg_str.green().bold(),
        // newline
        "Run".blue().italic(),
        "wally help".green().bold().italic(),
        "to see all available arguments, flags, and subcommands."
            .blue()
            .italic(),
    )
}

fn mk_dimensions_error_msg(dim: Dimensions, value: String) -> String {
    match dim {
        Dimensions::Height => format!(
            "{} is not a valid value for {}!\n{} {} {} {}{}",
            value.red().bold(),
            "--height".green().italic(),
            // newline
            "Height must be an integer between".blue().italic(),
            MIN_HEIGHT.to_string().green().bold().italic(),
            "and".blue().italic(),
            MAX_HEIGHT.to_string().green().bold().italic(),
            ".".blue().italic(),
        ),
        Dimensions::Width => format!(
            "{} is not a valid value for {}!\n{} {} {} {}{}",
            value.red().bold(),
            "--width".green().italic(),
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
        "{} is not a valid value for {}!\n{}",
        value.red().bold(),
        "--dot-size".green().italic(),
        "Dot size must be a positive number.".blue().italic(),
    )
}

fn mk_name_error_msg(value: String) -> String {
    format!(
        "{} is not a valid file name!\n{}",
        value.red().bold(),
        // newline
        "The name must be a valid path, \
         with a file extension matching one of the supported formats."
            .blue()
            .italic(),
    )
}

fn mk_padding_error_msg(value: String) -> String {
    format!(
        "{} is not a valid value for {}!\n{} {}",
        value.red().bold(),
        "--padding".green().bold(),
        // newline
        "--padding".green().bold().italic(),
        "must be a non-negative integer, and less than half the height/width."
            .blue()
            .italic(),
    )
}

fn mk_palette_error_msg(value: String) -> String {
    format!(
        "The palette {} doesn't exist!\n{} {} {}",
        value.red().bold(),
        // newline
        "Run".blue().italic(),
        "wally help".green().bold().italic(),
        "to see all available color palettes.".blue().italic(),
    )
}

fn mk_steps_error_msg(value: String) -> String {
    format!(
        "{} is not a valid value for {}!\n{} {} {} {} {}{}",
        value.red().bold(),
        "--steps".green().italic(),
        // newline
        "--steps".green().bold().italic(),
        "must be a positive integer, and less than".blue().italic(),
        "--height".green().bold().italic(),
        "and".blue().italic(),
        "--width".green().bold().italic(),
        ".".blue().italic(),
    )
}

fn mk_unexpected_error_msg(value: String) -> String {
    format!(
        "Found unexpected argument {}!\n{} {} {}",
        value.red().bold(),
        // newline
        "Run".blue().italic(),
        "wally help".green().bold().italic(),
        "to see all available arguments, flags, and subcommands."
            .blue()
            .italic(),
    )
}

pub(crate) fn mk_write_error_msg(err: ImageError, file: &str) -> String {
    format!(
        "Failed to save your wallpaper at {}!\n{}",
        file.blue().bold(),
        // newline
        err.to_string().red(),
    )
}
