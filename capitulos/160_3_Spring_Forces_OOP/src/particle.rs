use libfinal::p5::color_p5::{fill, fill1, fill2, fill3, stroke, stroke1};
use libfinal::p5::vector_p5::Vector3;
use libfinal::parametros::Parametros;
use libfinal::shapes::{circle, ellipse, stroke_weight};
use raylib::prelude::RaylibDrawHandle;

#[derive(Debug, Copy, Clone)]
pub struct Particle {
    locked: bool,
    acceleration: Vector3,
    pub velocity: Vector3,
    pub position: Vector3,
    mass: f32,
}

impl Particle {
    pub fn new(x: f32, y: f32) -> Particle {
        Particle {
            locked: false,
            acceleration: Vector3::new(0.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, 0.0, 0.0),
            position: Vector3::new(x, y, 1.0),
            mass: 1.0,
        }
    }

    pub fn apply_force(&mut self, force: Vector3) {
        let mut f = force.copy();
        f.div(self.mass);
        self.acceleration.add(&f);
    }

    pub fn update(&mut self) {
        if !self.locked {
            self.velocity.mult(0.99);
            self.velocity.add(&self.acceleration);
            self.position.add(&self.velocity);
            self.acceleration.mult(0.0);
        }
    }

    pub fn show(&mut self, d: &mut RaylibDrawHandle, param: &mut Parametros) {
        stroke1(255.0, param);
        stroke_weight(2.0, param);
        fill3(45.0, 197.0, 244.0, param);
        circle(param, d, self.position.x, self.position.y, 64.0);
    }
}
