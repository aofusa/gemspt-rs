#![allow(dead_code)]

use vec::{Vec, Dot, Normalize};
use ray::Ray;
use hitpoint::Hitpoint;

// 球の幾何学的な情報を持つ
#[derive(Debug)]
pub struct Sphere {
    radius_: f64,
    position_: Vec,
}

impl Sphere {
    pub fn new(radius: f64, position: Vec) -> Sphere {
        Sphere { radius_: radius, position_: position }
    }

    // 入力のrayに対する交差点までの距離を得る。
    // 交差したらその情報を,さもなくばNoneを返す。
    pub fn intersect(&self, ray: &Ray) -> Option<Hitpoint> {
        // 自己交差の判定用定数。
        const K_EPS: f64 = 1e-6_f64;

        let o_to_p: Vec = &self.position_ - ray.org;
        let b: f64 = Vec::dot(&o_to_p, &ray.dir);
        let c: f64 = &b * &b - Vec::dot(&o_to_p, &o_to_p) + &self.radius_ * &self.radius_;

        if &c < &0.0_f64 {
            return None
        }

        let sqrt_c: f64 = c.sqrt();
        let t1: f64 = &b - &sqrt_c;
        let t2: f64 = &b + &sqrt_c;

        // 微小距離内だったら交差しないとする（自己交差を避けるため）。
        if &t1 < &K_EPS && &t2 < &K_EPS {
            return None
        }

        let mut hitpoint = Hitpoint::new();

        // 交差するときは二点以上で交差する。（接する場合は一点）
        // 近い方を交差点とする。また、負値の場合はレイの逆方向で交差してるため交差していないとする。
        if &t1 > &K_EPS {
            hitpoint.distance = t1
        } else {
            hitpoint.distance = t2
        }

        hitpoint.position = ray.org + &hitpoint.distance * ray.dir;
        hitpoint.normal = Vec::normalize(&hitpoint.position - &self.position_);

        Some(hitpoint)
    }
}

