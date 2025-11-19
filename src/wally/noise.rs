use noise_perlin::perlin_2d;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Perlin2D {
    off_x: f64,
    off_y: f64,
    freq: f64,
}

impl Perlin2D {
    pub(crate) fn new<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
        Self {
            off_x: rng.random::<f64>() * 1000.0,
            off_y: rng.random::<f64>() * 1000.0,
            freq: rng.random_range(0.003..0.012),
        }
    }

    pub(crate) fn sample(&self, x: f64, y: f64) -> f64 {
        let v = perlin_2d(
            (x * self.freq + self.off_x) as f32,
            (y * self.freq + self.off_y) as f32,
        );

        0.5 * (v + 1.0) as f64
    }
}
