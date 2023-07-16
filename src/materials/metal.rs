use crate::types::{
    color::Color, material::Material, ray::Ray, rtweekend::random_in_unit_sphere, vector::Vec3,
};

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &crate::types::ray::Ray,
        rec: &crate::types::hittable::HitRecord,
        attenuation: &mut Color,
        scattered: &mut crate::types::ray::Ray,
    ) -> bool {
        let reflected = Vec3::reflect(&r_in.dir.normal(), &rec.normal);
        let t = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere());
        scattered.dir = t.dir;
        scattered.orig = t.orig;

        attenuation.x = self.albedo.x;
        attenuation.y = self.albedo.y;
        attenuation.z = self.albedo.z;

        scattered.dir.dot(&rec.normal) > 0.0
    }
}
