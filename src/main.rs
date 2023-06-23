mod types;
mod objects;
mod materials;

use std::rc::Rc;

use types::{color::Color, ray::Ray, vector::Point3, vector::Vec3,hittable::{HitRecord},hittable_list::HittableList, rtweekend::{random_in_unit_sphere, random_in_hemisphere}};
use rand::prelude::*;
use objects::sphere::Sphere;
use crate::{types::{hittable::Hittable, camera::Camera}, materials::{lambertian::Lambertian, metal::Metal, dielectric::Dielectric}};
use crate::types::rtweekend::INFINITY;

const ASPECT_RATIO:f32 = 16.0 / 9.0;
const IMAGE_WIDTH:u32 = 500;
const IMAGE_HEIGHT:u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL:u8 = 100;
const MAX_DEPTH:i8=50;

fn main() {

    // let mut world:HittableList<dyn Hittable>=HittableList::default();
    let mut world = HittableList::default();

    let material_ground = Rc::new(Lambertian {albedo: Color::new(0.8,0.8,0.0)});
    let material_center = Rc::new(Lambertian {albedo: Color::new(0.1,0.2,0.5)});
    let material_left   = Rc::new(Dielectric{ ir: 1.5 });
    let material_right  = Rc::new(Metal {albedo:Color::new(0.8, 0.6, 0.2),fuzz:1.0});

    let copy = Rc::clone(&material_ground);

    world.add(Rc::new(Sphere{center:Point3::new( 0.0, -100.5, -1.0),radius:100.0,mat_ptr:copy}));
    world.add(Rc::new(Sphere{center:Point3::new( 0.0, 0.0, -1.0),radius:0.5,mat_ptr:material_center}));
    world.add(Rc::new(Sphere{center:Point3::new(-1.0, 0.0, -1.0),radius:0.5,mat_ptr:material_left}));
    world.add(Rc::new(Sphere{center:Point3::new( 1.0, 0.0, -1.0),radius:0.5,mat_ptr:material_right}));

    // world.add(Rc::new(Sphere{center:Point3{x:1.0,y:0.0,z:-2.0},radius:0.5}));

    eprintln!("Start");
    eprintln!("{} {} {}",material_ground.albedo.x,material_ground.albedo.y,material_ground.albedo.z);
    eprintln!("Start");


    let camera=Camera::new();

    let mut rng = rand::thread_rng();

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    eprintln!("{} {} {}", IMAGE_HEIGHT, IMAGE_WIDTH, ASPECT_RATIO);

    for j in (0..IMAGE_HEIGHT).rev(){
        for i in 0..IMAGE_WIDTH  {
            // eprintln!("\rScanlines remaining: {} ", j);
            let mut pixel_color=Color::default();
            for _s in 0..SAMPLES_PER_PIXEL{
            
                let u = (i as f32 + rng.gen::<f32>()) / (IMAGE_WIDTH - 1) as f32;
                let v = (j as f32 + rng.gen::<f32>()) / (IMAGE_HEIGHT - 1) as f32;

                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r,&world,MAX_DEPTH);
            }
            types::color::write_color(&pixel_color,SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("Rendered");
}

fn ray_color(ray: &Ray,world:&HittableList,depth:i8) -> Color {
    let mut rec=HitRecord::default();
    
    if depth <= 0 {
        return Color::default();
    }

    if world.hit(&ray,0.001,INFINITY,&mut rec){
        // let target = rec.p + rec.normal + random_in_unit_sphere();
        let mut scattered=Ray::new(Vec3::default(),Vec3::default());
        let mut attenuation=Vec3::default();    
        if rec.mat_ptr.scatter(ray,&rec,&mut attenuation,&mut scattered){
            // eprintln!("{} {} {}",attenuation.x,attenuation.y,attenuation.z);
            return attenuation * ray_color(&scattered, world, depth-1);
        }

        return Color::default();
        
        // let random=random_in_hemisphere(&rec.normal);
        // let target = rec.p +random;
        // let new_ray = Ray::new(rec.p,target-rec.p);
        // return  0.5* ray_color(&new_ray,world,depth-1);
    }   
    
    // if world.hit(&ray,0.0,INFINITY,&mut rec){
    //     return 0.5 * (rec.normal + Color::new(1.0,1.0,1.0));
    // }

    let unit_direction = ray.dir.normal();
    let t = 0.5 * (unit_direction.y + 1.0);
    // let t = 0.5 * (ray.dir.y+1.0);
    // eprintln!("{}",t);
    // (1.0 - t) * Color::new(0.0, 0.0, 0.0) + t * Color::new(0.5, 0.7, 1.0)
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    // (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.0, 0.0, 0.0)
    // Color::new(1.0,1.0,1.0)*t

}