#![allow(dead_code)]

use vec::Vec;
use random::Random;
use constant::K_PI;

#[derive(Debug)]
pub struct Sampling;

impl Sampling {
    pub fn uniform_hemisphere_surface(
        random: &mut Random, normal: &Vec, tangent: &Vec, binormal: &Vec) -> Vec {
            let tz: f64 = random.next_range(0.0, 1.0);
            let phi: f64 = random.next_range(0.0, 2.0 * K_PI);
            let k: f64 = (1.0 - tz * tz).sqrt();
            let tx: f64 = k * phi.cos();
            let ty: f64 = k * phi.sin();

            tz * normal + tx * tangent + ty * binormal
    }

    pub fn cosine_weighted_hemisphere_surface(
        random: &mut Random, normal: &Vec, tangent: &Vec, binormal: &Vec) -> Vec {
            let phi: f64 = random.next_range(0.0, 2.0 * K_PI);
            let r2: f64 = random.next01();
            let r2s: f64 = r2.sqrt();

            let tx: f64 = r2s * phi.cos();
            let ty: f64 = r2s * phi.sin();
            let tz: f64 = (1.0 - r2).sqrt();

            tz * normal + tx * tangent + ty * binormal
    }
}

