#![allow(dead_code)]
#![allow(unused_variables)]

use random::Random;
use vec::{Vec, Dot, Reflect, create_ortho_normal_basis};
use constant::K_PI;
use sampling::Sampling;

type Color = Vec;

const DELTA: f64 = 1.0;

// Lambertian BRDF
// 所謂完全拡散面
pub struct LambertianMaterialSimple { emission: Color, reflectance: Color, }

// Lambertian BRDF
// 所謂完全拡散面
// インポータンスサンプリング版。
pub struct LambertianMaterial { emission: Color, reflectance: Color, }

// 正規化Phong BRDF
pub struct PhongMaterial { emission: Color, reflectance: Color, n: f64, }

// 理想的なガラス面。
pub struct GlassMaterial { emission: Color, reflectance: Color, ior: f64, }

// 光源としてふるまうマテリアル
pub struct Lightsource { emission: Color, reflectance: Color, }

// マテリアルインターフェース
pub enum Material {
    LambertianMaterialSimple(LambertianMaterialSimple),
    LambertianMaterial      (LambertianMaterial),
    PhongMaterial           (PhongMaterial),
    GlassMaterial           (GlassMaterial),
    Lightsource             (Lightsource),
}

pub trait MaterialTrait {
    fn emission(&self) -> &Color;
    fn reflectance(&self) -> &Color;

    // in, outはカメラ側から光を逆方向に追跡したときの入出方向とする。
    // 以下、in = -omega, out = omega'となる。
    fn eval(&self, input: &Vec, normal: &Vec, output: &Vec) -> Color;
    fn sample(&self, random: &mut Random, input: &Vec, normal: &Vec, pdf: &mut f64, brdf_value: &mut Color) -> Vec;
}

impl MaterialTrait for Material {
    fn emission(&self) -> &Color {
        match self {
            &Material::LambertianMaterialSimple ( ref material ) => { &material.emission() },
            &Material::LambertianMaterial       ( ref material ) => { &material.emission() },
            &Material::PhongMaterial            ( ref material ) => { &material.emission() },
            &Material::GlassMaterial            ( ref material ) => { &material.emission() },
            &Material::Lightsource              ( ref material ) => { &material.emission() },
        }
    }

    fn reflectance(&self) -> &Color {
        match self {
            &Material::LambertianMaterialSimple ( ref material ) => { &material.reflectance() },
            &Material::LambertianMaterial       ( ref material ) => { &material.reflectance() },
            &Material::PhongMaterial            ( ref material ) => { &material.reflectance() },
            &Material::GlassMaterial            ( ref material ) => { &material.reflectance() },
            &Material::Lightsource              ( ref material ) => { &material.reflectance() },
        }
    }

    fn eval(&self, input: &Vec, normal: &Vec, output: &Vec) -> Color {
        match self {
            &Material::LambertianMaterialSimple ( ref material ) => { material.eval(input, normal, output) },
            &Material::LambertianMaterial       ( ref material ) => { material.eval(input, normal, output) },
            &Material::PhongMaterial            ( ref material ) => { material.eval(input, normal, output) },
            &Material::GlassMaterial            ( ref material ) => { material.eval(input, normal, output)},
            &Material::Lightsource              ( ref material ) => { material.eval(input, normal, output) },
        }
    }

    fn sample(&self, random: &mut Random, input: &Vec, normal: &Vec,
        pdf: &mut f64, brdf_value: &mut Color) -> Vec {
        match self {
            &Material::LambertianMaterialSimple ( ref material ) => { material.sample(random, input, normal, pdf, brdf_value) },
            &Material::LambertianMaterial       ( ref material ) => { material.sample(random, input, normal, pdf, brdf_value) },
            &Material::PhongMaterial            ( ref material ) => { material.sample(random, input, normal, pdf, brdf_value) },
            &Material::GlassMaterial            ( ref material ) => { material.sample(random, input, normal, pdf, brdf_value) },
            &Material::Lightsource              ( ref material ) => { material.sample(random, input, normal, pdf, brdf_value) },
        }
    }
}

impl LambertianMaterialSimple {
    pub fn new(reflectance: Color) -> LambertianMaterialSimple {
        LambertianMaterialSimple {
            emission: Color { x: 0.0, y: 0.0, z: 0.0 },
            reflectance: reflectance,
        }
    }
}

impl MaterialTrait for LambertianMaterialSimple {
    fn emission(&self) -> &Color {
        &self.emission
    }

    fn reflectance(&self) -> &Color {
        &self.reflectance
    }
    
    // Lambertian BRDFはρ/πになる（ρは反射率）
    fn eval(&self, input: &Vec, normal: &Vec, output: &Vec) -> Color  {
        &self.reflectance / K_PI
    }

