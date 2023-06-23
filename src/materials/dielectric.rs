use rand::random;

use crate::types::{color::Color, material::Material, rtweekend::{random_unit_vector, refract}, ray::Ray, vector::Vec3};

pub struct Dielectric{
    pub ir:f32
}

impl Material for Dielectric{
    fn scatter(&self,r_in:&crate::types::ray::Ray, rec:&crate::types::hittable::HitRecord,attenuation:&mut Color,scattered:&mut crate::types::ray::Ray)->bool {
        attenuation.x=1.0;
        attenuation.y=1.0;
        attenuation.z=1.0;
        
        let refraction_ratio = if rec.front_face {1.0/self.ir} else {self.ir};

        let unit_direction = r_in.dir.normal();
        let refracted = refract( &unit_direction , &rec.normal , refraction_ratio);

        let cos_theta = f32::min((-unit_direction).dot(&rec.normal),1.0);
        let sin_theta= f32::sqrt(1.0 - cos_theta*cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction ;

        if cannot_refract || reflectance(cos_theta,refraction_ratio)>random(){
            direction=  Vec3::reflect(&unit_direction,&rec.normal);
        }else{
            direction = refract(&unit_direction,&rec.normal,refraction_ratio);
        }

        let t = &mut Ray::new(rec.p,direction);
        scattered.dir=t.dir;
        scattered.orig=t.orig;
        true
    }

}

    
    pub fn reflectance(cosine:f32,ref_idx:f32)->f32{
        let mut r0 = (1.0-ref_idx)/(1.0+ref_idx);
        r0=r0*r0;

        r0 + (1.0-r0)* f32::powi((1.0-cosine),5)
    }