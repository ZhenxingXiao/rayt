pub mod axis_aligned_bounding_box;
pub mod bounding_volume_hierarchy;
pub mod cube;
pub mod flip_normals;
pub mod medium;
pub mod rectangle;
pub mod rotate;
pub mod sphere;
pub mod translate;

use crate::camera::Ray;
use crate::data::assets::Assets;
use crate::data::vector::Vector;
use crate::world::geometry::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::world::geometry::bounding_volume_hierarchy::BoundingVolumeHierarchyNode;
use crate::world::geometry::cube::Cube;
use crate::world::geometry::flip_normals::FlipNormals;
use crate::world::geometry::medium::ConstantMedium;
use crate::world::geometry::rectangle::{XyRect, XzRect, YzRect};
use crate::world::geometry::rotate::RotateY;
use crate::world::geometry::sphere::{MovingSphere, Sphere};
use crate::world::geometry::translate::Translate;
use crate::world::materials::Material;
use anyhow::Error;
use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Geometry {
    Bvh(Box<BoundingVolumeHierarchyNode>),
    Sphere(Box<Sphere>),
    MovingSphere(Box<MovingSphere>),
    Cube(Box<Cube>),
    ConstantMedium(Box<ConstantMedium>),
    XyRect(Box<XyRect>),
    XzRect(Box<XzRect>),
    YzRect(Box<YzRect>),
    Flip(Box<FlipNormals>),
    Translate(Box<Translate>),
    RotateY(Box<RotateY>),
}

impl Hittable for Geometry {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitResult> {
        match self {
            Geometry::Bvh(inner) => inner.hit(ray, tmin, tmax),
            Geometry::Sphere(inner) => inner.hit(ray, tmin, tmax),
            Geometry::MovingSphere(inner) => inner.hit(ray, tmin, tmax),
            Geometry::Cube(inner) => inner.hit(ray, tmin, tmax),
            Geometry::ConstantMedium(inner) => inner.hit(ray, tmin, tmax),
            Geometry::XyRect(inner) => inner.hit(ray, tmin, tmax),
            Geometry::XzRect(inner) => inner.hit(ray, tmin, tmax),
            Geometry::YzRect(inner) => inner.hit(ray, tmin, tmax),
            Geometry::Flip(inner) => inner.hit(ray, tmin, tmax),
            Geometry::Translate(inner) => inner.hit(ray, tmin, tmax),
            Geometry::RotateY(inner) => inner.hit(ray, tmin, tmax),
        }
    }

    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AxisAlignedBoundingBox> {
        match self {
            Geometry::Bvh(inner) => inner.bounding_box(time_start, time_end),
            Geometry::Sphere(inner) => inner.bounding_box(time_start, time_end),
            Geometry::MovingSphere(inner) => inner.bounding_box(time_start, time_end),
            Geometry::Cube(inner) => inner.bounding_box(time_start, time_end),
            Geometry::ConstantMedium(inner) => inner.bounding_box(time_start, time_end),
            Geometry::XyRect(inner) => inner.bounding_box(time_start, time_end),
            Geometry::XzRect(inner) => inner.bounding_box(time_start, time_end),
            Geometry::YzRect(inner) => inner.bounding_box(time_start, time_end),
            Geometry::Flip(inner) => inner.bounding_box(time_start, time_end),
            Geometry::Translate(inner) => inner.bounding_box(time_start, time_end),
            Geometry::RotateY(inner) => inner.bounding_box(time_start, time_end),
        }
    }

    fn validate(&self, assets: &Assets) -> Result<(), Error> {
        match self {
            Geometry::Bvh(inner) => inner.validate(assets),
            Geometry::Sphere(inner) => inner.validate(assets),
            Geometry::MovingSphere(inner) => inner.validate(assets),
            Geometry::Cube(inner) => inner.validate(assets),
            Geometry::ConstantMedium(inner) => inner.validate(assets),
            Geometry::XyRect(inner) => inner.validate(assets),
            Geometry::XzRect(inner) => inner.validate(assets),
            Geometry::YzRect(inner) => inner.validate(assets),
            Geometry::Flip(inner) => inner.validate(assets),
            Geometry::Translate(inner) => inner.validate(assets),
            Geometry::RotateY(inner) => inner.validate(assets),
        }
    }

    fn is_attractor(&self) -> bool {
        match self {
            Geometry::Bvh(inner) => inner.is_attractor(),
            Geometry::Sphere(inner) => inner.is_attractor(),
            Geometry::MovingSphere(inner) => inner.is_attractor(),
            Geometry::Cube(inner) => inner.is_attractor(),
            Geometry::ConstantMedium(inner) => inner.is_attractor(),
            Geometry::XyRect(inner) => inner.is_attractor(),
            Geometry::XzRect(inner) => inner.is_attractor(),
            Geometry::YzRect(inner) => inner.is_attractor(),
            Geometry::Flip(inner) => inner.is_attractor(),
            Geometry::Translate(inner) => inner.is_attractor(),
            Geometry::RotateY(inner) => inner.is_attractor(),
        }
    }

