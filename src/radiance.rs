#![allow(dead_code)]

use ray::Ray;
use scene::{Scene, intersect_scene};
use random::Random;

use vec::{Vec, Dot, Multiply};

type Color = Vec;

// ray方向からの放射輝度を求める
pub fn radiance(scene: &Scene, ray: &Ray, random: &mut Random, depth: &i32) -> Color {
    const BACKGROUND_COLOR: Color = Color { x: 0.0, y: 0.0, z: 0.0 };
    const DEPTH_LIMIT: i32 = 10;
    // 打ち切りチェック
    if depth >= &DEPTH_LIMIT {
        return Color { x: 0.0, y: 0.0, z: 0.0 }
    }

    // シーンと交差判定
    // 交差チェック
    let (now_object, hitpoint) = match intersect_scene(scene, ray) {
        (Some(object), hitpoint) => (object, hitpoint),
        (None, _) => return BACKGROUND_COLOR
    };
    
    // マテリアル取得
    let now_material = now_object.get_material();
    let emission = now_material.emission();
    if &emission.x > &0.0 ||&emission.y > &0.0 || &emission.z > &0.0 {
        // 光源にヒットしたら放射項だけ返して終わる。
        // （今回、光源は反射率0と仮定しているため）
        return emission.clone()
    }

    // 次の方向をサンプリング + その方向のBRDF項の値を得る。
    let mut pdf = -1.0;
    let mut brdf_value: Color = Color { x: 0.0, y: 0.0, z: 0.0 };
    let dir_out = now_material.sample(random, &ray.dir, &hitpoint.normal, &mut pdf, &mut brdf_value);

    // cos項。
    let cost = Vec::dot(&hitpoint.normal, &dir_out);

    // レンダリング方程式をモンテカルロ積分によって再帰的に解く。
    Vec::multiply(
        brdf_value,
        radiance(scene, &Ray::new(&hitpoint.position, &dir_out), random, &(depth + 1)))
        * cost / pdf
}

