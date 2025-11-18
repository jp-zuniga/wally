use image::{Rgb, RgbImage};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use super::cli::WallyCLI;
use super::draw::{Circle, draw_circle};
use super::flavors::{COLOR_COUNT, RosePineFlavor};
use super::utils::{map_float, noise2d};

pub(crate) fn mk_wall(args: &WallyCLI, flavor: &RosePineFlavor) {
    let mut chaos = StdRng::from_rng(&mut rand::rng());

    let mut pixels = vec![flavor.base; (args.width * args.height) as usize];

    let grid_width = (args.width / args.steps * args.steps) as f64;
    let grid_height = (args.height / args.steps * args.steps) as f64;

    let alpha = 128.0 / 255.0;
    let radius = args.dot_size * 0.5;
    let padding = args.steps * 3;

    for gx in (0..=args.width).step_by(args.steps as usize) {
        for gy in (0..=args.height).step_by(args.steps as usize) {
            let n2 = noise2d(gx as f64 * 0.005, gy as f64 * 0.005);
            if !args.full_grid && n2 <= 0.5 {
                continue;
            }

            let color = flavor.get(chaos.random_range(0..COLOR_COUNT));

            let x_pos = map_float(
                gx as f64,
                0.0,
                grid_width,
                padding as f64,
                (args.width - padding) as f64,
            );

            let y_pos = map_float(
                gy as f64,
                0.0,
                grid_height,
                padding as f64,
                (args.height - padding) as f64,
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

    let mut img = RgbImage::new(args.width, args.height);

    for y in 0..args.height {
        for x in 0..args.width {
            let idx = (y * args.width + x) as usize;
            let p = pixels[idx];

            let [r, g, b] = p.to_rgb8();

            img.put_pixel(x, y, Rgb([r, g, b]));
        }
    }

    let file = format!("{}.{}", args.file_name, args.format.as_string());

    img.save(file.clone()).expect("Failed to save {file}");
    println!("Wrote {file}.");
}
