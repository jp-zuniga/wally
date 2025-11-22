use super::img::Color;

use super::consts::DEFAULT_AA_RANGE;
use super::utils::blend;

pub(crate) struct Circle {
    pub(crate) radius: f32,
    pub(crate) x: f32,
    pub(crate) y: f32,
}

pub(crate) fn draw_circle(
    pixels: &mut [Color],
    base_alpha: f32,
    color: Color,
    width: u32,
    height: u32,
    circle: Circle,
) {
    let r = circle.radius;
    let ra = r + DEFAULT_AA_RANGE;

    let r2 = r * r;
    let ra2 = ra * ra;

    let mut xmin = (circle.x - r - DEFAULT_AA_RANGE).floor() as i32;
    let mut xmax = (circle.x + r + DEFAULT_AA_RANGE).ceil() as i32;
    let mut ymin = (circle.y - r - DEFAULT_AA_RANGE).floor() as i32;
    let mut ymax = (circle.y + r + DEFAULT_AA_RANGE).ceil() as i32;

    let w_i = width as i32;
    let h_i = height as i32;

    xmin = xmin.clamp(0, w_i - 1);
    xmax = xmax.clamp(0, w_i - 1);
    ymin = ymin.clamp(0, h_i - 1);
    ymax = ymax.clamp(0, h_i - 1);

    for y in ymin..=ymax {
        let py = y as f32 + 0.5;
        let dy = py - circle.y;

        for x in xmin..=xmax {
            let px = x as f32 + 0.5;
            let dx = px - circle.x;
            let dist2 = dx * dx + dy * dy;

            if dist2 >= ra2 {
                continue;
            }

            let coverage = if dist2 <= r2 {
                1.0
            } else {
                let dist = dist2.sqrt();
                (ra - dist).clamp(0.0, 1.0)
            };

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
