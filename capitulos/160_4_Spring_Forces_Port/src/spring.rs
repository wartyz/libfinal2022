use crate::particle::Particle;
use libfinal::p5::color_p5::stroke1;
use libfinal::p5::vector_p5::Vector3;
use libfinal::parametros::Parametros;
use libfinal::shapes::{line, stroke_weight};
use raylib::prelude::RaylibDrawHandle;

pub struct Spring {
    k: f32,
    rest_length: f32,
    a: Particle,
    b: Particle,
}

impl Spring {
    pub fn new(k: f32, rest_length: f32, a: Particle, b: Particle) -> Spring {
        Spring {
            k,
            rest_length,
            a,
            b,
        }
    }
    pub fn update(&mut self) {
        let mut force = Vector3::sub_s(&self.b.position, &self.a.position);

        let x = force.mag() - self.rest_length;
        force.normalize();
        force.mult(self.k * x);
        self.b.apply_force(force);
        force.mult(-1.0);
        self.a.apply_force(force);
    }

    pub fn show(&mut self, d: &mut RaylibDrawHandle, param: &mut Parametros) {
        stroke_weight(4.0, param);
        stroke1(255.0, param);
        line(
            d,
            param,
            self.a.position.x,
            self.a.position.y,
            self.b.position.x,
            self.b.position.y,
        );
        // fill3(45.0, 197.0, 244.0, param);
        // ellipse(param, d, self.position.x, self.position.y, 64.0, 64.0);
    }
}
