use crate::vector::Vec3;

pub struct Color;
impl Color {
    #[allow(non_snake_case)]
    pub fn RGB(r: u8, g: u8, b: u8) -> Vec3<f32> {
        Vec3::new(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0
        )
    }
}
