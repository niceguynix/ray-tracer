use super::vector::Vec3;

pub const INFINITY: f32 = std::f32::INFINITY;

pub const PI: f32 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    return degrees * PI / 180.0;
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().normal()
}

pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        return in_unit_sphere;
    }
    -in_unit_sphere
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = f32::min(n.dot(&-*uv), 1.0);
    let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_parallel = -f32::sqrt(f32::abs(1.0 - r_out_perp.length_squared())) * *n;

    r_out_perp + r_out_parallel
}
