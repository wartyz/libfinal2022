use raylib::prelude::*;

use libfinal::ambient::draw_fps_fi;
use libfinal::constantes::*;
use libfinal::engine::Engine;
use libfinal::matem::*;
use libfinal::p5::color_p5::*;
use libfinal::p5::image_p5::P5Image;
use libfinal::p5::vector_p5::Vector3;
use libfinal::parametros::*;
use libfinal::shapes::*;
use libfinal::typography::{text, text_size};

//use crate::env_item::EnvItem;
//use crate::player::Player;
use crate::fluid::Fluid;

// Constantes del sketch

//pub const N: i32 = 128;
//pub const SCALE: i32 = 4;

// Ancho y alto de la pantalla
pub const ANCHO: i32 = crate::fluid::N * crate::fluid::SCALE;
pub const ALTO: i32 = crate::fluid::N * crate::fluid::SCALE;

// Aqui vendrá el pseudocódigo javascript
pub const MAX_BUILDINGS: usize = 100;

pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    iter: i32,
    fluid: Fluid,
    t: f32,
}

impl Sketch {
    pub fn new(rl: &mut RaylibHandle, th: &RaylibThread) -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);

        let fluid = Fluid::new(0.2, 0.0, 0.0000001);

        Sketch {
            engine,
            iter: 16,
            fluid,
            t: 0.0,
        }
    }

    // Función setup() de javascript
    pub fn setup(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {}

    pub fn update(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) -> bool {
        if !self.engine.update(rl, th) {
            return false;
        }
        true
    }

    // Función draw() de javascript
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        {
            //            let mut d = rl.begin_drawing(&th);
            //            background(&mut self.engine, &mut d, 245); //RAYWHITE
        }

        let cx = (0.5 * ANCHO as f32 / crate::fluid::SCALE as f32) as i32;
        let cy = (0.5 * ALTO as f32 / crate::fluid::SCALE as f32) as i32;
        for i in -1..1 {
            for j in -1..1 {
                self.fluid
                    .add_density(cx + i, cy + j, random_range(50.0, 150.0));
            }
        }
        for i in 0..2 {
            let angle = noise1(self.t) * TWO_PI * 2.0;
            let mut v = Vector3::from_angle_s(angle);
            v.mult(0.2);
            self.t += 0.01;
            self.fluid.add_velocity(cx as i32, cy as i32, v.x, v.y);
        }

        background(&mut self.engine, d, 245); //RAYWHITE
        self.fluid.step(self.iter);
        self.fluid.renderd(d, &mut self.engine.param);
        //fluid.renderV();
        //fluid.fadeD();
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
