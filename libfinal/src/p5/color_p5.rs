use raylib::prelude::*;

use crate::engine::Engine;
use crate::matem::{constrain, floor, lerp};
use crate::parametros::{ColorMode, Parametros};
use crate::utiles::aux_hsv_to_rgb;

#[derive(Debug, Clone, Copy, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const RED: Color = Color {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const GREEN: Color = Color {
        r: 0,
        g: 255,
        b: 0,
        a: 255,
    };
    pub const BLUE: Color = Color {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
    };
    pub const MAROON: Color = Color {
        r: 190,
        g: 33,
        b: 55,
        a: 255,
    };
    pub const LIME: Color = Color {
        r: 0,
        g: 158,
        b: 47,
        a: 255,
    };
    pub const DARKBLUE: Color = Color {
        r: 0,
        g: 82,
        b: 172,
        a: 255,
    };

    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub fn to_color_raylib(&self) -> raylib::color::Color {
        raylib::color::Color::new(self.r, self.g, self.b, self.a)
    }
}

// https://stackoverflow.com/questions/23090019/fastest-formula-to-get-hue-from-rgb
pub fn hue(color: Color) -> f32 {
    //println!("En color::hue()  color.r={:?} color.g={:?} color.b={:?}",

    let error = 0.01f32; // Para comparaciones de f32

    let r = color.r as f32 / 255.0;
    let g = color.g as f32 / 255.0;
    let b = color.b as f32 / 255.0;

    let min = min(min(r, g), b);
    let max = max(max(r, g), b);

    let mut hue = 0.0;

    if (max - r).abs() < error {
        // Uso control de error
        hue = (g - b) / (max - min);
    } else if (max - g).abs() < error {
        hue = 2.0 + (b - r) / (max - min);
    } else if (max - b).abs() < error {
        hue = 4.0 + (r - g) / (max - min);
    }

    hue *= 60.0;
    if hue > 0.0 {
        // h is a positive angle in the color wheel
        hue = floor(hue);
    } else {
        // h is a negative angle.
        hue = floor(360.0 - hue);
    }

    hue
}

// Extrae el valor de brillo HSB de un objeto Color.
// Luminance (standard for certain colour spaces): (0.2126*R + 0.7152*G + 0.0722*B)
pub fn brightness(color: Color) -> f32 {
    color.r as f32 * 0.2126 + color.g as f32 * 0.7152 + color.b as f32 * 0.0722
}

//Funciones auxiliares de hue()
pub fn max(a: f32, b: f32) -> f32 {
    if a >= b {
        a
    } else {
        b
    }
}

pub fn min(a: f32, b: f32) -> f32 {
    if a <= b {
        a
    } else {
        b
    }
}

pub fn stroke(stroke: Color, param: &mut Parametros) {
    param.stroke_bool = true;
    param.stroke_color = stroke;
}

pub fn stroke1(v: f32, param: &mut Parametros) {
    param.stroke_bool = true;
    param.stroke_color = match param.colormode {
        ColorMode::RGB => Color::new(v as u8, v as u8, v as u8, 255),
        //ColorMode::HSB => aux_hsv_to_rgb2(v, v, v, 255.0),
        _ => panic!("Error"),
    };
}

// Recibe color y transparencia
pub fn stroke2(c: f32, a: f32, param: &mut Parametros) {
    param.stroke_bool = true;
    param.stroke_color = match param.colormode {
        ColorMode::RGB => Color::new(c as u8, c as u8, c as u8, a as u8),
        //ColorMode::HSB => aux_hsv_to_rgb2(c, c, c, a),
        _ => panic!("Error"),
    };
}

pub fn stroke3(r: f32, g: f32, b: f32, param: &mut Parametros) {
    param.stroke_bool = true;
    param.stroke_color = match param.colormode {
        ColorMode::RGB => Color::new(r as u8, g as u8, b as u8, 255),
        ColorMode::HSB => aux_hsv_to_rgb(r, g, b),
        _ => panic!("Error"),
    };
}

