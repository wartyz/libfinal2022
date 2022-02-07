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
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
use std::thread::current;

use crate::cell::Cell;

//use libfinal::p5::sound::SoundFile;

// Ancho y alto de la pantalla
//pub const ANCHO: u32 = 400;
//pub const ALTO: u32 = 400;

// Aqui vendrá el pseudocódigo javascript
// Ancho y alto de la pantalla
pub const ANCHO: i32 = 1200;
pub const ALTO: i32 = 800;

pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    cols: i32,
    rows: i32,
    w: f32,
    grid: Vec<Cell>,

    current: Option<usize>,
}

impl Sketch {
    pub fn new() -> Sketch {
        let mut engine = Engine::new(ANCHO as f32, ALTO as f32);
        let w = 40.0;

        Sketch {
            engine,
            cols: floor(ANCHO as f32 / w) as i32,
            rows: floor(ALTO as f32 / w) as i32,
            w,
            grid: vec![],
            current: None,
        }
    }

    // Función setup() de javascript
    pub fn setup(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {
        //createcanvas(&mut self.engine, ANCHO, ALTO);
        for j in 0..self.rows {
            for i in 0..self.cols {
                let cell = Cell::new(i as i32, j as i32);
                self.grid.push(cell);
            }
        }
        self.current = Some(0);
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

        for g in &mut self.grid {
            g.show(self.w, &mut self.engine, d);
        }

        let mut next = None;
        if self.current.is_some() {
            let indice_next = self.current.unwrap();
            let mut n = self.grid[indice_next];
            next = n.check_neighbors(self.cols, self.rows, &mut self.grid);
        }

        //println!("en draw() 1  next = {:?}", next);
        if next.is_some() {
            self.grid[next.unwrap()].visited = true;

            self.current = next;
        } else {
            self.current = None;
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
