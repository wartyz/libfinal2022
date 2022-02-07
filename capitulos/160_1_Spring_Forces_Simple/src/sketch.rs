use libfinal::engine::Engine;
use libfinal::p5::color_p5::{background3, fill, fill3, no_fill, no_stroke, stroke1, stroke2};
use libfinal::shapes::circle;
use raylib::prelude::*;

// Constantes del sketch

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 600;
pub const ALTO: i32 = 400;

// Aqui vendrá el pseudocódigo javascript

pub struct Sketch {
    pub engine: Engine,
    // Variables globales del scketch
    y: f32,
    velocity: f32,
    rest_length: f32,
    k: f32,
}

impl Sketch {
    pub fn new(rl: &mut RaylibHandle, th: &RaylibThread) -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);

        Sketch {
            engine,
            y: 250.0,
            velocity: 0.0,
            rest_length: 200.0,
            k: 0.1,
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
        background3(&mut self.engine, d, 112, 50, 126);

        no_stroke(&mut self.engine.param);
        fill3(45.0, 197.0, 244.0, &mut self.engine.param);
        circle(&mut self.engine.param, d, 300.0, self.y, 64.0);

        let x = self.y - self.rest_length;
        let force = -self.k * x;

        //F = A
        self.velocity += force;
        self.y += self.velocity;

        self.velocity *= 0.99;
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
