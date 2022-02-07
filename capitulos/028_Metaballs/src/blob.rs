use raylib::core::drawing::RaylibDrawHandle;

use libfinal::matem::random_range;
use libfinal::p5::color_p5::{fill, no_fill, stroke1, Color};
use libfinal::parametros::CodigosTecla::Alt;
use libfinal::parametros::Parametros;
use libfinal::shapes::{ellipse, stroke_weight};

use crate::sketch::{ALTO, ANCHO};
use libfinal::p5::vector_p5::Vector3;

pub struct Blob {
    pub pos: Vector3,
    pub r: f32,
    vel: Vector3,
}

impl Blob {
    pub fn new(x: f32, y: f32) -> Blob {
        let mut vel = Vector3::random2d_s();
        vel.mult(random_range(2.0, 5.0));
        //let r = random_range(20.0, 120.0);
        let r = 40.0;
        Blob {
            pos: Vector3::new(x, y, 0.0),
            r,
            vel,
        }
    }

    pub fn update(&mut self) {
        self.pos.add(&self.vel);

        if self.pos.x > ANCHO as f32 || self.pos.x < 0.0 {
            self.vel.x *= -1.0;
        }

        if self.pos.y > ALTO as f32 || self.pos.y < 0.0 {
            self.vel.y *= -1.0;
        }
    }

    pub fn show(&mut self, d: &mut RaylibDrawHandle, param: &mut Parametros) {
        //no_fill(param);
        fill(Color::new(0, 0, 0, 255), param);
        stroke1(0.0, param);
        stroke_weight(4.0, param);
        ellipse(param, d, self.pos.x, self.pos.y, self.r * 2.0, self.r * 2.0);
    }
}
