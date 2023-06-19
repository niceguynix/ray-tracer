use crate::types::{color::Color, material::Material, rtweekend::random_unit_vector, ray::Ray};

pub struct Lambertian{
    pub albedo:Color
}

impl Material for Lambertian{
    fn scatter(&self,r_in:&crate::types::ray::Ray, rec:&crate::types::hittable::HitRecord,attenuation:&mut Color,scattered:&mut crate::types::ray::Ray)->bool {
        let mut scatter_direction= rec.normal + random_unit_vector();

        if scatter_direction.near_zero(){
            scatter_direction=rec.normal;
        }

        let t = Ray::new(rec.p,scatter_direction);
        scattered.dir=t.dir;
        scattered.orig=t.orig;

        attenuation.x =self.albedo.x;
        attenuation.y =self.albedo.y;
        attenuation.z =self.albedo.z;

        // eprintln!("{} {} {}",self.albedo.x,attenuation.y,attenuation.z);


        true 
    }
}