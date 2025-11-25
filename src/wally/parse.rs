use std::path::Path;

use super::consts::{
    MAX_HEIGHT, MAX_WIDTH, MIN_HEIGHT, MIN_PADDING, MIN_STEPS, MIN_WIDTH,
};

#[derive(Clone, Copy, Debug)]
pub(crate) enum Dimensions {
    Height,
    Width,
}

pub(crate) fn parse_file_name(s: &str) -> Result<String, String> {
    let value = s.trim();

    if value.is_empty() {
        return Err("file name must not be empty.".to_string());
    }

    let path = Path::new(value);

    path.file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| "file name must be valid UTF-8 and not empty.".to_string())?;

    Ok(value.to_string())
}

// ------------------------------------------------------------------------------------

fn pretty_parse_u32(label: &str, limit: i32, s: &str) -> Result<u32, String> {
    let value = s
        .parse::<i32>()
        .map_err(|_| format!("`{s}` must be a positive integer."))?;

    if value < limit {
        return Err(format!(
            "`{label}` must be greater than or equal to {limit}, got {value}."
        ));
    }

    Ok(value as u32)
}

pub(crate) fn parse_padding(s: &str) -> Result<u32, String> {
    pretty_parse_u32("padding", MIN_PADDING, s)
}

pub(crate) fn parse_steps(s: &str) -> Result<u32, String> {
    pretty_parse_u32("steps", MIN_STEPS, s)
}

// ------------------------------------------------------------------------------------

fn parse_positive_float(label: &str, s: &str) -> Result<f32, String> {
    let value = s
        .parse()
        .map_err(|_| format!("`{s}` is not a valid number."))?;

    if value <= 0.0 {
        return Err(format!("`{label}` must be greater than 0, got {value}."));
    }

    Ok(value)
}

pub(crate) fn parse_dot_size(s: &str) -> Result<f32, String> {
    parse_positive_float("dot_size", s)
}

// ------------------------------------------------------------------------------------

fn parse_dimensions(s: &str, dim: Dimensions) -> Result<u32, String> {
    let value = s
        .parse()
        .map_err(|_| format!("`{s}` is not a valid number."))?;

    match dim {
        Dimensions::Height => {
            if !(MIN_HEIGHT..=MAX_HEIGHT).contains(&value) {
                return Err(format!(
                    "`height` must be between {MIN_HEIGHT} and {MAX_HEIGHT}.",
                ));
            }
        },
        Dimensions::Width => {
            if !(MIN_WIDTH..=MAX_WIDTH).contains(&value) {
                return Err(format!(
                    "`width` must be between {MIN_WIDTH} and {MAX_WIDTH}.",
                ));
            }
        },
    }

    Ok(value)
}

pub(crate) fn parse_height(s: &str) -> Result<u32, String> {
    parse_dimensions(s, Dimensions::Height)
}

pub(crate) fn parse_width(s: &str) -> Result<u32, String> {
    parse_dimensions(s, Dimensions::Width)
}
