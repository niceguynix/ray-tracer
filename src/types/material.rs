use super::{ray::Ray, hittable::HitRecord, color::Color};

pub trait Material{
    fn scatter(&self,r_in:&Ray, rec:&HitRecord,attenuation:&mut Color,scattered:&mut Ray)->bool;
}