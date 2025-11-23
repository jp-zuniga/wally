use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use crate::wally::themes::ColorPalette;

use super::cli::WallyCLI;
use super::consts::BASE_ALPHA;
use super::draw::{Circle, draw_circle};
use super::img::write_img;
use super::noise::Perlin2D;

pub(crate) fn mk_dots<T: ColorPalette>(
    args: &WallyCLI,
    dot_size: f32,
    steps: u32,
    palette: T,
) {
    let width_usize = args.width as usize;
    let height_usize = args.height as usize;
    let steps_usize = steps as usize;

    let total_pixels = width_usize
        .checked_mul(height_usize)
        .expect("image dimensions are too large!");

    let width_f = args.width as f32;
    let height_f = args.height as f32;
    let padding_f = args.padding as f32;
    let steps_f = steps as f32;

    let grid_width = (width_f / steps_f).floor() * steps_f;
    let grid_height = (height_f / steps_f).floor() * steps_f;

    let x_min = padding_f;
    let x_max = width_f - padding_f;
    let y_min = padding_f;
    let y_max = height_f - padding_f;

    let x_scale = (x_max - x_min) / grid_width;
    let y_scale = (y_max - y_min) / grid_height;

    let aa_width = dot_size * 0.1;
    let palette_len = palette.len();

    let mut pixels = vec![palette.background(); total_pixels];

    let mut chaos = SmallRng::seed_from_u64(
        args.seed.unwrap_or_else(|| rand::rng().random::<u64>()),
    );

    let noise = Perlin2D::new(&mut chaos);

    for gx in (0..=args.width).step_by(steps_usize) {
        let gx_f = gx as f32;

        for gy in (0..=args.height).step_by(steps_usize) {
            let gy_f = gy as f32;

            let n2 = noise.sample(gx_f, gy_f);

            if chaos.random::<f32>() > n2 {
                continue;
            }

            let color = palette.get_color(chaos.random_range(0..palette_len));

            let x_pos = x_min + gx_f * x_scale;
            let y_pos = y_min + gy_f * y_scale;

            let circle = Circle {
                radius: dot_size,
                x: x_pos,
                y: y_pos,
            };

            draw_circle(
                &mut pixels,
                BASE_ALPHA,
                color,
                aa_width,
                args.width,
                args.height,
                circle,
            );
        }
    }

    write_img(
        format!("{}.{}", args.name, args.format.as_str()),
        args.format,
        args.width,
        args.height,
        &pixels,
    );
}
