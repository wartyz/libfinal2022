use raylib::core::drawing::RaylibDrawHandle;
use raylib::core::math::Vector2;

use libfinal::matem::{constrain, random_range};
use libfinal::p5::color_p5::*;
use libfinal::p5::vector_p5::Vector3;
use libfinal::parametros::Parametros;
use libfinal::shapes::*;

pub struct Particle {
    pos: Vector3,
    prev: Vector3,
    vel: Vector3,
    acc: Vector3,
}

impl Particle {
    pub fn new(x: f32, y: f32) -> Particle {
        //et mut vel = Vector2::random2d_s();
        //vel.set_mag(random_range(2.0, 5.0));
        Particle {
            pos: Vector3::new(x, y, 0.0),
            prev: Vector3::new(x, y, 0.0),
            vel: Vector3::random2d_s(),
            acc: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn update(&mut self) {
        self.vel.add(&self.acc);
        self.vel.limit(5.0);
        self.pos.add(&self.vel);
        self.acc.mult(0.0);
    }

    pub fn show(&mut self, d: &mut RaylibDrawHandle, param: &mut Parametros) {
        stroke2(255.0, 255.0, param);
        stroke_weight(4.0, param);
        //point(d, param, self.pos.x, self.pos.y);
        line(d, param, self.pos.x, self.pos.y, self.prev.x, self.prev.y);

        self.prev.x = self.pos.x;
        self.prev.y = self.pos.y;
    }

    pub fn attracted(&mut self, target: Vector3) {
        // dir = target - this.pos
        let mut force = Vector3::sub_s(&target, &self.pos);
        let mut d = force.mag();

        d = constrain(d, 0.1, 25.0);
        let g = 50.0; // 6.67408;
        let strength = g / (d * d);
        force.set_mag(strength);
        if d < 20.0 {
            force.mult(-10.0);
        }

        self.acc.add(&force);
    }
}
