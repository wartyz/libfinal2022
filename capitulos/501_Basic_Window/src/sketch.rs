use raylib::prelude::*;

use libfinal::constantes::*;
use libfinal::engine::Engine;
use libfinal::matem::{random, random_i32, random_usize};
use libfinal::p5::color_p5::{background, fill1, fill3, no_stroke, stroke1, Color};
use libfinal::p5::image_p5::P5Image;
use libfinal::parametros::CodigosTecla;
use libfinal::shapes::{circle, ellipse, rect, stroke_weight};
use libfinal::typography::{text, text_size};

//use crate::vehicle::Vehicle;

//// Ancho y alto de la pantalla
//pub const ANCHO: i32 = 1280;
//pub const ALTO: i32 = 720;

// Constantes del sketch

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 1200;
pub const ALTO: i32 = 800;

pub const COLS: usize = ANCHO as usize;
pub const ROWS: usize = ALTO as usize;

// Aqui vendrá el pseudocódigo javascript

pub struct Sketch {
    pub engine: Engine,
    // Variables globales del scketch
}

impl Sketch {
    pub fn new() -> Sketch {
        let mut engine = Engine::new(ANCHO as f32, ALTO as f32);

        //        for c in 0..COLS {
        //            for r in 0..ROWS {
        //                current.push(Color::new(0, 0, 0, 255));
        //                previous.push(Color::new(0, 0, 0, 255));
        //            }
        //        }

        //        let mut current = [[Color_fi::new(0, 0, 0, 255); COLS]; ROWS];
        //        let mut previous = [[Color_fi::new(0, 0, 0, 255); COLS]; ROWS];

        Sketch { engine }
    }

    // Función setup() de javascript
    pub fn setup(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {
        //        //createcanvas(&mut self.engine, ANCHO, ALTO);
        //        for j in 0..self.rows {
        //            for i in 0..self.cols {
        //                self.grid[j][i] = random_usize(2);
        //            }
        //        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) -> bool {
        if !self.engine.update(rl, th) {
            return false;
        }
        true
    }

    // Función draw() de javascript
    pub fn draw(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {
        let mut d = rl.begin_drawing(&th);

        background(&mut self.engine, &mut d, 150);

        fill3(0.0, 102.0, 153.0, &mut self.engine.param);
        text(
            &mut self.engine.param,
            &mut d,
            "Congrats! You created your first window!",
            190.0,
            200.0,
        );
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
