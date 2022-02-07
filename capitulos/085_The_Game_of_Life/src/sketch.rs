use raylib::prelude::*;

use libfinal::engine::Engine;
use libfinal::matem::{random, random_i32, random_usize};
use libfinal::p5::color_p5::{background, fill1, fill3, no_stroke, stroke1};
use libfinal::parametros::CodigosTecla;
use libfinal::shapes::{ellipse, rect, stroke_weight};
use libfinal::typography::{text, text_size};

//use crate::vehicle::Vehicle;

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 1280;
pub const ALTO: i32 = 720;

// Aqui vendrá el pseudocódigo javascript

pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    grid: Vec<Vec<usize>>,
    cols: usize,
    rows: usize,
    resolution: usize,
}

impl Sketch {
    pub fn new() -> Sketch {
        let mut engine = Engine::new(ANCHO as f32, ALTO as f32);

        let resolution = 10;
        let cols = (ANCHO / resolution) as usize;
        let rows = (ALTO / resolution) as usize;

        let mut grid = vec![vec![0; cols]; rows];

        Sketch {
            engine,
            grid,
            cols,
            rows,
            resolution: 10,
        }
    }

    // Función setup() de javascript
    pub fn setup(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {
        //createcanvas(&mut self.engine, ANCHO, ALTO);
        for j in 0..self.rows {
            for i in 0..self.cols {
                self.grid[j][i] = random_usize(2);
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

        for i in 0..self.cols {
            for j in 0..self.rows {
                let x = (i * self.resolution) as f32;
                let y = (j * self.resolution) as f32;
                if self.grid[j][i] == 1 {
                    fill1(255.0, &mut self.engine.param);
                    //fi.Fill1(255, s.Engine.Param)
                    stroke1(200.0, &mut self.engine.param);
                    //fi.Stroke1(0, s.Engine.Param)
                    rect(
                        d,
                        &mut self.engine.param,
                        x,
                        y,
                        (self.resolution - 1) as f32,
                        (self.resolution - 1) as f32,
                    );
                }
            }
        }

        let mut next = vec![vec![0; self.cols as usize]; self.rows as usize];

        // Compute next based on grid
        for i in 0..self.cols {
            for j in 0..self.rows {
                let state = self.grid[j][i];

                // Count live neighbors
                //sum := 0
                let g = &self.grid;
                let neighbors = &self.count_neighbors(g, i, j);

                if state == 0 && *neighbors == 3 {
                    next[j][i] = 1;
                } else if state == 1 && (*neighbors < 2 || *neighbors > 3) {
                    next[j][i] = 0;
                } else {
                    next[j][i] = state;
                }
            }
        }
        self.grid = next
    }
    pub fn count_neighbors(&self, grid: &Vec<Vec<usize>>, x: usize, y: usize) -> usize {
        let mut sum = 0;
        let kk = -1..2;
        let n = kk.len();
        for i in 0..n {
            for j in 0..n {
                let col = (x + i + self.cols) % self.cols;
                let row = (y + j + self.rows) % self.rows;

                sum += self.grid[row][col];
            }
        }
        sum -= self.grid[y][x];
        sum
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