    fn pdf_value(&self, origin: &Vector, direction: &Vector) -> f64 {
        match self {
            Geometry::Bvh(inner) => inner.pdf_value(origin, direction),
            Geometry::Sphere(inner) => inner.pdf_value(origin, direction),
            Geometry::MovingSphere(inner) => inner.pdf_value(origin, direction),
            Geometry::Cube(inner) => inner.pdf_value(origin, direction),
            Geometry::ConstantMedium(inner) => inner.pdf_value(origin, direction),
            Geometry::XyRect(inner) => inner.pdf_value(origin, direction),
            Geometry::XzRect(inner) => inner.pdf_value(origin, direction),
            Geometry::YzRect(inner) => inner.pdf_value(origin, direction),
            Geometry::Flip(inner) => inner.pdf_value(origin, direction),
            Geometry::Translate(inner) => inner.pdf_value(origin, direction),
            Geometry::RotateY(inner) => inner.pdf_value(origin, direction),
        }
    }

    fn random(&self, origin: &Vector) -> Vector {
        match self {
            Geometry::Bvh(inner) => inner.random(origin),
            Geometry::Sphere(inner) => inner.random(origin),
            Geometry::MovingSphere(inner) => inner.random(origin),
            Geometry::Cube(inner) => inner.random(origin),
            Geometry::ConstantMedium(inner) => inner.random(origin),
            Geometry::XyRect(inner) => inner.random(origin),
            Geometry::XzRect(inner) => inner.random(origin),
            Geometry::YzRect(inner) => inner.random(origin),
            Geometry::Flip(inner) => inner.random(origin),
            Geometry::Translate(inner) => inner.random(origin),
            Geometry::RotateY(inner) => inner.random(origin),
        }
    }
}

impl Geometry {
    pub fn flip(self) -> Geometry {
        FlipNormals::build(self)
    }

    pub fn translate(self, offset: Vector) -> Geometry {
        Translate::build(self, offset)
    }

    pub fn rotate_y(self, angle: f64) -> Result<Geometry, anyhow::Error> {
        let rotate = RotateY::build(self, angle)?;
        Ok(rotate)
    }
}

pub trait Hittable: Debug {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitResult>;

    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AxisAlignedBoundingBox>;

    fn validate(&self, assets: &Assets) -> Result<(), anyhow::Error>;

    fn is_attractor(&self) -> bool;

    fn pdf_value(&self, _origin: &Vector, _direction: &Vector) -> f64 {
        unimplemented!("{:?} is not implemented as an attractor", self)
    }

    fn random(&self, _origin: &Vector) -> Vector {
        unimplemented!("{:?} is not implemented as an attractor", self)
    }
}

#[derive(Debug, Clone)]
pub struct HitResult {
    pub distance: f64,
    pub ray: Ray,
    pub point: Vector,
    pub surface_normal: Vector,
    pub material: Material,
    pub texture_coords: (f64, f64),
}

impl HitResult {
    pub fn front_face(&self) -> bool {
        Vector::dot(self.ray.direction(), &self.surface_normal) < 0.0
    }

    pub fn face_normal(&self) -> Vector {
        if self.front_face() {
            self.surface_normal
        } else {
            -self.surface_normal
        }
    }
}

impl Ord for HitResult {
    fn cmp(&self, other: &HitResult) -> Ordering {
        // We should never get a NaN here. Panic if we do
        self.distance.partial_cmp(&other.distance).unwrap()
    }
}

impl PartialOrd for HitResult {
    fn partial_cmp(&self, other: &HitResult) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HitResult {
    fn eq(&self, other: &HitResult) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for HitResult {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hit_result_eq() {
        let hit_result = HitResult {
            distance: 0.0,
            ray: Ray::new(Vector::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0), 0.0),
            point: Vector::new(0.0, 0.0, 0.0),
            surface_normal: Vector::new(0.0, 0.0, 0.0),
            material: Material::Dielectric {
                refractive_index: 1.5,
            },
            texture_coords: (1.0, 0.5),
        };
        assert_eq!(hit_result.clone(), hit_result.clone());

        let other_hit_result = HitResult {
            distance: 1.0,
            ray: Ray::new(Vector::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0), 0.0),
            point: Vector::new(0.0, 0.0, 0.0),
            surface_normal: Vector::new(0.0, 0.0, 0.0),
            material: Material::Dielectric {
                refractive_index: 1.5,
            },
            texture_coords: (1.0, 0.5),
        };
        assert_ne!(hit_result, other_hit_result);
    }

    #[test]
    fn test_hit_result_ord() {
        let hit_result = HitResult {
            distance: 0.0,
            ray: Ray::new(Vector::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0), 0.0),
            point: Vector::new(0.0, 0.0, 0.0),
            surface_normal: Vector::new(0.0, 0.0, 0.0),
            material: Material::Dielectric {
                refractive_index: 1.5,
            },
            texture_coords: (1.0, 0.5),
        };
        let other_hit_result = HitResult {
            distance: 1.0,
            ray: Ray::new(Vector::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0), 0.0),
            point: Vector::new(0.0, 0.0, 0.0),
            surface_normal: Vector::new(0.0, 0.0, 0.0),
            material: Material::Dielectric {
                refractive_index: 1.5,
            },
            texture_coords: (1.0, 0.5),
        };
        assert!(other_hit_result > hit_result);
        assert!(hit_result < other_hit_result);
    }
}
