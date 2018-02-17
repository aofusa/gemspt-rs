#![allow(dead_code)]

use vec::Vec;
use sphere::Sphere;
use material::{Material, LambertianMaterial, PhongMaterial, GlassMaterial, Lightsource};
use ray::Ray;
use hitpoint::Hitpoint;

pub type Scene = [Box<SceneSphere<Box<Material>>>; 8];
type Color = Vec;

pub enum SceneRendering {
    SceneDiffuseOnly,
    SceneSpecular,
    SceneGlass,
}

#[derive(Debug)]
pub struct SceneSphere<T> {
    sphere_: Sphere,
    material_: T,
}

impl<T> SceneSphere<T> {
    pub fn new(sphere: Sphere, material: T) -> SceneSphere<T> {
        SceneSphere { sphere_: sphere, material_: material }
    }

    pub fn get_sphere(&self) -> &Sphere {
        &self.sphere_
    }

    pub fn get_material(&self) -> &T {
        &self.material_
    }
}

pub fn generate_scene(mode: SceneRendering) -> Scene {
    match mode {
        SceneRendering::SceneDiffuseOnly =>
            [
                Box::new(
                    SceneSphere::new (
                        Sphere::new (
                            100000.0,
                            Vec { x: 0.0, y: -100000.0, z: 0.0 }
                        ),
                        Box::new(
                            LambertianMaterial::new (
                                Color { x: 0.7, y: 0.7, z: 0.7 }
                            )
                        )
                    )
                ),
                Box::new(
                    SceneSphere::new (
                        Sphere::new (
                            100000.0,
                            Vec { x: 0.0, y:  100004.0, z: 0.0 }
                        ),
                        Box::new(
                            LambertianMaterial::new (
                                Color { x: 0.7, y: 0.7, z: 0.7 }
                            )
                        )
                    )
                ),
                Box::new(
                    SceneSphere::new (
                        Sphere::new (
                            100000.0,
                            Vec { x: -100003.0, y: 0.0, z: 0.0 }
                        ),
                        Box::new(
                            LambertianMaterial::new (
                                Color { x: 0.7, y: 0.1, z: 0.1 }
                            )
                        )
                    )
                ),
                Box::new(
                    SceneSphere::new (
                        Sphere::new (
                            100000.0,
                            Vec { x: 10009.0, y: 0.0, z: 0.0 }
                        ),
                        Box::new(
                            LambertianMaterial::new (
                                Color { x: 0.7, y: 0.7, z: 0.7 }
                            )
                        )
                    )
                ),
                Box::new(
                    SceneSphere::new (
                        Sphere::new (
                            100000.0,
                            Vec { x: 0.0, y: 0.0, z: -100003.0 }
                        ),
                        Box::new(
                            LambertianMaterial::new (
                                Color { x: 0.1, y: 0.7, z: 0.1 }
                            )
                        )
                    )
                ),
                Box::new(
                    SceneSphere::new (
                        Sphere::new (
                            100.0,
                            Vec { x: 0.0, y: 103.99, z: 0.0 }
                        ),
                        Box::new(
                            Lightsource::new (
                                Color { x: 8.0, y: 8.0, z: 8.0 }
                            )
                        )
                    )
                ),
                Box::new(
                    SceneSphere::new (
                        Sphere::new (
                            1.0,
                            Vec { x: -2.0, y: 1.0, z: 0.0 }
                        ),
                        Box::new(
                            LambertianMaterial::new (
                                Color { x: 0.7, y: 0.7, z: 0.7 }
                            )
                        )
                    )
                ),
                Box::new(
                    SceneSphere::new (
                        Sphere::new (
                            1.0,
                            Vec { x: 2.0, y: 1.0, z: 0.0 }
                        ),
                        Box::new(
                            LambertianMaterial::new (
                                Color { x: 0.1, y: 0.1, z: 0.7 }
                            )
                        )
                    )
                ),
            ],
        SceneRendering::SceneSpecular =>
            [
                Box::new(
                    SceneSphere::new (
                        Sphere::new (
                            100000.0,
                            Vec { x: 0.0, y: -100000.0, z: 0.0 }
                        ),
                        Box::new(
                            PhongMaterial::new (
                                Color { x: 0.999, y: 0.999, z: 0.999 },
                                100.0
                            )
                        )
                    )
                ),
                Box::new(
                    SceneSphere::new (
                        Sphere::new (
                            100000.0,
                            Vec { x: 0.0, y:  100004.0, z: 0.0 }
                        ),
                        Box::new(
                            LambertianMaterial::new (
                                Color { x: 0.7, y: 0.7, z: 0.7 }
                            )
                        )
                    )
                ),
                Box::new(
                    SceneSphere::new (
                        Sphere::new (
                            100000.0,
                            Vec { x: -100003.0, y: 0.0, z: 0.0 }
                        ),
                        Box::new(
                            LambertianMaterial::new (
                                Color { x: 0.7, y: 0.1, z: 0.1 }
                            )
                        )
                    )
                ),
                Box::new(
                    SceneSphere::new (
                        Sphere::new (
                            100000.0,
                            Vec { x: 10009.0, y: 0.0, z: 0.0 }
                        ),
                        Box::new(
                            LambertianMaterial::new (
                                Color { x: 0.7, y: 0.7, z: 0.7 }
                            )
                        )
                    )
                ),
                Box::new(
                    SceneSphere::new (
                        Sphere::new (
                            100000.0,
                            Vec { x: 0.0, y: 0.0, z: -100003.0 }
                        ),
                        Box::new(
                            LambertianMaterial::new (
                                Color { x: 0.1, y: 0.7, z: 0.1 }
                            )
                        )
                    )
                ),
                Box::new(
                    SceneSphere::new (
                        Sphere::new (
                            100.0,
                            Vec { x: 0.0, y: 103.99, z: 0.0 }
                        ),
                        Box::new(
                            Lightsource::new (
                                Color { x: 8.0, y: 8.0, z: 8.0 }
                            )
                        )
                    )
                ),
                Box::new(
                    SceneSphere::new (
                        Sphere::new (
                            1.0,
                            Vec { x: -2.0, y: 1.0, z: 0.0 }
                        ),
                        Box::new(
                            LambertianMaterial::new (
                                Color { x: 0.7, y: 0.7, z: 0.7 }
                            )
                        )
                    )
                ),
                Box::new(
                    SceneSphere::new (
                        Sphere::new (
                            1.0,
                            Vec { x: 2.0, y: 1.0, z: 0.0 }
                        ),
                        Box::new(
                            LambertianMaterial::new (
                                Color { x: 0.1, y: 0.1, z: 0.7 }
                            )
                        )
                    )
                ),
            ],
        SceneRendering::SceneGlass =>
            [
                Box::new(
                    SceneSphere::new (
                        Sphere::new (
                            100000.0,
                            Vec { x: 0.0, y: -100000.0, z: 0.0 }
                        ),
                        Box::new(
                            LambertianMaterial::new (
                                Color { x: 0.7, y: 0.7, z: 0.7 }
                            )
                        )
                    )
                ),
                Box::new(
                    SceneSphere::new (
                        Sphere::new (
                            100000.0,
                            Vec { x: 0.0, y:  100004.0, z: 0.0 }
                        ),
                        Box::new(
                            LambertianMaterial::new (
                                Color { x: 0.7, y: 0.7, z: 0.7 }
                            )
                        )
                    )
                ),
                Box::new(
                    SceneSphere::new (
                        Sphere::new (
                            100000.0,
                            Vec { x: -100003.0, y: 0.0, z: 0.0 }
                        ),
                        Box::new(
                            LambertianMaterial::new (
                                Color { x: 0.7, y: 0.1, z: 0.1 }
                            )
                        )
                    )
                ),
                Box::new(
                    SceneSphere::new (
                        Sphere::new (
                            100000.0,
                            Vec { x: 10009.0, y: 0.0, z: 0.0 }
                        ),
                        Box::new(
                            LambertianMaterial::new (
                                Color { x: 0.7, y: 0.7, z: 0.7 }
                            )
                        )
                    )
                ),
                Box::new(
                    SceneSphere::new (
                        Sphere::new (
                            100000.0,
                            Vec { x: 0.0, y: 0.0, z: -100003.0 }
                        ),
                        Box::new(
                            LambertianMaterial::new (
                                Color { x: 0.1, y: 0.7, z: 0.1 }
                            )
                        )
                    )
                ),
                Box::new(
                    SceneSphere::new (
                        Sphere::new (
                            100.0,
                            Vec { x: 0.0, y: 103.99, z: 0.0 }
                        ),
                        Box::new(
                            Lightsource::new (
                                Color { x: 8.0, y: 8.0, z: 8.0 }
                            )
                        )
                    )
                ),
                Box::new(
                    SceneSphere::new (
                        Sphere::new (
                            1.0,
                            Vec { x: -2.0, y: 1.0, z: 0.0 }
                        ),
                        Box::new(
                            LambertianMaterial::new (
                                Color { x: 0.7, y: 0.7, z: 0.7 }
                            )
                        )
                    )
                ),
                Box::new(
                    SceneSphere::new (
                        Sphere::new (
                            1.0,
                            Vec { x: 2.0, y: 1.0, z: 0.0 }
                        ),
                        Box::new(
                            GlassMaterial::new (
                                Color { x: 0.999999, y: 0.999999, z: 0.999999 },
                                1.5
                            )
                        )
                    )
                ),
            ],
        }
}

// シーンとの交差判定関数。
pub fn intersect_scene<'a>(scene: &'a Scene, ray: &'a Ray) -> (Option<&'a SceneSphere<Box<Material>>>, Hitpoint) {
    // 初期化
    let mut hitpoint = Hitpoint::new();
    let mut now_object: Option<&SceneSphere<Box<Material>>> = None;

    // 線形探索
    for object in scene.iter() {
        let mut tmp_hitpoint = Hitpoint::new();
        if object.get_sphere().intersect(&ray, &mut tmp_hitpoint) {
            if &tmp_hitpoint.distance < &hitpoint.distance {
                hitpoint = tmp_hitpoint;
                now_object = Some(object);
            }
        }
    }

    (now_object, hitpoint)
}

