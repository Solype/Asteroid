use rand::Rng;
use rand_distr::{Distribution, Normal};

pub fn sample_truncated_norm(
    mean: f32,
    std_dev: f32,
    lower: f32,
    upper: f32,
    rng: &mut impl Rng,
) -> f32 {
    let dist = Normal::new(mean, std_dev).unwrap();
    loop {
        let v = dist.sample(rng);
        if v >= lower && v <= upper {
            return v;
        }
    }
}

pub fn f(x: f32) -> f32 {
    2.0 * (-(x / 2.0 - 2.5).tanh() + 2.0)
}
