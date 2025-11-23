use super::img::Color;

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
    aa_width: f32,
    width: u32,
    height: u32,
    circle: Circle,
) {
    let r = circle.radius;
    let ra = r + aa_width;

    let r2 = r * r;
    let ra2 = ra * ra;

    let w_i = width as i32;
    let h_i = height as i32;

    let xmin = ((circle.x - ra).floor() as i32).clamp(0, w_i - 1);
    let xmax = ((circle.x + ra).ceil() as i32).clamp(0, w_i - 1);
    let ymin = ((circle.y - ra).floor() as i32).clamp(0, h_i - 1);
    let ymax = ((circle.y + ra).ceil() as i32).clamp(0, h_i - 1);

    let xmin_f = xmin as f32;
    let width_usize = width as usize;

    for y in ymin..=ymax {
        let py = y as f32 + 0.5;
        let dy = py - circle.y;
        let dy2 = dy * dy;

        let row_start = (y as usize) * width_usize;

        let mut dx = xmin_f - circle.x;

        for x in xmin..=xmax {
            let dist2 = dx * dx + dy2;
            dx += 1.0;

            if dist2 >= ra2 {
                continue;
            }

            let coverage = if dist2 <= r2 {
                1.0
            } else {
                let dist = dist2.sqrt();
                (ra - dist).max(0.0).min(1.0)
            };

            if coverage <= 0.0 {
                continue;
            }

            let alpha = base_alpha * coverage;
            let idx = row_start + x as usize;

            let dst = pixels[idx];
            pixels[idx] = blend(color, dst, alpha);
        }
    }
}
