use raylib::prelude::*;

use libfinal::engine::Engine;
use libfinal::matem::{floor, random, random_usize};
//use libfinal::rendering::createcanvas;
use libfinal::p5::color_p5::{background, fill1, fill3, no_stroke, stroke1};

use crate::cell::Cell;

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 1200;
pub const ALTO: i32 = 800;

// Aqui vendrá el pseudocódigo javascript


pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    cols: i32,
    rows: i32,
    w: f32,
    grid: Vec<Cell>,

    current: Option<usize>,

    stack: Vec<Option<usize>>,
}

impl Sketch {
    pub fn new() -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);
        let w = 10.0;


        Sketch {
            engine,
            cols: floor(ANCHO as f32 / w) as i32,
            rows: floor(ALTO as f32 / w) as i32,
            w,
            grid: vec![],
            current: None,
            stack: vec![],
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

//        let mut next = None;
//        if self.current.is_some() {
//            let indice_next = self.current.unwrap();
//            let mut n = self.grid[indice_next];
//            next = n.check_neighbors(self.cols, self.rows, &mut self.grid);
//        }


        let mut next = None;
        if self.current.is_some() {
            self.grid[self.current.unwrap()].visited = true;
            self.grid[self.current.unwrap()].highlight(self.w, &mut self.engine, d);

            //STEP 1
            //let next = self.grid[self.current.unwrap()].check_neighbors(self.cols, self.rows, &mut             self.grid);

            let indice_next = self.current.unwrap();
            let mut n = self.grid[indice_next];
            next = n.check_neighbors(self.cols, self.rows, &mut self.grid);
        }

        //println!("en draw() 1  next = {:?}", next);
        if next.is_some() {
            self.grid[next.unwrap()].visited = true;

            // STEP 2 Hace PUSH en el stack
            self.stack.push(self.current);
            // STEP 3
            self.remove_walls(self.current, next);

            self.current = next;
        } else {
            if self.stack.len() > 0 {
                // hace POP del stack
                self.current = self.stack.pop().unwrap();
            }
            //self.current = None;
        }
    }

    pub fn remove_walls(&mut self, a: Option<usize>, b: Option<usize>) {
        let x = self.grid[a.unwrap()].i - self.grid[b.unwrap()].i;
        if x == 1 {
            self.grid[a.unwrap()].walls[3] = false;
            self.grid[b.unwrap()].walls[1] = false;
        } else if x == -1 {
            self.grid[a.unwrap()].walls[1] = false;
            self.grid[b.unwrap()].walls[3] = false;
        }

        let y = self.grid[a.unwrap()].j - self.grid[b.unwrap()].j;

        if y == 1 {
            self.grid[a.unwrap()].walls[0] = false;
            self.grid[b.unwrap()].walls[2] = false;
        } else if y == -1 {
            self.grid[a.unwrap()].walls[2] = false;
            self.grid[b.unwrap()].walls[0] = false;
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