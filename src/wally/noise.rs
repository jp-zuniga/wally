use noise_perlin::perlin_2d;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Perlin2D {
    off_x: f32,
    off_y: f32,
    freq: f32,
}

impl Perlin2D {
    pub(crate) fn new<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self {
            off_x: rng.random::<f32>() * 1000.0,
            off_y: rng.random::<f32>() * 1000.0,
            freq: rng.random_range(0.003..0.012),
        }
    }

    pub(crate) fn sample(&self, x: f32, y: f32) -> f32 {
        let v = perlin_2d(x * self.freq + self.off_x, y * self.freq + self.off_y);

        0.5 * (v + 1.0)
    }
}
