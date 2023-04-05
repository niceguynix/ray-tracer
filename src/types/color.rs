pub type Color = super::vector::Vec3;

pub fn write_color(color: &Color) {
    print!(
        "{} {} {}\n",
        255.999 * color.x,
        255.999 * color.y,
        255.999 * color.z
    );
}
