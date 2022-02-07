use libfinal::engine::Engine;
use libfinal::matem::{cos, sin};
use libfinal::p5::color_p5::{background, fill1, stroke1};
use libfinal::p5::vector_p5::Vector3;
use libfinal::shapes::{circle, line, stroke_weight};
use raylib::prelude::RaylibDrawHandle;
use raylib::{RaylibHandle, RaylibThread};
use std::f32::consts::PI;

// Constantes del sketch

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 600;
pub const ALTO: i32 = 800;

// Aqui vendrá el pseudocódigo javascript
pub const GRAVITY: f32 = 1.0;

pub struct Sketch {
    pub engine: Engine,
    // Variables globales del scketch
    angle: f32,
    angle_v: f32,
    angle_a: f32,
    bob: Vector3,
    len: f32,
    origin: Vector3,
}

impl Sketch {
    pub fn new(rl: &mut RaylibHandle, th: &RaylibThread) -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);

        Sketch {
            engine,

            angle: PI / 4.0,
            angle_v: 0.0,
            angle_a: 0.001,
            bob: Vector3::new(0.0, 0.0, 0.0),
            len: 600.0,
            origin: Vector3::new(300.0, 0.0, 0.0),
        }
    }

    // Función setup() de javascript
    pub fn setup(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {}

    pub fn update(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) -> bool {
        if !self.engine.update(rl, th) {
            return false;
        }
        true
    }

    // Función draw() de javascript
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        background(&mut self.engine, d, 0);

        let force = GRAVITY * sin(self.angle);
        self.angle_a = -1.0 * force / self.len;
        self.angle_v += self.angle_a;
        self.angle += self.angle_v;

        self.angle_v *= 0.99;

        self.bob.x = self.len * sin(self.angle) + self.origin.x;
        self.bob.y = self.len * cos(self.angle) + self.origin.y;

        stroke1(255.0, &mut self.engine.param);
        stroke_weight(8.0, &mut self.engine.param);
        fill1(127.0, &mut self.engine.param);
        line(
            d,
            &mut self.engine.param,
            self.origin.x,
            self.origin.y,
            self.bob.x,
            self.bob.y,
        );
        circle(&mut self.engine.param, d, self.bob.x, self.bob.y, 64.0);
    }

    pub fn key_pressed(&mut self) {
        //        if self.engine.param.key == CodigosTecla::A {
        //            self.left.mover(-10.0);
        //        } else if self.engine.param.key == CodigosTecla::Z {
        //            self.left.mover(10.0)
        //        } else if self.engine.param.key == CodigosTecla::J {
        //            self.right.mover(-10.0)
        //        } else if self.engine.param.key == CodigosTecla::M {
        //            self.right.mover(10.0)
        //        }
    }

    pub fn key_released(&mut self) {
        //        if self.engine.param.keyr == CodigosTecla::A {
        //            self.left.mover(0.0);
        //        } else if self.engine.param.keyr == CodigosTecla::Z {
        //            self.left.mover(0.0)
        //        } else if self.engine.param.keyr == CodigosTecla::J {
        //            self.right.mover(0.0)
        //        } else if self.engine.param.keyr == CodigosTecla::M {
        //            self.right.mover(0.0)
        //        }
    }
}
