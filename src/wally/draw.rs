use super::img::Color;

use super::consts::DEFAULT_AA_RANGE;
use super::utils::blend;

pub(crate) struct Circle {
    pub(crate) radius: f64,
    pub(crate) x: f64,
    pub(crate) y: f64,
}

pub(crate) fn draw_circle(
    pixels: &mut [Color],
    base_alpha: f64,
    color: Color,
    width: u32,
    height: u32,
    circle: Circle,
) {
    let xmin = (circle.x - circle.radius - DEFAULT_AA_RANGE).floor() as i32;
    let xmax = (circle.x + circle.radius + DEFAULT_AA_RANGE).ceil() as i32;
    let ymin = (circle.y - circle.radius - DEFAULT_AA_RANGE).floor() as i32;
    let ymax = (circle.y + circle.radius + DEFAULT_AA_RANGE).ceil() as i32;

    for y in ymin..=ymax {
        if y < 0 || y >= height as i32 {
            continue;
        }

        for x in xmin..=xmax {
            if x < 0 || x >= width as i32 {
                continue;
            }

            let px = x as f64 + 0.5;
            let py = y as f64 + 0.5;

            let dx = px - circle.x;
            let dy = py - circle.y;
            let dist = (dx * dx + dy * dy).sqrt();

            let coverage = (circle.radius + DEFAULT_AA_RANGE - dist).clamp(0.0, 1.0);
            if coverage <= 0.0 {
                continue;
            }

            let alpha = base_alpha * coverage;
            let idx = (y as u32 * width + x as u32) as usize;
            let dst = pixels[idx];

            pixels[idx] = blend(color, dst, alpha);
        }
    }
}
