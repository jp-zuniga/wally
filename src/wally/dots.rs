use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::wally::themes::ThemeFlavor;

use super::cli::WallyCLI;
use super::draw::{Circle, draw_circle};
use super::img::write_img;
use super::noise::Perlin2D;
use super::utils::map_float;

pub(crate) fn mk_dots<T: ThemeFlavor>(args: &WallyCLI, palette: T) {
    let mut pixels = vec![palette.background(); (args.width * args.height) as usize];

    let grid_width = (args.width / args.steps * args.steps) as f64;
    let grid_height = (args.height / args.steps * args.steps) as f64;

    let alpha = 128.0 / 255.0;
    let radius = args.dot_size * 0.5;

    let mut chaos = StdRng::from_rng(&mut rand::rng());

    let noise = Perlin2D::new(&mut chaos);

    for gx in (0..=args.width).step_by(args.steps as usize) {
        for gy in (0..=args.height).step_by(args.steps as usize) {
            let n2 = noise.sample(gx as f64, gy as f64);

            if chaos.random::<f64>() > n2 {
                continue;
            }

            let color = palette.get_color(chaos.random_range(0..palette.len()));

            let x_pos = map_float(
                gx as f64,
                0.0,
                grid_width,
                args.padding as f64,
                (args.width - args.padding) as f64,
            );

            let y_pos = map_float(
                gy as f64,
                0.0,
                grid_height,
                args.padding as f64,
                (args.height - args.padding) as f64,
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
                args.width,
                args.height,
                cur_circle,
            );
        }
    }

    write_img(
        format!("{}.{}", args.file_name, args.format.as_string()),
        &args.width,
        &args.height,
        &pixels,
    );
}
