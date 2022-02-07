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

use crate::particle::Particle;

// Constantes del sketch

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 800;
pub const ALTO: i32 = 800;

// Aqui vendrá el pseudocódigo javascript
pub const MAX_BUILDINGS: usize = 100;

pub struct Sketch {
    pub engine: Engine,
    attractors: Vec<Vector3>,
    particles: Vec<Particle>,
    // Variables globales del scketch
}

impl Sketch {
    pub fn new(rl: &mut RaylibHandle, th: &RaylibThread) -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);

        let particles = vec![];

        let attractors = vec![];

        Sketch {
            engine,
            attractors,
            particles,
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
        background(&mut self.engine, d, 51);
        stroke1(255.0, &mut self.engine.param);
        stroke_weight(4.0, &mut self.engine.param);

        self.particles
            .push(Particle::new(random(ANCHO as f32), random(ALTO as f32)));
        //particles.push(Particle::new(200.0, 200.0));

        //point(d, &mut self.engine.param, self.attractor.x, self.attractor.y);

        // Dibuja attractors
        for i in 0..self.attractors.len() {
            stroke3(0.0, 255.0, 0.0, &mut self.engine.param);
            point(
                d,
                &mut self.engine.param,
                self.attractors[i].x,
                self.attractors[i].y,
            );
        }

        for i in 0..self.particles.len() {
            for j in 0..self.attractors.len() {
                self.particles[i].attracted(self.attractors[j]);
            }

            self.particles[i].update();
            self.particles[i].show(d, &mut self.engine.param);
        }
    }

    pub fn mouse_pressed(&mut self) {
        self.attractors.push(Vector3::new(
            self.engine.param.mouse_posicion.x,
            self.engine.param.mouse_posicion.y,
            0.0,
        ));
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
