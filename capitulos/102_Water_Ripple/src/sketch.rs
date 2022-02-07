// 10:00 error suma con overflow

use raylib::prelude::*;

use libfinal::constantes::*;
use libfinal::engine::Engine;
use libfinal::image::ImageLf;
use libfinal::image::*;
use libfinal::matem::{random, random_i32, random_usize};
use libfinal::p5::color_p5::{background, fill1, fill3, no_stroke, stroke1, Color};
use libfinal::p5::image_p5::P5Image;
use libfinal::parametros::{CodigosRaton, CodigosTecla};
use libfinal::shapes::{ellipse, rect, stroke_weight};
use libfinal::typography::{text, text_size};

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 800;
pub const ALTO: i32 = 800;

// Constantes del sketch
pub const COLS: usize = ANCHO as usize;
pub const ROWS: usize = ALTO as usize;

// Aqui vendrá el pseudocódigo javascript

pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    //    current: [[f32; COLS]; ROWS],
    //    previous: [[f32; COLS]; ROWS],

    //    current: [[Color_fi; ANCHO as usize]; ALTO as usize],
    //    previous: [[Color_fi; ANCHO as usize]; ALTO as usize],
    current: Vec<Vec<f32>>,
    previous: Vec<Vec<f32>>,
    dampening: f32,
}

impl Sketch {
    pub fn new() -> Sketch {
        let mut engine = Engine::new(ANCHO as f32, ALTO as f32);

        let dampening = 0.9;

        let mut current = vec![vec![0.0; ROWS]; COLS];
        let mut previous = vec![vec![0.0; ROWS]; COLS];

        //        let current = [[0.0; COLS]; ROWS];
        //        let previous = [[0.0; COLS]; ROWS];

        //        let color_negro = Color::new(0, 0, 0, 255);
        //        let mut current = vec![vec![color_negro; ROWS]; COLS];
        //        let mut previous = vec![vec![color_negro; ROWS]; COLS];

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
        //        for i in 0..COLS {
        //            for j in 0..ROWS {
        //                self.current[i][j] = 100.0;
        //                self.previous[i][j] = 100.0;
        //            }
        //        }
        self.previous[COLS / 2][ROWS / 2] = 1500.0;
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) -> bool {
        ImageLf::load_pixels(&mut self.engine.param, rl, th);
        if !self.engine.update(rl, th) {
            return false;
        }
        //        if self.engine.param.mouse_boton == CodigosRaton::Izquierdo {
        //            println!("En update");
        //            self.previous
        //                [self.engine.param.mouse_posicion.x as usize]
        //                [self.engine.param.mouse_posicion.y as usize] =
        //                Color::new(255, 255, 255, 255);
        //        }
        true
    }

    // Función draw() de javascript
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        //let mut d = rl.begin_drawing(&th);
        background(&mut self.engine, d, 0);

        //        ImageLf::load_pixels(&mut self.engine.param, rl, th);

        for i in 1..COLS - 1 {
            for j in 1..ROWS - 1 {
                self.current[i][j] = (self.previous[i - 1][j]
                    + self.previous[i + 1][j]
                    + self.previous[i][j - 1]
                    + self.previous[i][j + 1])
                    / 2.0
                    - self.current[i][j];

                self.current[i][j] = self.current[i][j] * self.dampening;

                //                    ((self.previous[i - 1][j]
                //                        .wrapping_add(self.previous[i + 1][j])
                //                        .wrapping_add(self.previous[i][j - 1])
                //                        .wrapping_add(self.previous[i][j + 1])) / 2)
                //                        .wrapping_sub(self.current[i][j]);

                let index = i + j * COLS;

                let c = self.current[i][j] as u8;

                //                if c != 0 {
                //                    println!("c = {}", c);
                //                }
                self.engine.param.pixels[index] = raylib::color::Color::new(c, c, c, 255);
            }
        }

        //ImageLf::update_pixels(&mut self.engine.param, rl, th, ANCHO, ALTO);

        //let mut temp: [[f32; COLS]; ROWS] = [[0.0; COLS]; ROWS];
        let mut temp: f32 = 0.0;
        for i in 0..COLS {
            for j in 0..ROWS {
                //temp[i][j] = self.previous[i][j];
                //                  self.previous[i][j] = self.current[i][j];
                //                self.current[i][j] = temp[i][j];

                let temp = self.previous[i][j];
                self.previous[i][j] = self.current[i][j];
                self.current[i][j] = temp;
            }
        }

        //let mut d = rl.begin_drawing(&th);

        //background(&mut self.engine, &mut d, 0);

        //P5Image::load_pixels(&mut self.engine.param, rl, &th);
        //        let mut v_pix: Vec<Color> = vec![];
        //
        //
        //        // Creo que no hace falta, estudiarlo
        //        //P5Image::load_pixels(&mut self.engine.param, rl, th);
        //
        //        let anch = COLS - 1;
        //        let alto = ROWS - 1;
        //
        //        for i in 1..anch {
        //            for j in 1..alto {
        //                self.current[i][j].r = (self.previous[i - 1][j].r +
        //                    self.previous[i + 1][j].r +
        //                    self.previous[i][j - 1].r +
        //                    self.previous[i][j + 1].r) / 2 -
        //                    self.current[i][j].r;
        //
        //                self.current[i][j].g = (self.previous[i - 1][j].g +
        //                    self.previous[i + 1][j].g +
        //                    self.previous[i][j - 1].g +
        //                    self.previous[i][j + 1].g) / 2 -
        //                    self.current[i][j].g;
        //
        //                self.current[i][j].b = (self.previous[i - 1][j].b +
        //                    self.previous[i + 1][j].b +
        //                    self.previous[i][j - 1].b +
        //                    self.previous[i][j + 1].b) / 2 -
        //                    self.current[i][j].b;
        //
        ////                self.current[i][j].a = (self.previous[i - 1][j].a +
        ////                    self.previous[i + 1][j].a +
        ////                    self.previous[i][j - 1].a +
        ////                    self.previous[i][j + 1].a) / 2 -
        ////                    self.current[i][j].a;
        //
        //
        //                //let index = i + j * anch;
        //
        //                let r = ((self.current[i][j].r) as f32 * self.dampening) as u8;
        //                let g = ((self.current[i][j].g) as f32 * self.dampening) as u8;
        //                let b = ((self.current[i][j].b) as f32 * self.dampening) as u8;
        //                //let a = ((self.current[i][j].a) as f32 * self.dampening) as u8;
        //                let a = 255;
        //
        //
        //                v_pix.push(Color::new(r, g, b, a));
        //            }
        //        }
        //        let kk = v_pix.len();
        //        P5Image::update_pixels(&mut self.engine.param, rl, th, v_pix, anch as i32 - 1, alto as i32 - 1);
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
