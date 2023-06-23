use super::{vector::{Point3, Vec3}, ray::Ray, rtweekend::degrees_to_radians};

pub struct Camera{
    origin:Point3,
    lower_left_corner:Point3,
    horizontal:Vec3,
    vertical:Vec3,
}

impl Camera{
        pub fn new(lookfrom:Point3,lookat:Point3,vup:Vec3,vfov:f32,aspect_ratio:f32)->Self{
            
            let theta = degrees_to_radians(vfov);
            let h = (theta/2.0).tan();
            
            let viewport_height = 2.0 * h;
            let viewport_width = aspect_ratio * viewport_height;
            let focal_length = 1.0;

            let w = (lookfrom-lookat).normal();
            let u = vup.cross(&w).normal();
            let v = w.cross(&u);

            let origin = lookfrom;
            let horizontal = viewport_width*u;
            let vertical = viewport_height*v;
            let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - w;

            Camera { origin, lower_left_corner, horizontal, vertical }
        }

        pub fn get_ray(&self,u:f32,v:f32)->Ray{
            Ray::new(self.origin,self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin)
        }
}