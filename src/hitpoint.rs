#![allow(dead_code)]

use vec::Vec;
use constant::K_INF;

#[derive(Debug)]
pub struct Hitpoint {
    pub distance: f64,
    pub normal: Vec,
    pub position: Vec,
}

impl Hitpoint {
    pub fn new() -> Hitpoint {
        Hitpoint {
            distance: K_INF,
            normal: Vec { x: 0.0, y: 0.0, z: 0.0 },
            position: Vec { x: 0.0, y: 0.0, z: 0.0 },
        }
    }
}

