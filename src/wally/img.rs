use clap::ValueEnum;
use image::{ExtendedColorType, ImageFormat, save_buffer_with_format};
use rayon::{
    iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator},
    slice::ParallelSliceMut,
};

pub(crate) fn write_img(
    file: String,
    format: WallFormats,
    width: u32,
    height: u32,
    pixels: &[Color],
) {
    let mut buf = vec![0u8; pixels.len() * 3];

    buf.par_chunks_mut(3)
        .zip(pixels.par_iter())
        .for_each(|(chunk, p)| {
            let [r, g, b] = p.to_rgb();
            chunk[0] = r;
            chunk[1] = g;
            chunk[2] = b;
        });

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

    #[inline]
    pub fn to_rgb(self) -> [u8; 3] {
        [self.r as u8, self.g as u8, self.b as u8]
    }
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(crate) enum WallFormats {
    Png,
    Jpg,
}

impl WallFormats {
    pub fn as_str(&self) -> &'static str {
        match &self {
            WallFormats::Png => "png",
            WallFormats::Jpg => "jpg",
        }
    }

    pub fn as_image_format(&self) -> ImageFormat {
        match self {
            WallFormats::Png => ImageFormat::Png,
            WallFormats::Jpg => ImageFormat::Jpeg,
        }
    }
}
