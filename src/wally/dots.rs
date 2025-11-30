use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use super::cli::args::WallArgs;
use super::consts::BASE_ALPHA;
use super::draw::draw_tiled_dots;
use super::img::{Color, write_img};
use super::noise::Perlin2D;
use super::themes::Palette;
use super::utils::resolve_output_file;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Dot {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) radius: f32,
    pub(crate) r2: f32,
    pub(crate) ra2: f32,
    pub(crate) x_min: i32,
    pub(crate) x_max: i32,
    pub(crate) y_min: i32,
    pub(crate) y_max: i32,
    pub(crate) color: Color,
    pub(crate) aa_width: f32,
}

pub(crate) fn mk_dots(args: &WallArgs, dot_size: f32, steps: u32, palette: Palette) {
    let steps_usize = steps as usize;
    let width_usize = args.width as usize;
    let height_usize = args.height as usize;
    let total_pixels = width_usize * height_usize;

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
    let mut dots = Vec::new();

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

            let r = dot_size;
            let ra = r + aa_width;
            let r2 = r * r;
            let ra2 = ra * ra;

            let x_min_px =
                ((x_pos - ra).floor() as i32).clamp(0, args.width as i32 - 1);
            let x_max_px = ((x_pos + ra).ceil() as i32).clamp(0, args.width as i32 - 1);
            let y_min_px =
                ((y_pos - ra).floor() as i32).clamp(0, args.height as i32 - 1);
            let y_max_px =
                ((y_pos + ra).ceil() as i32).clamp(0, args.height as i32 - 1);

            dots.push(Dot {
                x: x_pos,
                y: y_pos,
                radius: r,
                r2,
                ra2,
                x_min: x_min_px,
                x_max: x_max_px,
                y_min: y_min_px,
                y_max: y_max_px,
                color,
                aa_width,
            });
        }
    }

    draw_tiled_dots(&mut pixels, &dots, args.height, args.width, BASE_ALPHA);

    write_img(
        resolve_output_file(&args.name, args.format),
        args.format,
        args.width,
        args.height,
        &pixels,
    );
}
