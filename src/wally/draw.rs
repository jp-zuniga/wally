use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use rayon::slice::ParallelSliceMut;

use super::consts::TILE_HEIGHT;
use super::dots::Dot;
use super::img::Color;

#[inline]
pub(crate) fn blend(src: Color, dst: Color, alpha: f32) -> Color {
    let inv = 1.0 - alpha;

    Color {
        r: alpha * src.r + inv * dst.r,
        g: alpha * src.g + inv * dst.g,
        b: alpha * src.b + inv * dst.b,
    }
}

pub(crate) fn draw_tiled_dots(
    pixels: &mut [Color],
    dots: &[Dot],
    height: u32,
    width: u32,
    base_alpha: f32,
) {
    let width_usize = width as usize;
    let height_i = height as i32;

    if width_usize == 0 || height_i <= 0 || dots.is_empty() {
        return;
    }

    let num_tiles = ((height_i + TILE_HEIGHT - 1) / TILE_HEIGHT) as usize;

    let mut buckets: Vec<Vec<usize>> = vec![Vec::new(); num_tiles];

    for (idx, dot) in dots.iter().enumerate() {
        let mut first_tile = dot.y_min / TILE_HEIGHT;
        let mut last_tile = dot.y_max / TILE_HEIGHT;

        if first_tile < 0 {
            first_tile = 0;
        }

        if last_tile >= num_tiles as i32 {
            last_tile = num_tiles as i32 - 1;
        }

        for tile_y in first_tile..=last_tile {
            buckets[tile_y as usize].push(idx);
        }
    }

    let tile_row_count = TILE_HEIGHT as usize;
    let row_stride = width_usize;

    pixels
        .par_chunks_mut(row_stride * tile_row_count)
        .enumerate()
        .zip(buckets.into_par_iter())
        .for_each(|((tile_idx, tile_pixels), bucket)| {
            if bucket.is_empty() {
                return;
            }

            let chunk_rows = tile_pixels.len() / row_stride;
            if chunk_rows == 0 {
                return;
            }

            let tile_start_y = (tile_idx * tile_row_count) as i32;
            let mut tile_end_y = tile_start_y + chunk_rows as i32 - 1;
            if tile_end_y >= height_i {
                tile_end_y = height_i - 1;
            }

            for local_row in 0..chunk_rows {
                let y = tile_start_y + local_row as i32;
                if y > tile_end_y {
                    break;
                }

                let py = y as f32 + 0.5;

                let row_slice_start = local_row * row_stride;
                let row_slice_end = row_slice_start + row_stride;
                let row_slice = &mut tile_pixels[row_slice_start..row_slice_end];

                for &dot_idx in &bucket {
                    let dot = &dots[dot_idx];

                    if y < dot.y_min || y > dot.y_max {
                        continue;
                    }

                    let dy = py - dot.y;
                    let dy2 = dy * dy;

                    if dy2 >= dot.ra2 {
                        continue;
                    }

                    let mut dx = dot.x_min as f32 - dot.x;
                    let xmin = dot.x_min;
                    let xmax = dot.x_max;

                    for x in xmin..=xmax {
                        let dist2 = dx * dx + dy2;
                        dx += 1.0;

                        if dist2 >= dot.ra2 {
                            continue;
                        }

                        let coverage = if dist2 <= dot.r2 {
                            1.0
                        } else {
                            (dot.radius + dot.aa_width - dist2.sqrt()).clamp(0.0, 1.0)
                        };

                        if coverage <= 0.0 {
                            continue;
                        }

                        let alpha = base_alpha * coverage;
                        let idx = x as usize;

                        let dst = row_slice[idx];
                        row_slice[idx] = blend(dot.color, dst, alpha);
                    }
                }
            }
        });
}
