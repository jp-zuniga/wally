use super::img::Color;

pub(crate) fn blend(src: Color, dst: Color, alpha: f32) -> Color {
    let inv = 1.0 - alpha;

    Color {
        r: alpha * src.r + inv * dst.r,
        g: alpha * src.g + inv * dst.g,
        b: alpha * src.b + inv * dst.b,
    }
}

pub(crate) fn parse_float(s: &str) -> Result<f32, String> {
    let value = s
        .parse()
        .map_err(|_| format!("`{s}` is not a valid number."))?;

    if value > 0.0 {
        Ok(value)
    } else {
        Err(format!("`dot_size` must be greater than 0, got {value}."))
    }
}

pub(crate) fn parse_file_arg(s: &str) -> Result<String, String> {
    let value = s.to_string();
    let parts: Vec<&str> = value.split_terminator('.').collect();

    if parts.len() == 1 {
        Ok(value)
    } else {
        Err(String::from(
            "must specify file format with `--format`, not a file extension.",
        ))
    }
}
