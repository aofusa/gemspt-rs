#![allow(dead_code, unused_variables)]

use std;
use std::sync::{Arc, Mutex};

use rayon::prelude::*;

use radiance::radiance;
use ppm::save_ppm_file;
use random::Random;

use vec::{Vec, Normalize, Cross};
use scene::Scene;
use ray::Ray;

macro_rules! clone_value {
    ($x:expr, $n:expr) => ({
        let mut temp_vec = std::vec::Vec::new();
        for _ in 0..$n {
            temp_vec.push($x);
        }
        temp_vec
    })
}

pub fn render(
    scene: &Scene, filename: &str, width: i32, height: i32,
    num_sample_per_subpixel: i32, num_subpixel: i32,
    is_parallel: bool) {

    // カメラ位置。
    let camera_position = Vec { x: 7.0, y: 3.0, z: 7.0 };
    let camera_lookat   = Vec { x: 0.0, y: 1.0, z: 0.0 };
    let camera_dir      = Vec::normalize(&camera_lookat - &camera_position);
    let camera_up       = Vec { x: 0.0, y: 1.0, z: 0.0 };

    // ワールド座標系でのイメージセンサーの大きさ。
    let sensor_width = 30.0 * width as f64 / height as f64; // アスペクト比調整。
    let sensor_height= 30.0;
    // イメージセンサーまでの距離。
    let sensor_dist  = 45.0;
    // イメージセンサーを張るベクトル。
    let sensor_x_vec = Vec::normalize(Vec::cross(&camera_dir, &camera_up)) * &sensor_width;
    let sensor_y_vec = Vec::normalize(Vec::cross(&sensor_x_vec, &camera_dir)) * &sensor_height;
    let sensor_center = &camera_position + &camera_dir * &sensor_dist;

    let mut image = clone_value![Vec { x: 0.0, y: 0.0, z: 0.0 }, (width * height) as u64];
    println!("{}x{} {} spp", width, height, num_sample_per_subpixel * (num_subpixel * num_subpixel));

    // Rayonを使った並列処理
    // 実装するために下の逐次処理版と一部ロジックが変わってしまった
    // 1. for 文から for_each メソッドへの変更
    // 2. 1.に伴う 二重ループの除去
    // 3. 1.に伴う image 配列へのアクセスロジックの変更
    // 4. 進捗表示処理の追加と表示回数抑制のための if 文の追加
    if is_parallel {
        let count = Arc::new(Mutex::new(0));
        image.par_iter_mut().enumerate().for_each(|(i, pixel)| {
            // 進捗表示
            {
                let mut count = count.lock().unwrap();
                *count += 1;
                let count = *count;
                if count % width == 0 {
                    eprint!("\rRendering (y= {}, {} %)\x1b[0K", count / width as i32, 100.0 * count as f64 / (width * height) as f64);
                }
            }
            let y = i as i32 / width;
            let x = i as i32 - y * width;
            let mut random = Random::new((y * width + x + 1) as u64);
            let image_index = height - y - 1;
            // num_subpixel x num_subpixel のスーパーサンプリング。
            for sy in 0..num_subpixel {
                for sx in 0..num_subpixel {
                    let mut accumulated_radiance = Vec { x: 0.0, y: 0.0, z: 0.0 };
                    // let accumulated_radiance: Vec;
                    // 一つのサブピクセルあたりsamples回サンプリングする。
                    for _s in 0..num_sample_per_subpixel {
                        let rate = 1.0 / num_subpixel as f64;
                        let r1 = sx as f64 * rate + rate / 2.0;
                        let r2 = sy as f64 * rate + rate / 2.0;
                        // イメージセンサー上の位置。
                        let position_on_sensor =
                            &sensor_center +
                            &sensor_x_vec * ((r1 + x as f64) / width as f64 - 0.5) +
                            &sensor_y_vec * ((r2 + image_index as f64) / height as f64 - 0.5);
                        // レイを飛ばす方向。
                        let dir = Vec::normalize(&position_on_sensor - &camera_position);

                        accumulated_radiance = accumulated_radiance +
                            radiance(scene, &Ray::new(&camera_position, &dir), &mut random, &0)
                            / num_sample_per_subpixel as f64 / (num_subpixel * num_subpixel) as f64;
                    }
                    *pixel = &*pixel + accumulated_radiance;
                }
            }
        });
        eprintln!("");
    }
    // 逐次処理
    else {
        for y in 0..height {
            eprint!("\rRendering (y= {}, {} %)\x1b[0K", y, 100.0 * y as f64 / (height - 1) as f64);
            for x in 0..width {
                let mut random = Random::new((y * width + x + 1) as u64);
                let image_index = ((height - y - 1) * width + x) as usize;
                // num_subpixel x num_subpixel のスーパーサンプリング。
                for sy in 0..num_subpixel {
                    for sx in 0..num_subpixel {
                        let mut accumulated_radiance = Vec { x: 0.0, y: 0.0, z: 0.0 };
                        // let accumulated_radiance: Vec;
                        // 一つのサブピクセルあたりsamples回サンプリングする。
                        for _s in 0..num_sample_per_subpixel {
                            let rate = 1.0 / num_subpixel as f64;
                            let r1 = sx as f64 * rate + rate / 2.0;
                            let r2 = sy as f64 * rate + rate / 2.0;
                            // イメージセンサー上の位置。
                            let position_on_sensor =
                                &sensor_center +
                                &sensor_x_vec * ((r1 + x as f64) / width as f64 - 0.5) +
                                &sensor_y_vec * ((r2 + y as f64) / height as f64 - 0.5);
                            // レイを飛ばす方向。
                            let dir = Vec::normalize(&position_on_sensor - &camera_position);

                            accumulated_radiance = accumulated_radiance +
                                radiance(scene, &Ray::new(&camera_position, &dir), &mut random, &0)
                                / num_sample_per_subpixel as f64 / (num_subpixel * num_subpixel) as f64;
                        }
                        image[image_index] = &image[image_index] + accumulated_radiance;
                    }
                }
            }
        }
        eprintln!("");
    }

    // 出力
    save_ppm_file(filename, &image, width, height);
}

