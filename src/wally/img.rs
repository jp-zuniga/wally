use clap::ValueEnum;
use image::{Rgb, RgbImage};

pub(crate) fn write_img(file: String, width: &u32, height: &u32, pixels: &[Color]) {
    let mut img = RgbImage::new(*width, *height);

    for y in 0..*height {
        for x in 0..*width {
            let idx = (*width * y + x) as usize;
            let p = pixels[idx];

            let [r, g, b] = p.to_rgb8();

            img.put_pixel(x, y, Rgb([r, g, b]));
        }
    }

    img.save(&file).expect("Failed to save `{file}`!");
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

    pub fn to_rgb8(self) -> [u8; 3] {
        [
            self.r.clamp(0.0, 255.0).round() as u8,
            self.g.clamp(0.0, 255.0).round() as u8,
            self.b.clamp(0.0, 255.0).round() as u8,
        ]
    }
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(crate) enum ImgFormats {
    Png,
    Jpg,
}

impl ImgFormats {
    pub fn as_string(&self) -> String {
        match &self {
            ImgFormats::Png => String::from("png"),
            ImgFormats::Jpg => String::from("jpg"),
        }
    }
}
