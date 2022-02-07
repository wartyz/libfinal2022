use libfinal::engine::Engine;
use libfinal::p5::color_p5::{background3, fill, fill3, no_fill, no_stroke, stroke1, stroke2};
use libfinal::p5::vector_p5::Vector3;

use crate::particle::Particle;
use crate::spring::Spring;
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
    bob: Particle,
    anchor: Particle,
    spring: Spring,

    // velocity: Vector3,
    // rest_length: f32,
    // k: f32,
    gravity: Vector3,
}

impl Sketch {
    pub fn new(rl: &mut RaylibHandle, th: &RaylibThread) -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);
        let bob = Particle::new(350.0, 0.0);
        let anchor = Particle::new(200.0, 210.0);
        Sketch {
            engine,
            bob,
            anchor,
            spring: Spring::new(0.01, 200.0, bob, anchor),
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

        self.spring.show(d, param);
        self.spring.update();
        self.anchor.show(d, param);
        self.anchor.update();

        if mouse_is_pressed(param) {
            self.bob
                .position
                .set(param.mouse_posicion.x, param.mouse_posicion.y, 1.0);
            self.bob.velocity.set(0.0, 0.0, 0.0);
        }
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
