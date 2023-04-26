mod types;

use types::{color::Color, ray::Ray, vector::Point3, vector::Vec3};

//Image dimensions
const ASPECT_RATIO: f32 = 16.0 / 9.0;

const IMAGE_WIDTH: u32 = 400;
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

fn main() {
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
            let pixel_color = ray_color(&r);
            types::color::write_color(&pixel_color);
        }
    }

    eprintln!("Rendered");
}

fn ray_color(ray: &Ray) -> Color {
    let int_sph = intersect_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, ray);
    if int_sph != -1.0 {
        let t = ray.at(int_sph) - Vec3::new(0.0, 0.0, -1.0);
        let int_pt = t.normal();
        return 0.5 * Color::new(int_pt.x + 1.0, int_pt.y + 1.0, int_pt.z + 1.0);
    }
    let unit_direction = ray.dir.normal();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(0.0, 0.0, 0.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn intersect_sphere(center: &Point3, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.orig - *center;
    let a = ray.dir.dot(&ray.dir);
    let b = 2.0 * oc.dot(&ray.dir);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - f32::sqrt(discriminant)) / (2.0 * a);
    }
}
