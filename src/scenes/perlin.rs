use crate::camera::{CameraSave, Lens};
use crate::config::ConfigSave;
use crate::data::colour::Colour;
use crate::data::vector::Vector;
use crate::world::background::Background;
use crate::world::geometry::sphere::Sphere;
use crate::world::geometry::Geometry;
use crate::world::materials::Material;
use crate::world::texture::perlin::build_noise_config;
use crate::world::texture::Texture;
use crate::world::WorldSave;

pub fn build() -> Result<ConfigSave, anyhow::Error> {
    let aspect = 1.5;

    let camera = CameraSave::new(
        &Vector::new(13.0, 2.0, 3.0),
        &Vector::new(0.0, 0.0, 0.0),
        &Vector::new(0.0, 1.0, 0.0),
        aspect,
        Lens::new(20.0, 0.0, 10.0),
        0.0,
        1.0,
    );

    let mut geometries: Vec<Geometry> = Vec::with_capacity(2);

    geometries.push(Sphere::build(
        Vector::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian {
            albedo: Texture::Noise {
                base_colour: Colour::new(1.0, 1.0, 1.0),
                scale: 5.0,
                noisiness: 10.0,
                noise_config: build_noise_config(),
            },
        },
    ));
    geometries.push(Sphere::build(
        Vector::new(0.0, 2.0, 0.0),
        2.0,
        Material::Lambertian {
            albedo: Texture::Noise {
                base_colour: Colour::new(1.0, 1.0, 1.0),
                scale: 5.0,
                noisiness: 10.0,
                noise_config: build_noise_config(),
            },
        },
    ));

    let white = Colour::new(1.0, 1.0, 1.0);
    let blue = Colour::new(0.5, 0.7, 1.0);
    let background = Background::new(white, blue);

    let world = WorldSave::new(background, geometries);

    Ok(ConfigSave::new(aspect, camera, world))
}
