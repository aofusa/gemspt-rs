#![allow(dead_code)]

use std::u64;

// xorshift64*による乱数ジェネレータ
// Sebastiano Vigna. An experimental exploration of Marsaglia's xorshift generators, scrambled. CoRR, abs/1402.6246, 2014. 
// http://xorshift.di.unimi.it/
#[derive(Debug)]
pub struct XorShift {
    x: u64,
}

impl XorShift {

    pub fn next(&mut self) -> u64 {
        self.x ^= self.x >> 12;  // a
        self.x ^= self.x << 25;  // b
        self.x ^= self.x >> 27;  // c
        self.x.wrapping_mul(2685821657736338717_u64)
    }

    pub fn next01(&mut self) -> f64 {
        self.next() as f64 / u64::MAX as f64
    }

    // [min_value, max_value]
    pub fn next_range(&mut self, min_value: f64, max_value: f64) -> f64 {
        let inv: f64 = max_value - min_value;
        (self.next() as f64 * (inv / u64::MAX as f64)) + min_value
    }

    pub fn new(initial_seed: u64) -> XorShift {
        if initial_seed == 0 {
            XorShift { x: 0xDEADBEEFDEADBEEF } // xorshift64*のseedは非ゼロでないといけない。
        } else {
            XorShift { x: initial_seed }
        }
    }
}

pub type Random = XorShift;
