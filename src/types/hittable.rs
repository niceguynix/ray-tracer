use std::rc::Rc;

use crate::materials::lambertian::Lambertian;

use super::{
    ray::Ray,
    vector::{Point3, Vec3},
    material::Material, color::Color,
};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr:Rc<dyn Material>,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.dir.dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

impl HitRecord {
    pub fn default() -> Self {
        HitRecord {
            p: Vec3::default(),
            normal: Vec3::default(),
            mat_ptr:Rc::new(Lambertian{albedo:Color::new(1.0,1.0,1.0)}),
            t: 0.0,
            front_face: true,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}
