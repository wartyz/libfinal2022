use raylib::prelude::*;

use libfinal::constantes::TWO_PI;
use libfinal::engine::Engine;
//use libfinal::environment::frame_rate;
use libfinal::matem::{random, random_usize};
use libfinal::p5::color_p5::{background, fill1, fill3, no_stroke, stroke1};

use libfinal::parametros::CodigosTecla;
//use libfinal::rendering::createcanvas;
use libfinal::shapes::{ellipse, stroke_weight};
use libfinal::transform::translate;
use libfinal::typography::{text, text_size};

use crate::planet::Planet;

//use libfinal::p5::sound::SoundFile;

// Aqui vendrá el pseudocódigo javascript

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 600;
pub const ALTO: i32 = 600;

pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    sun: Planet,
}

impl Sketch {
    pub fn new() -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);
        Sketch {
            engine,
            sun: Planet::new(50.0, 0.0, 0.0, random(TWO_PI)),
        }
    }

    // Función setup() de javascript
    //pub fn setup(&mut self, d: &mut RaylibDrawHandle) {
    pub fn setup(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {
        //createcanvas(&mut self.engine, ANCHO, ALTO);
        self.sun.spawn_moons(5, 1.0);
    }

    //pub fn update(&mut self, d: &mut RaylibDrawHandle) -> bool {
    pub fn update(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) -> bool {
        if !self.engine.update(rl, th) {
            return false;
        }
        true
    }

    // Función draw() de javascript
    //pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        background(&mut self.engine, d, 51);
        translate(
            self.engine.param.ancho / 2.0,
            self.engine.param.alto / 2.0,
            &mut self.engine.param,
        );
        self.sun.show(&mut self.engine, d);
        self.sun.orbit();
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
