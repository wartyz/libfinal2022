use raylib::prelude::*;

use crate::parametros::Parametros;

pub fn text(param: &mut Parametros, d: &mut RaylibDrawHandle, txt: &str, x: f32, y: f32) {
    d.draw_text(
        txt,
        x as i32,
        y as i32,
        param.tamafont,
        param.fill_color.to_color_raylib(),
    );
}

pub fn text_size(param: &mut Parametros, t: i32) {
    param.tamafont = t;
}
