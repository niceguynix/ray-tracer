pub type Color = super::vector::Vec3;

pub fn write_color(color: &Color) {
    print!(
        "{} {} {}\n",
        (255.999 * color.x) as u32,
        (255.999 * color.y) as u32,
        (255.999 * color.z) as u32,
    );
}
