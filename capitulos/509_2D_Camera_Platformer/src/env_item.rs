//use raylib::core::color::Color;
use libfinal::p5::color_p5::Color;
use raylib::core::math::Rectangle;

pub struct EnvItem {
    pub rect: Rectangle,
    pub blocking: bool,
    pub color: Color,
}

impl EnvItem {
    pub fn new(x: f32, y: f32, ancho: f32, alto: f32, blocking: bool, color: Color) -> EnvItem {
        let rect = Rectangle::new(x, y, ancho, alto);
        EnvItem {
            rect,
            blocking,
            color,
        }
    }
}
