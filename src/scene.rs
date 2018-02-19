#![allow(dead_code)]

use vec::Vec;
use sphere::Sphere;
use material::{Material, LambertianMaterial, PhongMaterial, GlassMaterial, Lightsource};
use ray::Ray;
use hitpoint::Hitpoint;

pub type Scene = [SceneSphere; 8];
type Color = Vec;

pub enum SceneRendering {
    SceneDiffuseOnly,
    SceneSpecular,
    SceneGlass,
}

pub struct SceneSphere {
    sphere: Sphere,
    material: Material,
}

impl SceneSphere {
    pub fn new(sphere: Sphere, material: Material) -> SceneSphere {
        SceneSphere { sphere: sphere, material: material }
    }

    pub fn get_sphere(&self) -> &Sphere {
        &self.sphere
    }

    pub fn get_material(&self) -> &Material {
        &self.material
    }
}

pub fn generate_scene(mode: SceneRendering) -> Scene {
    match mode {
        SceneRendering::SceneDiffuseOnly =>
            [
                SceneSphere::new (
                    Sphere::new (
                        100000.0,
                        Vec { x: 0.0, y: -100000.0, z: 0.0 }
                    ),
                    Material::LambertianMaterial (
                        LambertianMaterial::new (Color { x: 0.7, y: 0.7, z: 0.7 })
                    )
                ),
                SceneSphere::new (
                    Sphere::new (
                        100000.0,
                        Vec { x: 0.0, y:  100004.0, z: 0.0 }
                    ),
                    Material::LambertianMaterial (
                        LambertianMaterial::new (Color { x: 0.7, y: 0.7, z: 0.7 })
                    )
                ),
                SceneSphere::new (
                    Sphere::new (
                        100000.0,
                        Vec { x: -100003.0, y: 0.0, z: 0.0 }
                    ),
                    Material::LambertianMaterial (
                        LambertianMaterial::new (Color { x: 0.7, y: 0.1, z: 0.1 })
                    )
                ),
                SceneSphere::new (
                    Sphere::new (
                        100000.0,
                        Vec { x: 10009.0, y: 0.0, z: 0.0 }
                    ),
                    Material::LambertianMaterial (
                        LambertianMaterial::new (Color { x: 0.7, y: 0.7, z: 0.7 })
                    )
                ),
                SceneSphere::new (
                    Sphere::new (
                        100000.0,
                        Vec { x: 0.0, y: 0.0, z: -100003.0 }
                    ),
                    Material::LambertianMaterial (
                        LambertianMaterial::new (Color { x: 0.7, y: 0.7, z: 0.1 })
                    )
                ),
                SceneSphere::new (
                    Sphere::new (
                        100.0,
                        Vec { x: 0.0, y: 103.99, z: 0.0 }
                    ),
                    Material::Lightsource (
                        Lightsource::new (Color { x: 8.0, y: 8.0, z: 8.0 })
                    )
                ),
                SceneSphere::new (
                    Sphere::new (
                        1.0,
                        Vec { x: -2.0, y: 1.0, z: 0.0 }
                    ),
                    Material::LambertianMaterial (
                        LambertianMaterial::new (Color { x: 0.7, y: 0.7, z: 0.7 })
                    )
                ),
                SceneSphere::new (
                    Sphere::new (
                        1.0,
                        Vec { x: 2.0, y: 1.0, z: 0.0 }
                    ),
                    Material::LambertianMaterial (
                        LambertianMaterial::new (Color { x: 0.1, y: 0.1, z: 0.7 })
                    )
                ),
            ],
        SceneRendering::SceneSpecular =>
            [
                SceneSphere::new (
                    Sphere::new (
                        100000.0,
                        Vec { x: 0.0, y: -100000.0, z: 0.0 }
                    ),
                    Material::PhongMaterial (
                        PhongMaterial::new (
                            Color { x: 0.999, y: 0.999, z: 0.999 },
                            100.0
                        )
                    )
                ),
                SceneSphere::new (
                    Sphere::new (
                        100000.0,
                        Vec { x: 0.0, y:  100004.0, z: 0.0 }
                    ),
                    Material::LambertianMaterial (
                        LambertianMaterial::new (Color { x: 0.7, y: 0.7, z: 0.7 })
                    )
                ),
                SceneSphere::new (
                    Sphere::new (
                        100000.0,
                        Vec { x: -100003.0, y: 0.0, z: 0.0 }
                    ),
                    Material::LambertianMaterial (
                        LambertianMaterial::new (Color { x: 0.7, y: 0.1, z: 0.1 })
                    )
                ),
                SceneSphere::new (
                    Sphere::new (
                        100000.0,
                        Vec { x: 10009.0, y: 0.0, z: 0.0 }
                    ),
                    Material::LambertianMaterial (
                        LambertianMaterial::new (Color { x: 0.7, y: 0.7, z: 0.7 })
                    )
                ),
                SceneSphere::new (
                    Sphere::new (
                        100000.0,
                        Vec { x: 0.0, y: 0.0, z: -100003.0 }
                    ),
                    Material::LambertianMaterial (
                        LambertianMaterial::new (Color { x: 0.1, y: 0.7, z: 0.1 })
                    )
                ),
                SceneSphere::new (
                    Sphere::new (
                        100.0,
                        Vec { x: 0.0, y: 103.99, z: 0.0 }
                    ),
                    Material::Lightsource (
                        Lightsource::new (Color { x: 8.0, y: 8.0, z: 8.0 })
                    )
                ),
                SceneSphere::new (
                    Sphere::new (
                        1.0,
                        Vec { x: -2.0, y: 1.0, z: 0.0 }
                    ),
                    Material::LambertianMaterial (
                        LambertianMaterial::new (Color { x: 0.7, y: 0.7, z: 0.7 })
                    )
                ),
                SceneSphere::new (
                    Sphere::new (
                        1.0,
                        Vec { x: 2.0, y: 1.0, z: 0.0 }
                    ),
                    Material::LambertianMaterial (
                        LambertianMaterial::new (Color { x: 0.1, y: 0.1, z: 0.7 })
                    )
                ),
            ],
        SceneRendering::SceneGlass =>
            [
                SceneSphere::new (
                    Sphere::new (
                        100000.0,
                        Vec { x: 0.0, y: -100000.0, z: 0.0 }
                    ),
                    Material::LambertianMaterial (
                        LambertianMaterial::new (Color { x: 0.7, y: 0.7, z: 0.7 })
                    )
                ),
                SceneSphere::new (
                    Sphere::new (
                        100000.0,
                        Vec { x: 0.0, y:  100004.0, z: 0.0 }
                    ),
                    Material::LambertianMaterial (
                        LambertianMaterial::new (Color { x: 0.7, y: 0.7, z: 0.7 })
                    )
                ),
                SceneSphere::new (
                    Sphere::new (
                        100000.0,
                        Vec { x: -100003.0, y: 0.0, z: 0.0 }
                    ),
                    Material::LambertianMaterial (
                        LambertianMaterial::new (Color { x: 0.7, y: 0.1, z: 0.1 })
                    )
                ),
                SceneSphere::new (
                    Sphere::new (
                        100000.0,
                        Vec { x: 10009.0, y: 0.0, z: 0.0 }
                    ),
                    Material::LambertianMaterial (
                        LambertianMaterial::new (Color { x: 0.7, y: 0.7, z: 0.7 })
                    )
                ),
                SceneSphere::new (
                    Sphere::new (
                        100000.0,
                        Vec { x: 0.0, y: 0.0, z: -100003.0 }
                    ),
                    Material::LambertianMaterial (
                        LambertianMaterial::new (Color { x: 0.1, y: 0.7, z: 0.1 })
                    )
                ),
                SceneSphere::new (
                    Sphere::new (
                        100.0,
                        Vec { x: 0.0, y: 103.99, z: 0.0 }
                    ),
                    Material::Lightsource (
                        Lightsource::new (Color { x: 8.0, y: 8.0, z: 8.0 })
                    )
                ),
                SceneSphere::new (
                    Sphere::new (
                        1.0,
                        Vec { x: -2.0, y: 1.0, z: 0.0 }
                    ),
                    Material::LambertianMaterial (
                        LambertianMaterial::new (Color { x: 0.7, y: 0.7, z: 0.7 })
                    )
                ),
                SceneSphere::new (
                    Sphere::new (
                        1.0,
                        Vec { x: 2.0, y: 1.0, z: 0.0 }
                    ),
                    Material::GlassMaterial (
                        GlassMaterial::new (
                            Color { x: 0.999999, y: 0.999999, z: 0.999999 },
                            1.5
                        )
                    )
                ),
            ],
        }
}

// シーンとの交差判定関数。
pub fn intersect_scene<'a>(scene: &'a Scene, ray: &'a Ray) -> (Option<&'a SceneSphere>, Hitpoint) {
    // 初期化
    let mut hitpoint = Hitpoint::new();
    let mut now_object: Option<&SceneSphere> = None;

    // 線形探索
    for object in scene.iter() {
        if let Some(hit) = object.get_sphere().intersect(&ray) {
            if &hit.distance < &hitpoint.distance {
                hitpoint = hit;
                now_object = Some(object);
            }
        }
    }

    (now_object, hitpoint)
}

