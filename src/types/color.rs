pub type Color = super::vector::Vec3;

fn clamp(x:f32,min:f32,max:f32)->f32{
    if x < min {return min};
    if x > max {return max};
    return x;
}


pub fn write_color(color: &Color,samples_per_pixel:u8) {

    let r = color.x;
    let g = color.y;
    let b = color.z;

    let scale = 1.0 / samples_per_pixel as f32;
    r *= scale;
    g *= scale;
    b *= scale;

    print!(
        "{} {} {}\n",
        (256.0 * clamp(r, 0.0, 0.999)) as f32,
        (256.0 * clamp(g, 0.0, 0.999)) as f32,
        (256.0 * clamp(b, 0.0, 0.999)) as f32
    );
}