//    pub fn stroke1(&mut self, v: i32) {
//        self.stroke = true;
//        self.stroke_color = StrokeColor {
//            r: v as f32 / 255.0,
//            g: v as f32 / 255.0,
//            b: v as f32 / 255.0,
//            a: 1f32,
//        };
//    }
pub fn no_stroke(param: &mut Parametros) {
    param.stroke_bool = false;
}

pub fn get_stroke_color(param: &mut Parametros) -> Color {
    Color::new(
        param.stroke_color.r,
        param.stroke_color.g,
        param.stroke_color.b,
        param.stroke_color.a,
    )
}

pub fn background(_engine: &mut Engine, d: &mut RaylibDrawHandle, c: u8) {
    //pub fn background(engine: &mut Engine, c: u8) {
    //d.clear_background(raylib::color::Color::WHITE);

    //let mut d = engine.rl.begin_drawing(&engine.th);

    //let mut h = engine.d.begin_drawing(&engine.th);

    //let mut d = engine.devuelve_handle();

    d.clear_background(raylib::color::Color::new(c, c, c, 255));
}
pub fn background3(_engine: &mut Engine, d: &mut RaylibDrawHandle, r: u8, g: u8, b: u8) {
    //pub fn background(engine: &mut Engine, c: u8) {
    //d.clear_background(raylib::color::Color::WHITE);

    //let mut d = engine.rl.begin_drawing(&engine.th);

    //let mut h = engine.d.begin_drawing(&engine.th);

    //let mut d = engine.devuelve_handle();

    d.clear_background(raylib::color::Color::new(r, g, b, 255));
}
// Para no tenerlo que poner en setup()
pub fn background_una_vez(engine: &mut Engine, d: &mut RaylibDrawHandle, c: u8) {
    if engine.param.background_1_vez == false {
        d.clear_background(raylib::color::Color::new(c, c, c, 255));
        engine.param.background_1_vez = true;
    }
}

//Presenta en pantalla una imagen de fondo
pub fn background_image(_engine: &mut Engine) {}

pub fn fill(color: Color, param: &mut Parametros) {
    param.fill_bool = true;
    param.fill_color = color;
}

pub fn fill1(c: f32, param: &mut Parametros) {
    param.fill_bool = true;
    param.fill_color = Color::new(c as u8, c as u8, c as u8, 255);
}

pub fn fill2(c: f32, a: f32, param: &mut Parametros) {
    param.fill_bool = true;
    param.fill_color = Color::new(c as u8, c as u8, c as u8, a as u8);
}

pub fn fill3(r: f32, g: f32, b: f32, param: &mut Parametros) {
    param.fill_bool = true;
    param.fill_color = match param.colormode {
        ColorMode::RGB => Color::new(r as u8, g as u8, b as u8, 255),

        ColorMode::HSB => aux_hsv_to_rgb(r, g, b),

        _ => panic!("Error"),
    };
}

pub fn fill4(r: f32, g: f32, b: f32, a: f32, param: &mut Parametros) {
    param.fill_bool = true;
    param.fill_color = Color::new(r as u8, g as u8, b as u8, a as u8);
}

pub fn no_fill(param: &mut Parametros) {
    param.fill_bool = false;
}

pub fn colormode(cm: ColorMode, param: &mut Parametros) {
    param.colormode = cm;
}

pub fn color1(c: u8) -> Color {
    Color::new(c, c, c, c)
}

// Recibe dos colores y devuelve un color interpolado
pub fn lerp_color(color1: Color, color2: Color, mut amt: f32) -> Color {
    if amt > 1.0 {
        amt = 1.0;
    }

    if amt < 0.0 {
        amt = 0.0;
    }

    let r0 = lerp(color1.r as f32, color2.r as f32, amt);
    let r = constrain(r0, 0.0, 255.0) as u8;
    let g0 = lerp(color1.g as f32, color2.g as f32, amt);
    let g = constrain(g0, 0.0, 255.0) as u8;
    let b0 = lerp(color1.b as f32, color2.b as f32, amt);
    let b = constrain(b0, 0.0, 255.0) as u8;
    Color::new(r, g, b, 255)
}
