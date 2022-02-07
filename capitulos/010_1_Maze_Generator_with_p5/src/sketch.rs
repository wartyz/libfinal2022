use raylib::prelude::*;

use libfinal::engine::Engine;
//use libfinal::environment::frame_rate;
use libfinal::matem::{floor, random, random_usize};
use libfinal::p5::color_p5::{background, fill1, fill3, no_stroke, stroke1};
use libfinal::parametros::CodigosTecla;
//use libfinal::rendering::createcanvas;
use libfinal::shapes::{ellipse, stroke_weight};
use libfinal::transform::translate;
use libfinal::typography::{text, text_size};

use crate::cell::Cell;

//use libfinal::p5::sound::SoundFile;

// Ancho y alto de la pantalla
//pub const ANCHO: u32 = 400;
//pub const ALTO: u32 = 400;

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 800;
pub const ALTO: i32 = 800;

// Aqui vendrá el pseudocódigo javascript

pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    cols: usize,
    rows: usize,
    w: f32,
    grid: Vec<Cell>,
}

impl Sketch {
    pub fn new() -> Sketch {
        let nombre = "Ventana";
        let engine = Engine::new(ANCHO as f32, ALTO as f32);
        let w = 40.0;

        Sketch {
            engine,
            cols: floor(ANCHO as f32 / w) as usize,
            rows: floor(ALTO as f32 / w) as usize,
            w,
            grid: vec![],
        }
    }

    // Función setup() de javascript
    pub fn setup(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {
        //createcanvas(&mut self.engine, ANCHO, ALTO);
        for j in 0..self.rows {
            for i in 0..self.cols {
                let cell = Cell::new(i, j);
                self.grid.push(cell);
            }
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) -> bool {
        if !self.engine.update(rl, th) {
            return false;
        }
        true
    }

    // Función draw() de javascript
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        background(&mut self.engine, d, 51);
        for i in 0..self.grid.len() {
            self.grid[i].show(self.w, &mut self.engine, d);
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