    // 単純に半球一様サンプリングする。
    fn sample(
        &self, random: &mut Random, input: &Vec, normal: &Vec,
        pdf: &mut f64, brdf_value: &mut Color) -> Vec {

        let mut binormal: Vec = Vec { x: 0.0, y: 0.0, z: 0.0 };
        let mut tangent: Vec = Vec { x: 0.0, y: 0.0, z: 0.0 };
        let now_normal: &Vec = normal;

        create_ortho_normal_basis(now_normal, &mut tangent, &mut binormal);
        let dir: Vec = Sampling::uniform_hemisphere_surface(random, now_normal, &mut tangent, &mut binormal);

        *pdf = 1.0 / (2.0 * K_PI);
        *brdf_value = self.eval(input, normal, &dir);

        dir
    }
}

impl LambertianMaterial {
    pub fn new(reflectance: Color) -> LambertianMaterial {
        LambertianMaterial {
            emission: Color { x: 0.0, y: 0.0, z: 0.0 },
            reflectance: reflectance,
        }
    }
}

impl MaterialTrait for LambertianMaterial {
    fn emission(&self) -> &Color {
        &self.emission
    }

    fn reflectance(&self) -> &Color {
        &self.reflectance
    }
    
    // Lambertian BRDFはρ/πになる（ρは反射率）
    fn eval(&self, input: &Vec, normal: &Vec, output: &Vec) -> Color  {
        &self.reflectance / K_PI
    }

    // pdfとしてcosΘ/piを使用してインポータンスサンプリングする。
    fn sample(
        &self, random: &mut Random, input: &Vec, normal: &Vec,
        pdf: &mut f64, brdf_value: &mut Color) -> Vec {

        let mut binormal: Vec = Vec { x: 0.0, y: 0.0, z: 0.0 };
        let mut tangent: Vec = Vec { x: 0.0, y: 0.0, z: 0.0 };
        let now_normal: &Vec = normal;

        create_ortho_normal_basis(now_normal, &mut tangent, &mut binormal);
        let dir: Vec = Sampling::cosine_weighted_hemisphere_surface(random, now_normal, &mut tangent, &mut binormal);

        *pdf = Vec::dot(normal, &dir) / K_PI;
        *brdf_value = self.eval(input, normal, &dir);

        dir
    }
}

impl PhongMaterial {
    pub fn new(reflectance: Color, n: f64) -> PhongMaterial {
        PhongMaterial {
            emission: Color { x: 0.0, y: 0.0, z: 0.0 },
            reflectance: reflectance,
            n: n,
        }
    }
}

impl MaterialTrait for PhongMaterial {
    fn emission(&self) -> &Color {
        &self.emission
    }

    fn reflectance(&self) -> &Color {
        &self.reflectance
    }
    
    // 正規化Phong
    fn eval(&self, input: &Vec, normal: &Vec, output: &Vec) -> Color {
        if Vec::dot(normal, output) < 0.0 {
            // 次のレイの方向が地面より下の方向だったら0。
            return Color { x: 0.0, y: 0.0, z: 0.0 }
        }

        let reflection_dir: Vec = Vec::reflect(input, normal);
        let mut cosa: f64 = Vec::dot(&reflection_dir, output);
        if &cosa < &0.0 {
            cosa = 0.0;
        }

        &self.reflectance * (&self.n+ 2.0) / (2.0 * K_PI) * cosa.powf(self.n)
    }

    // BRDF形状をpdfとして使ってインポータンスサンプリングする。
    fn sample(
        &self, random: &mut Random, input: &Vec, normal: &Vec,
        pdf: &mut f64, brdf_value: &mut Color) -> Vec {

        let dir: Vec;
        let reflection_dir: Vec = Vec::reflect(input, normal);
        let mut binormal: Vec = Vec { x: 0.0, y: 0.0, z: 0.0 };
        let mut tangent: Vec = Vec { x: 0.0, y: 0.0, z: 0.0 };
        create_ortho_normal_basis(&reflection_dir, &mut tangent, &mut binormal);

        let u1: f64 = random.next01();
        let u2: f64 = random.next01();

        let phi: f64 = &u1 * 2.0 * &K_PI;
        let theta = &u2.powf(1.0 / (&self.n + 1.0)).acos();

        dir = tangent * theta.sin() * phi.cos() + &reflection_dir * theta.cos() + binormal * theta.sin() * phi.sin();

        let mut cosa: f64 = Vec::dot(&reflection_dir, &dir);
        if &cosa < &0.0 {
            cosa = 0.0;
        }
        *pdf = (&self.n + 1.0) / (2.0 * K_PI) * &cosa.powf(self.n);
        *brdf_value = self.eval(input, normal, &dir);

        dir
    }
}

impl GlassMaterial {
    pub fn new(reflectance: Color, ior: f64) -> GlassMaterial {
        GlassMaterial {
            emission: Color { x: 0.0, y: 0.0, z: 0.0 },
            reflectance: reflectance,
            ior: ior,
        }
    }
}

impl MaterialTrait for GlassMaterial {
    fn emission(&self) -> &Color {
        &self.emission
    }

    fn reflectance(&self) -> &Color {
        &self.reflectance
    }
    
