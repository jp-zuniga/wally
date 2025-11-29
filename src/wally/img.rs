use clap::ValueEnum;
use colored::Colorize;
use image::{ExtendedColorType, ImageFormat, save_buffer_with_format};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use rayon::slice::ParallelSliceMut;

use super::cli::error::{exit_with_error, mk_dir_create_error_msg, mk_write_error_msg};
use super::utils::get_absolute_path;

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
    pub(crate) fn to_rgb(self) -> [u8; 3] {
        [self.r as u8, self.g as u8, self.b as u8]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, ValueEnum)]
pub(crate) enum WallFormats {
    Png,
    Jpg,
}

impl WallFormats {
    pub(crate) fn as_str(&self) -> &'static str {
        match &self {
            Self::Jpg => "jpg",
            Self::Png => "png",
        }
    }

    pub(crate) fn as_image_format(&self) -> ImageFormat {
        match self {
            Self::Jpg => ImageFormat::Jpeg,
            Self::Png => ImageFormat::Png,
        }
    }

    pub(crate) fn default() -> Self {
        Self::Png
    }

    pub(crate) fn from_str(format: &str) -> Result<Self, ()> {
        match format {
            "jpg" | "jpeg" => Ok(Self::Jpg),
            "png" => Ok(Self::Png),
            _ => Err(()),
        }
    }
}

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

    if let Some(parent) = std::path::Path::new(&file).parent() {
        if !parent.as_os_str().is_empty() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                let dir = parent.display().to_string();
                exit_with_error(1, mk_dir_create_error_msg(&dir, &e));
            }
        }
    }

    if let Err(e) = save_buffer_with_format(
        &file,
        &buf,
        width,
        height,
        ExtendedColorType::Rgb8,
        format.as_image_format(),
    ) {
        exit_with_error(1, mk_write_error_msg(e, &file));
    }

    println!();
    println!("{}", "Success!".green().bold());
    println!(
        "{} {}{}",
        "You can find your new wallpaper here:".blue().italic(),
        get_absolute_path(&file).yellow().bold().italic(),
        "!".blue().italic(),
    )
}
