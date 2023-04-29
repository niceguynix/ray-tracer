use crate::types::vector::Point3;
use crate::types::hittable::Hittable;

pub struct Sphere{
    pub center:Point3,
    pub radius:f32,
}

impl Hittable for Sphere{
    fn hit(&self,r:&crate::types::ray::Ray,t_min:f32,t_max:f32,rec:&mut crate::types::hittable::HitRecord)->bool {
        let oc = r.orig - self.center;
        let a = r.dir.dot(&r.dir);
        let b = 2.0 * oc.dot(&r.dir);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        
        if discriminant < 0.0{ return false};

        let mut root = (-b - f32::sqrt(discriminant)) / (2.0 * a);
        
        if root<t_min || root>t_max{
            root = (-b + f32::sqrt(discriminant)) / (2.0 * a);
            if root<t_min || root>t_max{
                return false;
            }
        }

        rec.t=root;
        rec.p=r.at(rec.t);
        let normal=(rec.p-self.center) / self.radius;
        rec.set_face_normal(&r, normal);
        true
    }
}