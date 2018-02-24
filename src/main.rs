extern crate rayon;

mod constant;
mod vec;
mod hitpoint;
mod random;
mod ray;
mod sampling;
mod sphere;
mod material;
mod ppm;
mod scene;
mod radiance;
mod render;

use scene::{SceneRendering, generate_scene};
use render::render;

fn main() {
    println!("gemspt 2015 written in Rust");

    let scene = generate_scene(SceneRendering::SceneGlass);

    render(
        &scene,
        "image.ppm", // 保存ファイル名
        640, 480,    // 解像度
        1,           // サブピクセルごとのサンプリング数
        4,           // サブピクセルの縦横解像度
        true);       // 並列処理の有効化

    println!("Done.");
}

