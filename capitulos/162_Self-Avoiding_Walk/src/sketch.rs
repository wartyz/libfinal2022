// no funciona bien
use raylib::prelude::*;

use crate::spot::Spot;
use libfinal::engine::Engine;
use libfinal::matem::{floor, random, random_usize};
use libfinal::p5::color_p5::{background, background_una_vez, no_fill, stroke1, stroke2};
use libfinal::parametros::ModosBeginShape;
use libfinal::shapes::{begin_shape, end_shape, point, stroke_weight, vertex};
use libfinal::transform::translate;

// Constantes del sketch

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 400;
pub const ALTO: i32 = 400;

// Aqui vendrá el pseudocódigo javascript

pub struct Sketch {
    pub engine: Engine,
    // Variables globales del scketch
    //all_options: Vec<(f32, f32, bool)>,
    grid: Vec<Vec<Spot>>,
    spacing: f32,
    cols: f32,
    rows: f32,
    path: Vec<Spot>,
    spot: Spot,
}

impl Sketch {
    pub fn new(rl: &mut RaylibHandle, th: &RaylibThread) -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);

        let spacing = 5.0;
        let cols = floor(ANCHO as f32 / spacing);
        let rows = floor(ANCHO as f32 / spacing);

        let mut grid = Self::make_2d_array(cols as usize, rows as usize, spacing);

        let path = vec![];
        let spot = grid[0][0].clone();

        Sketch {
            engine,

            grid,
            spacing,
            cols,
            rows,
            path,
            spot,
        }
    }

    // Función setup() de javascript
    pub fn setup(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {
        for i in 0..self.cols as usize {
            for j in 0..self.rows as usize {
                self.grid[i][j] = Spot::new(i, j, self.spacing);
            }
        }
        self.spot = self.grid[0][0].clone();
        self.path.push(self.spot.clone());
        self.spot.visited = true;

        //self.spot.unwrap().visited = true;
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) -> bool {
        if !self.engine.update(rl, th) {
            return false;
        }
        true
    }

    // Función draw() de javascript
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        background_una_vez(&mut self.engine, d, 51);
        translate(
            self.spacing * 0.5,
            self.spacing * 0.5,
            &mut self.engine.param,
        );

        if let Some(mut sp) =
            self.spot
                .next_spot(self.rows as usize, self.cols as usize, &self.grid)
        {
            self.path.push(sp.clone());
            sp.visited = true;
        } else {
            let mut stuck = self.path.pop().unwrap();
            stuck.clear();
            self.spot = self.path[self.path.len() - 1].clone();
        }

        if self.path.len() == self.cols as usize * self.rows as usize {
            println!("solved!");

            // break;
        }

        stroke1(255.0, &mut self.engine.param);
        stroke_weight(self.spacing * 0.25, &mut self.engine.param);
        no_fill(&mut self.engine.param);
        begin_shape(ModosBeginShape::Close);
        for spot in &self.path {
            vertex(spot.x, spot.y, &mut self.engine.param);
        }
        end_shape(d, &mut self.engine.param, ModosBeginShape::Close);

        stroke1(255.0, &mut self.engine.param);
        stroke_weight(self.spacing * 0.5, &mut self.engine.param);
        point(d, &mut self.engine.param, self.spot.x, self.spot.y);
    }

    fn make_2d_array(cols: usize, rows: usize, spacing: f32) -> Vec<Vec<Spot>> {
        let mut arr = vec![vec![Spot::new(0, 0, 0.0); cols]; rows];

        arr
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
