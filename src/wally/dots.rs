use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::wally::themes::ColorPalette;

use super::cli::WallyCLI;
use super::draw::{Circle, draw_circle};
use super::img::write_img;
use super::noise::Perlin2D;
use super::utils::map_float;

pub(crate) fn mk_dots<T: ColorPalette>(
    args: &WallyCLI,
    dot_size: &f32,
    steps: &u32,
    palette: T,
) {
    let mut pixels = vec![palette.background(); (args.width * args.height) as usize];

    let grid_width = (args.width / steps * steps) as f32;
    let grid_height = (args.height / steps * steps) as f32;

    let alpha = 128.0 / 255.0;
    let radius = dot_size * 0.5;
    let aa_width = dot_size * 0.1;

    let mut chaos =
        StdRng::seed_from_u64(args.seed.unwrap_or_else(|| rand::rng().random::<u64>()));

    let noise = Perlin2D::new(&mut chaos);

    for gx in (0..=args.width).step_by(*steps as usize) {
        for gy in (0..=args.height).step_by(*steps as usize) {
            let n2 = noise.sample(gx as f32, gy as f32);

            if chaos.random::<f32>() > n2 {
                continue;
            }

            let color = palette.get_color(chaos.random_range(0..palette.len()));

            let x_pos = map_float(
                gx as f32,
                0.0,
                grid_width,
                args.padding as f32,
                (args.width - args.padding) as f32,
            );

            let y_pos = map_float(
                gy as f32,
                0.0,
                grid_height,
                args.padding as f32,
                (args.height - args.padding) as f32,
            );

            let cur_circle = Circle {
                radius,
                x: x_pos,
                y: y_pos,
            };

            draw_circle(
                &mut pixels,
                alpha,
                color,
                aa_width,
                args.width,
                args.height,
                cur_circle,
            );
        }
    }

    write_img(
        format!("{}.{}", args.name, args.format.as_string()),
        args.format,
        args.width,
        args.height,
        &pixels,
    );
}
