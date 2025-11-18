use clap::ValueEnum;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Color {
    pub(crate) r: f64,
    pub(crate) g: f64,
    pub(crate) b: f64,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(crate) enum ImgFormats {
    Png,
    Jpg,
}

impl Color {
    pub const fn from_u8(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r as f64,
            g: g as f64,
            b: b as f64,
        }
    }

    pub fn to_rgb8(self) -> [u8; 3] {
        [
            self.r.clamp(0.0, 255.0).round() as u8,
            self.g.clamp(0.0, 255.0).round() as u8,
            self.b.clamp(0.0, 255.0).round() as u8,
        ]
    }
}

impl ImgFormats {
    pub fn as_string(&self) -> String {
        match &self {
            ImgFormats::Png => String::from("png"),
            ImgFormats::Jpg => String::from("jpg"),
        }
    }
}
