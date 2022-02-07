use raylib::prelude::*;

use libfinal::engine::Engine;
use libfinal::p5::color_p5::*;
use libfinal::parametros::Parametros;
use libfinal::shapes::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub r: f32,

    pub growing: bool,
}

impl Circle {
    pub fn new(x: f32, y: f32) -> Circle {
        Circle {
            x,
            y,
            r: 1.0,
            growing: true,
        }
    }

    pub fn grow(&mut self) {
        if self.growing {
            self.r += 0.5;
        }
    }

    pub fn edges(&self, param: &mut Parametros) -> bool {
        (self.x + self.r) > param.ancho
            || (self.x - self.r) < 0.0
            || (self.y + self.r) > param.alto
            || (self.y - self.r) < 0.0
    }

    //pub fn show(&mut self, param: &mut Parametros, engine: &mut Engine) {
    pub fn show(&self, engine: &mut Engine, d: &mut RaylibDrawHandle) {
        let p = &mut engine.param; // Nuevo invento ***********

        stroke1(255.0, p);

        stroke_weight(2.0, p);
        no_fill(p);
        ellipse(&mut engine.param, d, self.x, self.y, self.r, self.r);
        //ellipse(engine.canvas.as_mut().unwrap(), p, self.x, self.y, self.r, self.r);
    }
}