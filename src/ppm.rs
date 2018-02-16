#![allow(dead_code)]

use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::{BufWriter, Write};

use vec::Vec;

type Color = Vec;

fn clamp(x: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else if x > 1.0 {
        1.0
    } else {
        x
    }
}

fn to_ldr(x: f64) -> i32 {
    // ディスプレイのガンマが2.2であることを仮定し、1/2.2乗する。
    // 簡易的なLDR化処理。
    (clamp(x).powf(1.0/2.2) * 255.0 + 0.5) as i32
}

pub fn save_ppm_file(filename: &str, image: &[Color], width: i32, height: i32) {
    // 目的ファイルに対する`Path`を作成
    let path = Path::new(filename);
    let display = path.display();

    // ファイルを書き込み専用モードで開く。返り値は`io::Result<File>`
    let mut file_handler = BufWriter::new( match File::create(&path) {
        // `io::Error`の`description`メソッドはエラーを説明する文字列を返す。
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           Error::description(&why)),
        Ok(file) => file,
    });

    file_handler.write(format!("P3\n{} {}\n{}\n", width, height, 255).as_bytes()).unwrap();
    for color in image {
        file_handler.write(format!("{} {} {} ", to_ldr(color.x), to_ldr(color.y), to_ldr(color.z)).as_bytes()).unwrap();
    }
}

