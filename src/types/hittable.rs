use super::{ray::Ray, vector::{Point3, Vec3}};

struct HitRecord {
    p:Point3,
    normal:Vec3,
    t:f32,
}

trait Hittable{
    fn hit(r:Ray,t_min:f32,t_max:f32,rec:HitRecord)->bool;
}