    // 理想的なガラス面におけるBRDFはディラックのδ関数を使ってδ/cosΘとなる。
    // δ関数を表現することは出来ないが、モンテカルロ積分においてはpdfにもδ関数が現れるため分母と分子で打ち消し合う。
    // そこでcosΘと反射率だけ入れておく。
    // in, outはカメラ側から追跡したときの入出方向なので、光の入射方向はoutになるため、cosθは法線とoutの内積になる。
    // 反射率や透過率（Fr,Ft）は含まれていないことに注意！
    fn eval(&self, input: &Vec, normal: &Vec, output: &Vec) -> Color  {
        &self.reflectance * &DELTA / Vec::dot(normal, output)
    }

    fn sample(
        &self, random: &mut Random, input: &Vec, normal: &Vec,
        pdf: &mut f64, brdf_value: &mut Color) -> Vec {
        let now_normal: Vec = match Vec::dot(normal, input) {
             x => if x < 0.0 {
                    normal.clone()
                 } else {
                     -normal
                 }
             }; // 交差位置の法線（物体からのレイの入出を考慮。
        let into: bool = Vec::dot(normal, &now_normal) > 0.0; // レイがオブジェクトから出るのか、入るのか。
        let n1: f64 = 1.0; // 真空の屈折率
        let n2: &f64 = &self.ior; // オブジェクトの屈折率
        let n: f64 = if into {
                n1 / n2
             } else {
                n2 / n1
             };

        // Snellの法則を用いて屈折方向を計算する。
        let dir_dot_normal: f64 = Vec::dot(input, &now_normal);
        let cos2t_2: f64 = 1.0 - &n * &n * (1.0 - &dir_dot_normal * &dir_dot_normal);
        
        // 全反射
        let reflection_dir: Vec = Vec::reflect(input, &now_normal);
        if &cos2t_2 < &0.0 {
            // pdfはディラックのδ関数なので実数値にはならないが、将来的にモンテカルロ積分において、
            // 分母と分子の両方にδが表れるため結局打ち消し合うため、1でよい。あくまでδであること忘れないためにDELTAを入れておくが、実態は1。
            *pdf = DELTA;
            *brdf_value = self.eval(input, normal, &reflection_dir);

            return reflection_dir
        }

        // 屈折の方向
        let refraction_dir: Vec = input * &n - &now_normal * (&dir_dot_normal * &n + cos2t_2.sqrt());
        
        // Fresnelの式
        let cost_1: f64 = Vec::dot(&-input, &now_normal);
        let cost_2: f64 = cos2t_2.sqrt();
        let r_parallel: f64 = (&n * &cost_1 - &cost_2) / (&n * &cost_1 + &cost_2);
        let r_perpendicular: f64 = (&cost_1 - &n * &cost_2) / (&cost_1 + &n * &cost_2);
        let fr: f64 = &0.5 * (&r_parallel * &r_parallel + &r_perpendicular * &r_perpendicular);

        let factor: f64 = (if into {
            n1 / n2
        } else {
            n2 / n1
        }).powf(2.0); // レイの運ぶ放射輝度は屈折率の異なる物体間を移動するとき、屈折率の比の二乗の分だけ変化する。
        let ft: f64 = (&1.0 - &fr) * &factor; // 屈折方向の割合。
        
        // ロシアンルーレットで屈折か反射かを決定する。
        // ロシアンルーレットの確率は反射率ということしておく。
        let probability: f64 = fr;
        if &random.next01() < &probability { // 反射
            // pdfはディラックのδ関数なので実数値にはならないが、将来的にモンテカルロ積分において、
            // 分母と分子の両方にδが表れるため結局打ち消し合うため、1でよい。あくまでδであること忘れないためにDELTAを入れておくが、実態は1。
            *pdf = DELTA * &probability;
            *brdf_value = &fr * self.eval(input, normal, &reflection_dir);

            return reflection_dir;
        } else { // 屈折
            *pdf = DELTA * (&1.0 - &probability);
            *brdf_value = &ft * self.eval(input, normal, &reflection_dir);

            refraction_dir
        }
    }
}

impl Lightsource {
    pub fn new(emission: Color) -> Lightsource {
        Lightsource {
            emission: emission,
            reflectance: Color { x: 0.0, y: 0.0, z: 0.0 },
        }
    }

    fn eval_pdf(&self, input: &Vec, normal: &Vec, output: &Vec) -> f64 {
        assert!(false);
        -1.0
    }
}

impl MaterialTrait for Lightsource {
    fn emission(&self) -> &Color {
        &self.emission
    }

    fn reflectance(&self) -> &Color {
        &self.reflectance
    }

    fn eval(&self, input: &Vec, normal: &Vec, output: &Vec) -> Color {
        assert!(false);
        Color { x: 0.0, y: 0.0, z: 0.0}
    }

    fn sample(&self, random: &mut Random, input: &Vec, normal: &Vec, pdf: &mut f64, brdf_value: &mut Color) -> Vec {
        assert!(false);
        Color { x: 0.0, y: 0.0, z: 0.0}
    }
}

