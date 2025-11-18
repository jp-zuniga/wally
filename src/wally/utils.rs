use super::img::Color;

pub(crate) fn blend(src: Color, dst: Color, alpha: f64) -> Color {
    let inv = 1.0 - alpha;

    Color {
        r: alpha * src.r + inv * dst.r,
        g: alpha * src.g + inv * dst.g,
        b: alpha * src.b + inv * dst.b,
    }
}

pub(crate) fn map_float(value: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) -> f64 {
    out_min + (value - in_min) * (out_max - out_min) / (in_max - in_min)
}

pub(crate) fn noise2d(x: f64, y: f64) -> f64 {
    let v = (0.1 * x + 0.13 * y).sin() * (0.17 * x - 0.19 * y).cos();
    0.5 * (v + 1.0)
}

pub(crate) fn parse_float(s: &str) -> Result<f64, String> {
    let value = s
        .parse()
        .map_err(|_| format!("`{s}` is not a valid number."))?;

    if value > 0.0 {
        Ok(value)
    } else {
        Err(format!("`dot_size` must be greater than 0, got {value}."))
    }
}

pub(crate) fn parse_str(s: &str) -> Result<String, String> {
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
