//https://www.youtube.com/watch?v=mhjuuHl6qHM
// 37:45

use raylib::prelude::*;

use libfinal::ambient::draw_fps_fi;
use libfinal::constantes::*;
use libfinal::engine::Engine;
use libfinal::matem::*;
use libfinal::p5::color_p5::*;
use libfinal::p5::image_p5::P5Image;
use libfinal::parametros::*;
use libfinal::shapes::*;
use libfinal::typography::{text, text_size};

// Aqui vendrá el pseudocódigo javascript
use crate::boid::Boid;

//use crate::env_item::EnvItem;
//use crate::player::Player;

// Constantes del sketch

//pub const N: i32 = 128;
//pub const SCALE: i32 = 4;

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 640;
pub const ALTO: i32 = 360;

pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    flock: Vec<Boid>,

    align_slider: f32,
    cohesion_slider: f32,
    separation_slider: f32,
}

impl Sketch {
    pub fn new(rl: &mut RaylibHandle, th: &RaylibThread) -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);
        let flock: Vec<Boid> = vec![];

        //let flock = Boid::new(ANCHO as f32, ALTO as f32);

        Sketch {
            engine,
            flock,

            align_slider: 0.0,
            cohesion_slider: 0.0,
            separation_slider: 0.0,
        }
    }

    // Función setup() de javascript
    pub fn setup(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {
        for _ in 0..1000 {
            let b = Boid::new(ANCHO as f32, ALTO as f32);

            self.flock.push(b);
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
        {
            background(&mut self.engine, d, 51);
            // Draw GUI controls
            //------------------------------------------------------------------------------
            self.align_slider = d.gui_slider_bar(
                Rectangle::new(0.0, 40.0, 120.0, 20.0),
                None,
                Some(rstr!("Align")),
                self.align_slider,
                -450.0,
                450.0,
            );

            self.separation_slider = d.gui_slider_bar(
                Rectangle::new(0.0, 70.0, 120.0, 20.0),
                None,
                Some(rstr!("Separation")),
                self.separation_slider,
                -450.0,
                450.0,
            );

            self.cohesion_slider = d.gui_slider_bar(
                Rectangle::new(0.0, 100.0, 120.0, 20.0),
                None,
                Some(rstr!("Cohesion")),
                self.cohesion_slider,
                -450.0,
                450.0,
            );
        }
        //        // Creamos el único RaylibDrawHandle
        //        let mut d = rl.begin_drawing(&th);

        //        for boid in self.flock.iter() {
        //            boid.flock(&self.flock);
        //            boid.update()
        //                .show(&mut d, &mut self.engine.param);
        //        }

        // -------------  OJO me ha costado mucho llegar a esto!!!!!!!!!
        for k in 0..self.flock.len() {
            self.flock[k] = self.flock[k]
                .edges()
                .flock(
                    &self.flock,
                    self.align_slider,
                    self.cohesion_slider,
                    self.separation_slider,
                )
                .update()
                .show(d, &mut self.engine.param);
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
