use super::{ray::Ray, vector::{Point3, Vec3}};

pub struct HitRecord {
    pub p:Point3,
    pub normal:Vec3,
    pub t:f32,
    front_face:bool
}


impl HitRecord{
    pub fn set_face_normal(&mut self,r:&Ray,outward_normal:Vec3) {
        self.front_face = r.dir.dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {outward_normal} else { -outward_normal};
    }
}

pub trait Hittable{
    fn hit(&self,r:Ray,t_min:f32,t_max:f32,rec:&mut HitRecord)->bool;
}