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

    let xmin = (circle.x - r - DEFAULT_AA_RANGE).floor() as i32;
    let xmax = (circle.x + r + DEFAULT_AA_RANGE).ceil() as i32;
    let ymin = (circle.y - r - DEFAULT_AA_RANGE).floor() as i32;
    let ymax = (circle.y + r + DEFAULT_AA_RANGE).ceil() as i32;

    let w_i = width as i32;
    let h_i = height as i32;

    for y in ymin..=ymax {
        if y < 0 || y >= h_i {
            continue;
        }

        let py = y as f32 + 0.5;
        let dy = py - circle.y;

        let mut px = xmin as f32 + 0.5;
        let mut dx = px - circle.x;

        for x in xmin..=xmax {
            if x < 0 || x >= w_i {
                px += 1.0;
                dx += 1.0;
                continue;
            }

            let dist2 = dx * dx + dy * dy;

            if dist2 >= ra2 {
                px += 1.0;
                dx += 1.0;
                continue;
            }

            let coverage = if dist2 <= r2 {
                1.0
            } else {
                let dist = dist2.sqrt();
                (ra - dist).clamp(0.0, 1.0)
            };

            if coverage <= 0.0 {
                px += 1.0;
                dx += 1.0;
                continue;
            }

            let alpha = base_alpha * coverage;
            let idx = (y as u32 * width + x as u32) as usize;
            let dst = pixels[idx];

            pixels[idx] = blend(color, dst, alpha);

            px += 1.0;
            dx += 1.0;
        }
    }
}
