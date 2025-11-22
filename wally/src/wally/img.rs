use clap::ValueEnum;
use image::{ExtendedColorType, ImageFormat, save_buffer_with_format};

pub(crate) fn write_img(
    file: String,
    format: WallFormats,
    width: u32,
    height: u32,
    pixels: &[Color],
) {
    let mut buf = Vec::with_capacity((width * height * 3) as usize);

    for p in pixels {
        let [r, g, b] = p.to_rgb();
        buf.push(r);
        buf.push(g);
        buf.push(b);
    }

    save_buffer_with_format(
        &file,
        &buf,
        width,
        height,
        ExtendedColorType::Rgb8,
        format.as_image_format(),
    )
    .unwrap_or_else(|e| panic!("Failed to save `{file}`: {e}"));

    println!("Successfully wrote `{file}`!");
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Color {
    pub(crate) r: f32,
    pub(crate) g: f32,
    pub(crate) b: f32,
}

impl Color {
    pub const fn from_u8(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r as f32,
            g: g as f32,
            b: b as f32,
        }
    }

    pub fn to_rgb(self) -> [u8; 3] {
        [
            self.r.clamp(0.0, 255.0).round() as u8,
            self.g.clamp(0.0, 255.0).round() as u8,
            self.b.clamp(0.0, 255.0).round() as u8,
        ]
    }
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(crate) enum WallFormats {
    Png,
    Jpg,
}

impl WallFormats {
    pub fn as_string(&self) -> String {
        match &self {
            WallFormats::Png => String::from("png"),
            WallFormats::Jpg => String::from("jpg"),
        }
    }

    pub fn as_image_format(&self) -> ImageFormat {
        match self {
            WallFormats::Png => ImageFormat::Png,
            WallFormats::Jpg => ImageFormat::Jpeg,
        }
    }
}
