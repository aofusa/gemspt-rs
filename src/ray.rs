#![allow(dead_code)]

use vec::Vec;

#[derive(Debug)]
pub struct Ray<'a> {
    pub org: &'a Vec,
    pub dir: &'a Vec,
}

impl<'a> Ray<'a> {
    pub fn new(org: &'a Vec, dir: &'a Vec) -> Ray<'a> {
        Ray { org: org, dir: dir }
    }
}

