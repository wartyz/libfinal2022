use raylib::prelude::*;

use libfinal::constantes::*;
use libfinal::engine::Engine;
use libfinal::matem::{random, random_i32, random_usize};
use libfinal::p5::color_p5::{background, fill1, fill3, no_stroke, stroke1, Color};
use libfinal::p5::image_p5::P5Image;
use libfinal::p5::vector_p5::Vector2Fi;
use libfinal::parametros::CodigosTecla;
use libfinal::shapes::{ellipse, rect, stroke_weight};
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
    //    current: [[Color_fi; ANCHO as usize]; ALTO as usize],
    //    previous: [[Color_fi; ANCHO as usize]; ALTO as usize],
    current: Vec<Vec<u8>>,
    previous: Vec<Vec<u8>>,
    dampening: f32,
}

impl Sketch {
    pub fn new() -> Sketch {
        let mut engine = Engine::new(ANCHO as f32, ALTO as f32);

        let dampening = 0.99;

        let mut current = vec![vec![0; ROWS]; COLS * 4];
        let mut previous = vec![vec![0; ROWS]; COLS * 4];

        //        for c in 0..COLS {
        //            for r in 0..ROWS {
        //                current.push(Color::new(0, 0, 0, 255));
        //                previous.push(Color::new(0, 0, 0, 255));
        //            }
        //        }

        //        let mut current = [[Color_fi::new(0, 0, 0, 255); COLS]; ROWS];
        //        let mut previous = [[Color_fi::new(0, 0, 0, 255); COLS]; ROWS];

        Sketch {
            engine,
            current,
            previous,
            dampening,
        }
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
        //let mut d = rl.begin_drawing(&th);

        //background(&mut self.engine, &mut d, 0);

        //P5Image::load_pixels(&mut self.engine.param, rl, &th);
        P5Image::load_pixels(&mut self.engine.param, rl, th);

        for i in 1..(COLS - 1) {
            for j in 1..(ROWS - 1) {
                self.current[i][j] = (self.previous[i - 1][j]
                    + self.previous[i + 1][j]
                    + self.previous[i][j - 1]
                    + self.previous[i][j + 1])
                    / 2
                    - self.current[i][j];

                self.current[i][j] = ((self.current[i][j]) as f32 * self.dampening) as u8;

                // A diferencia de Processing, el arreglo píxeles en p5.js tiene 4 entradas
                // para cada píxel, por lo que tenemos que multiplicar el índice por 4 y luego
                // establecer las entradas para cada componente de color por separado.
                let index = (i + j * COLS) * 4;
                let c = Color::new(
                    self.current[i][j],
                    self.current[i][j],
                    self.current[i][j],
                    255,
                );
                //                self.engine.param.pixels[index + 0] = self.current[i][j];
                //                self.engine.param.pixels[index + 1] = self.current[i][j];
                //                self.engine.param.pixels[index + 2] = self.current[i][j];
            }
        }

        P5Image::update_pixels();
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
