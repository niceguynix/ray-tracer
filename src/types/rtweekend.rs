use super::vector::Vec3;

pub const INFINITY: f32 = std::f32::INFINITY;

pub const PI: f32 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    return degrees * PI / 180.0;
}

pub fn random_in_unit_sphere()->Vec3 {
    loop {
        let p = Vec3::random_range(-1.0,1.0);
        if p.length_squared() >= 1.0 {continue;}
        return p;
    }
}

pub fn random_unit_vector()->Vec3{
    random_in_unit_sphere().normal()
}

pub fn random_in_hemisphere(normal:&Vec3)->Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        return in_unit_sphere;
    }
    -in_unit_sphere
}