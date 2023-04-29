mod types;
mod objects;

use std::rc::Rc;

use types::{color::Color, ray::Ray, vector::Point3, vector::Vec3,hittable::{HitRecord},hittable_list::HittableList};

use objects::sphere::Sphere;
use crate::types::hittable::Hittable;
use crate::types::rtweekend::INFINITY;

//Image dimensions
const ASPECT_RATIO: f32 = 16.0 / 9.0;

const IMAGE_WIDTH: u32 = 500;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

//Camera dimensions
const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f32 = 1.0;

//Viewport properties
const ORIGIN: Point3 = Point3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};
const HORIZONTAL: Vec3 = Vec3 {
    x: VIEWPORT_WIDTH,
    y: 0.0,
    z: 0.0,
};
const VERTICAL: Vec3 = Vec3 {
    x: 0.0,
    y: VIEWPORT_HEIGHT,
    z: 0.0,
};

impl<T: Hittable + ?Sized> Hittable for &T {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32,rec:&mut HitRecord) -> bool {
        (*self).hit(r, t_min, t_max,rec)
    }
}

fn main() {

    // let mut world:HittableList<dyn Hittable>=HittableList::default();
    let mut world = HittableList::default();
    
    world.add(Rc::new(Sphere{center:Point3{x:0.0,y:0.0,z:-1.0},radius:0.5}));
    world.add(Rc::new(Sphere{center:Point3{x:1.0,y:-100.5,z:-1.0},radius:100.0}));
    world.add(Rc::new(Sphere{center:Point3{x:1.0,y:0.0,z:-2.0},radius:0.5}));

    let lower_left_corner = ORIGIN
        - HORIZONTAL / 2.0
        - VERTICAL / 2.0
        - Vec3 {
            x: 0.0,
            y: 0.0,
            z: FOCAL_LENGTH,
        };

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    eprintln!("{} {} {}", IMAGE_HEIGHT, IMAGE_WIDTH, ASPECT_RATIO);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            // eprintln!("\rScanlines remaining: {} ", j);

            let u = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let v = j as f32 / (IMAGE_HEIGHT - 1) as f32;

            let r = Ray::new(
                ORIGIN,
                lower_left_corner + u * HORIZONTAL + v * VERTICAL - ORIGIN,
            );
            let pixel_color = ray_color(&r,&world);
            types::color::write_color(&pixel_color);
        }
    }

    eprintln!("Rendered");
}

fn ray_color(ray: &Ray,world:&HittableList) -> Color {
    let mut rec=HitRecord::default();
    
    if world.hit(&ray,0.0,INFINITY,&mut rec){
        return 0.5*(rec.normal+Color::new(0.5,0.5,0.5));
    }
    

    let unit_direction = ray.dir.normal();
    let t = 0.5 * (unit_direction.y + 1.0);
    // let t = 0.5 * (ray.dir.y+1.0);
    // eprintln!("{}",t);
    (1.0 - t) * Color::new(0.0, 0.0, 0.0) + t * Color::new(0.5, 0.7, 1.0)
    // (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.0, 0.0, 0.0)
    // Color::new(1.0,1.0,1.0)*t

}