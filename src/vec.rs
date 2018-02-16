#![allow(dead_code)]

use std::ops::{Add, Sub, Mul, Div, Neg};
use std::f64;

// ベクトル演算用クラス
#[derive(Debug, Clone)]
pub struct Vec {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Add for Vec {
    type Output = Vec;

    fn add(self, other: Vec) -> Vec {
        Vec { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl<'a, 'b> Add<&'a Vec> for &'b Vec {
    type Output = Vec;

    fn add(self, other: &'a Vec) -> Vec {
        Vec { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl<'a> Add<Vec> for &'a Vec {
    type Output = Vec;

    fn add(self, other: Vec) -> Vec {
        Vec { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl<'a> Add<&'a Vec> for Vec {
    type Output = Vec;

    fn add(self, other: &'a Vec) -> Vec {
        Vec { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl<'a, 'b> Sub<&'a Vec> for &'b Vec {
    type Output = Vec;

    fn sub(self, other: &'a Vec) -> Vec {
        Vec { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl Sub for Vec {
    type Output = Vec;

    fn sub(self, other: Vec) -> Vec {
        Vec { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl<'a> Sub<Vec> for &'a Vec {
    type Output = Vec;

    fn sub(self, other: Vec) -> Vec {
        Vec { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl<'a> Sub<&'a Vec> for Vec {
    type Output = Vec;

    fn sub(self, other: &'a Vec) -> Vec {
        Vec { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl<'a, 'b> Mul<&'a f64> for &'b Vec {
    type Output = Vec;

    fn mul(self, x: &'a f64) -> Vec {
        Vec { x: self.x * x, y: self.y * x, z: self.z * x }
    }
}

impl<'a> Mul<f64> for &'a Vec {
    type Output = Vec;

    fn mul(self, x: f64) -> Vec {
        Vec { x: self.x * x, y: self.y * x, z: self.z * x }
    }
}

impl<'a> Mul<&'a f64> for Vec {
    type Output = Vec;

    fn mul(self, x: &'a f64) -> Vec {
        Vec { x: self.x * x, y: self.y * x, z: self.z * x }
    }
}

impl Mul<f64> for Vec {
    type Output = Vec;

    fn mul(self, x: f64) -> Vec {
        Vec { x: self.x * x, y: self.y * x, z: self.z * x }
    }
}

impl<'a> Div<f64> for &'a Vec {
    type Output = Vec;

    fn div(self, x: f64) -> Vec {
        Vec { x: self.x / x, y: self.y / x, z: self.z / x }
    }
}

impl Div<f64> for Vec {
    type Output = Vec;

    fn div(self, x: f64) -> Vec {
        Vec { x: self.x / x, y: self.y / x, z: self.z / x }
    }
}

impl<'a> Neg for &'a Vec {
    type Output = Vec;

    fn neg(self) -> Vec {
        Vec { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl Neg for Vec {
    type Output = Vec;

    fn neg(self) -> Vec {
        Vec { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl Vec {
    pub fn length_squared(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn length(&self) -> f64 {
        (self.length_squared()).sqrt()
    }
}

impl<'a, 'b> Mul<&'a Vec> for &'b f64 {
    type Output = Vec;

    fn mul(self, v: &'a Vec) -> Vec {
        v * *self
    }
}

impl<'a> Mul<&'a Vec> for f64 {
    type Output = Vec;

    fn mul(self, v: &'a Vec) -> Vec {
        v * self
    }
}

impl<'a> Mul<Vec> for &'a f64 {
    type Output = Vec;

    fn mul(self, v: Vec) -> Vec {
        v * *self
    }
}

impl Mul<Vec> for f64 {
    type Output = Vec;

    fn mul(self, v: Vec) -> Vec {
        v * self
    }
}

pub trait Normalize<T> {
    fn normalize(v: T) -> Vec;
}

impl Normalize<Vec> for Vec {
    fn normalize(v: Vec) -> Vec {
        &v * (1.0 / v.length())
    }
}

impl<'a> Normalize<&'a Vec> for Vec {
    fn normalize(v: &'a Vec) -> Vec {
        v * (1.0 / v.length())
    }
}

pub trait Multiply<T> {
    fn multiply(v1: T, v2: T) -> Vec;
}

impl<'a> Multiply<&'a Vec> for Vec {
    fn multiply(v1: &Vec, v2: &Vec) -> Vec {
        Vec { x: v1.x*v2.x, y: v1.y*v2.y, z: v1.z*v2.z }
    }
}

impl Multiply<Vec> for Vec {
    fn multiply(v1: Vec, v2: Vec) -> Vec {
        Vec { x: v1.x*v2.x, y: v1.y*v2.y, z: v1.z*v2.z }
    }
}

pub trait Dot<T> {
    fn dot(v1: T, v2: T) -> f64;
}

impl<'a> Dot<&'a Vec> for Vec {
    fn dot(v1: &Vec, v2: &Vec) -> f64 {
        v1.x*v2.x + v1.y*v2.y + v1.z*v2.z
    }
}

impl Dot<Vec> for Vec {
    fn dot(v1: Vec, v2: Vec) -> f64 {
        v1.x*v2.x + v1.y*v2.y + v1.z*v2.z
    }
}

pub trait Cross<T> {
    fn cross(v1: T, v2: T) -> Vec;
}

impl<'a> Cross<&'a Vec> for Vec {
    fn cross(v1: &Vec, v2: &Vec) -> Vec {
        Vec {
            x: (v1.y * v2.z) - (v1.z * v2.y),
            y: (v1.z * v2.x) - (v1.x * v2.z),
            z: (v1.x * v2.y) - (v1.y * v2.x)
        }
    }
}

impl Cross<Vec> for Vec {
    fn cross(v1: Vec, v2: Vec) -> Vec {
        Vec {
            x: (v1.y * v2.z) - (v1.z * v2.y),
            y: (v1.z * v2.x) - (v1.x * v2.z),
            z: (v1.x * v2.y) - (v1.y * v2.x)
        }
    }
}

pub trait Reflect<T> {
    fn reflect(v1: T, v2: T) -> Vec;
}

impl<'a> Reflect<&'a Vec> for Vec {
    fn reflect(v: &Vec, normal: &Vec) -> Vec {
        Vec::normalize(v - normal * 2.0 * Vec::dot(normal, v))
    }
}

impl Reflect<Vec> for Vec {
    fn reflect(v: Vec, normal: Vec) -> Vec {
        Vec::normalize(&v - &normal * 2.0 * Vec::dot(&normal, &v))
    }
}

// 正規直交基底を作る
pub fn create_ortho_normal_basis(
    normal: &Vec, tangent: &mut Vec, binormal: &mut Vec) {
    if f64::abs(normal.x) > f64::abs(normal.y) {
        *tangent = Vec::normalize(Vec::cross(&Vec{x: 0.0, y: 1.0, z: 0.0}, normal));
    } else {
        *tangent = Vec::normalize(Vec::cross(&Vec{x: 1.0, y: 0.0, z: 0.0}, normal));
    }
    *binormal = Vec::normalize(Vec::cross(normal, tangent));
}

