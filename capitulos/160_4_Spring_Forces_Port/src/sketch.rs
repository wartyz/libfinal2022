use libfinal::engine::Engine;
use libfinal::p5::color_p5::{
    background3, fill, fill3, no_fill, no_stroke, stroke1, stroke2, stroke3,
};
use libfinal::p5::vector_p5::Vector3;

use crate::particle::Particle;
use crate::spring::Spring;
use libfinal::events::mouse_is_pressed;
use libfinal::parametros::ModosBeginShape;
use libfinal::shapes::{begin_shape, circle, end_shape, line, stroke_weight, vertex};
use raylib::prelude::*;

// Constantes del sketch

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 800;
pub const ALTO: i32 = 800;

// Aqui vendrá el pseudocódigo javascript

pub struct Sketch {
    pub engine: Engine,
    // Variables globales del scketch
    particles: Vec<Particle>,
    springs: Vec<Spring>,
    spacing: f32,
    k: f32,
    gravity: Vector3,
}

impl Sketch {
    pub fn new(rl: &mut RaylibHandle, th: &RaylibThread) -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);
        let mut particles = vec![];
        let mut springs = vec![];
        let spacing = 50.0;
        let k = 0.1;
        for i in 0..10 {
            particles.push(Particle::new(ANCHO as f32 / 2.0, i as f32 * spacing));
            if i != 0 {
                let a = particles.get(i).unwrap();
                let b = particles.get(i - 1).unwrap();
                let spring = Spring::new(k, spacing, a.clone(), b.clone());
                springs.push(spring);
            }
        }

        Sketch {
            engine,
            particles,
            springs,
            spacing,
            k,
            gravity: Vector3::new(0.0, 0.1, 0.0),
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

    // 15:49 --------------------------------------------------------
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        background3(&mut self.engine, d, 112, 50, 126);
        let param = &mut self.engine.param;

        // stroke_weight(4.0, param);
        // no_stroke(param);
        // stroke1(255.0, param);
        // line(
        //     d,
        //     param,
        //     self.anchor.x,
        //     self.anchor.y,
        //     self.bob.x,
        //     self.bob.y,
        // );
        // fill3(45.0, 197.0, 244.0, param);
        // circle(param, d, self.anchor.x, self.anchor.y, 32.0);
        // circle(param, d, self.bob.x, self.bob.y, 64.0);

        for s in &mut self.springs {
            s.update();
            //s.show();
        }
        no_fill(param);
        stroke3(252.0, 238.0, 33.0, param);
        stroke_weight(8.0, param);

        begin_shape(ModosBeginShape::Close);

        let head = self.particles.get(0).unwrap();
        vertex(head.position.x, head.position.y, param);
        for p in &mut self.particles {
            p.apply_force(self.gravity);
            p.update();
            vertex(p.position.x, p.position.y, param);
            //p.show();
        }
        let mut tail = self
            .particles
            .get(self.particles.len() - 1)
            .unwrap()
            .clone();
        vertex(tail.position.x, tail.position.y, param);
        end_shape(d, param, ModosBeginShape::Close);

        fill3(45.0, 197.0, 244.0, param);
        circle(param, d, tail.position.x, tail.position.y, 64.0);

        if mouse_is_pressed(param) {
            tail.position
                .set(param.mouse_posicion.x, param.mouse_posicion.y, 0.0);
            tail.velocity.set(0.0, 0.0, 1.0);
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
