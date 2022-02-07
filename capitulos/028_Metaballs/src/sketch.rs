//https://www.youtube.com/watch?v=ccYLb7cLB1I    Video 028
// Pruebas duracion codigo
use std::time::{Duration, Instant};

use raylib::prelude::*;

use libfinal::ambient::draw_fps_fi;
use libfinal::constantes::*;
use libfinal::engine::Engine;
use libfinal::image::ImageLf;
use libfinal::matem::{constrain, dist4, random, round};
use libfinal::p5::color_p5::{colormode, Color};
use libfinal::p5::image_p5::P5Image;
use libfinal::parametros::Filtros::Gray;
use libfinal::parametros::*;
use libfinal::shapes::*;
use libfinal::typography::{text, text_size};

use crate::blob::Blob;

// Constantes del sketch

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 720;
pub const ALTO: i32 = 360;

// Aqui vendrá el pseudocódigo javascript

pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    //
    //b: Blob,
    blobs: Vec<Blob>,
    //textura: Option<raylib::prelude::Texture2D>,
}

impl Sketch {
    pub fn new() -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);

        let mut blobs: Vec<Blob> = vec![];
        for _ in 0..10 {
            blobs.push(Blob::new(random(ANCHO as f32), random(ALTO as f32)));
        }
        Sketch {
            engine,
            blobs,
            //textura: None,
            //b: Blob::new(100.0, 100.0),
        }
    }

    // Función setup() de javascript
    pub fn setup(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {
        colormode(ColorMode::HSB, &mut self.engine.param);
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) -> bool {
        ImageLf::load_pixels(&mut self.engine.param, rl, th);

        //let start = Instant::now();
        for x in 0..ANCHO as usize {
            //let start = Instant::now();
            for y in 0..ALTO as usize {
                let index = x + y * ANCHO as usize;

                let mut sum: f32 = 0.0;
                for i in 0..self.blobs.len() {
                    let d = dist4(x as f32, y as f32, self.blobs[i].pos.x, self.blobs[i].pos.y);
                    sum += 100.0 * self.blobs[0].r / d;
                }

                self.engine.param.pixels[index] =
                    //raylib::color::Color::color_from_hsv(Vector3::new(sum % 255.0, 255.0, 255.0));
                    raylib::color::Color::color_from_hsv(sum % 255.0, 255.0, 255.0);
                //Color::new(sum as u8, sum as u8, 255, 255);
            }

            //let duration = start.elapsed();
            //dbg!(duration);
        }
        //let duration = start.elapsed();
        //dbg!(duration);
        //
        //        self.textura =
        //            Some(ImageLf::preupdate_pixels(&mut self.engine.param, rl, th, ANCHO, ALTO));

        if !self.engine.update(rl, th) {
            return false;
        }
        true
    }

    // Función draw() de javascript
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        //background(&mut self.engine, d, 51);

        //        for x in 0..ANCHO {
        //            for y in 0..ALTO {
        //                let index = (x + y * ANCHO) as usize;
        //                let mut sum: f32 = 0.0;
        //                for i in 0..self.blobs.len() {
        //                    let d =
        //                        dist4(x as f32, y as f32, self.blobs[i].pos.x, self.blobs[i].pos.y);
        //                    sum += 100.0 * self.blobs[0].r / d;
        //                }
        //                self.engine.param.pixels[index] =
        //                    Color::new(sum as u8, sum as u8, sum as u8, 255);
        //            }
        //        }

        //ImageLf::update_pixels(&mut self.engine.param, d, self.textura.as_ref(), ANCHO, ALTO);
        //ImageLf::update_pixels(d, self.textura.as_ref());
        ImageLf::update_pixels(&mut self.engine.param, d, ANCHO, ALTO);

        for i in 0..self.blobs.len() {
            self.blobs[i].update();
            //self.blobs[i].show(d, &mut self.engine.param);
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
