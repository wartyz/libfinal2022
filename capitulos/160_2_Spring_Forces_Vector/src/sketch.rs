use libfinal::engine::Engine;
use libfinal::p5::color_p5::{background3, fill, fill3, no_fill, no_stroke, stroke1, stroke2};
use libfinal::p5::vector_p5::Vector3;

use libfinal::events::mouse_is_pressed;
use libfinal::shapes::{circle, line, stroke_weight};
use raylib::prelude::*;

// Constantes del sketch

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 600;
pub const ALTO: i32 = 400;

// Aqui vendrá el pseudocódigo javascript

pub struct Sketch {
    pub engine: Engine,
    // Variables globales del scketch
    bob: Vector3,
    anchor: Vector3,

    velocity: Vector3,
    rest_length: f32,
    k: f32,
    gravity: Vector3,
}

impl Sketch {
    pub fn new(rl: &mut RaylibHandle, th: &RaylibThread) -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);

        Sketch {
            engine,
            bob: Vector3::new(350.0, 0.0, 0.0),
            anchor: Vector3::new(300.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, 0.0, 0.0),
            rest_length: 200.0,
            k: 0.1,
            gravity: Vector3::new(0.0, 0.1, 0.0),
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

    // 15:49 --------------------------------------------------------
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        background3(&mut self.engine, d, 112, 50, 126);
        let param = &mut self.engine.param;

        stroke_weight(4.0, param);
        stroke1(255.0, param);
        line(
            d,
            param,
            self.anchor.x,
            self.anchor.y,
            self.bob.x,
            self.bob.y,
        );
        fill3(45.0, 197.0, 244.0, param);
        circle(param, d, self.anchor.x, self.anchor.y, 32.0);
        circle(param, d, self.bob.x, self.bob.y, 64.0);

        if mouse_is_pressed(param) {
            self.bob.x = param.mouse_posicion.x;
            self.bob.y = param.mouse_posicion.y;
            self.velocity.set(0.0, 0.0, 0.0);
        }

        let mut force = Vector3::sub_s(&self.bob, &self.anchor);

        let x = force.mag() - self.rest_length;
        force.normalize();
        force.mult(-1.0 * self.k * x);

        // //F = A
        self.velocity.add(&force);
        self.velocity.add(&self.gravity);
        self.bob.add(&self.velocity);
        self.velocity.mult(0.99);
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
