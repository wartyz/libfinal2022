use raylib::prelude::*;

use libfinal::engine::Engine;
use libfinal::matem::{constrain, floor, mapa, random, random_range};
use libfinal::p5::color_p5::{
    background, colormode, fill1, fill2, fill3, fill4, no_stroke, stroke1, stroke2, stroke3,
};
use libfinal::p5::vector_p5::Vector3;
use libfinal::parametros::Parametros;
use libfinal::shapes::{ellipse, point, stroke_weight};

#[derive(Debug, Clone, Copy)]
pub struct Walker {
    pos: Vector3,
    stuck: bool,
    pub r: f32,
    hu: f32,
}

impl Walker {
    pub fn new(engine: &mut Engine) -> Walker {
        Walker {
            pos: random_point(engine),
            stuck: false,
            r: 8.0,
            hu: 0.0,
        }
    }
    pub fn new1(engine: &mut Engine, radius: f32) -> Walker {
        Walker {
            pos: random_point(engine),
            stuck: false,
            r: radius,
            hu: 0.0,
        }
    }
    pub fn new3(engine: &mut Engine, x: f32, y: f32, radius: f32) -> Walker {
        Walker {
            pos: Vector3::new(x, y, 0.0),
            stuck: true,
            r: radius,
            hu: 0.0,
        }
    }
    pub fn new4(engine: &mut Engine, x: f32, y: f32, stuck: bool, radius: f32) -> Walker {
        Walker {
            pos: Vector3::new(x, y, 0.0),
            stuck,
            r: x,
            hu: 0.0,
        }
    }

    pub fn walk(&mut self, engine: &mut Engine) {
        let vel = Vector3::random2d_s();
        //let vel = Vector2::new(random_range(-1.0, 1.0), random_range(-0.5, 1.0));
        self.pos.add(&vel);
        self.pos.x = constrain(self.pos.x, 0.0, engine.param.ancho);
        self.pos.y = constrain(self.pos.y, 0.0, engine.param.alto);
    }

    pub fn check_stuck(&mut self, others: &Vec<Walker>) -> bool {
        for i in 0..others.len() {
            let d = dist_sq(&self.pos, &others[i].pos);
            if d < (self.r * self.r + others[i].r * others[i].r + 2.0 * others[i].r * self.r) {
                //if random(1.0) < 0.1 {
                self.stuck = true;
                return true;
                //break;
                //}
            }
        }
        return false;
    }

    pub fn set_hue(&mut self, hu: f32) {
        self.hu = hu;
    }

    pub fn show(&mut self, engine: &mut Engine, d: &mut RaylibDrawHandle) {
        let p = &mut engine.param; // Nuevo invento ***********
        no_stroke(p);
        if self.stuck {
            fill4(self.hu, 255.0, 100.0, 200.0, p);
        } else {
            fill3(360.0, 0.0, 255.0, p);
        }
        ellipse(
            &mut engine.param,
            d,
            self.pos.x,
            self.pos.y,
            self.r * 2.0,
            self.r * 2.0,
        );
        //        ellipse(&mut engine.canvas.as_mut().unwrap(),
        //                p,
        //                self.pos.x,
        //                self.pos.y,
        //                self.r * 2.0,
        //                self.r * 2.0);
    }
}

pub fn random_point(engine: &mut Engine) -> Vector3 {
    let w = engine.param.ancho;
    let h = engine.param.alto;

    let i = floor(random(4.0)) as i32;

    match i {
        0 => {
            let x = random(w);
            Vector3::new(x, 0.0, 0.0)
        }
        1 => {
            let x = random(w);
            Vector3::new(x, h, 0.0)
        }
        2 => {
            let y = random(h);
            Vector3::new(0.0, y, 0.0)
        }
        3 => {
            let y = random(h);
            Vector3::new(w, y, 0.0)
        }
        _ => {
            panic!("fallo en random_point")
        }
    }
}

pub fn dist_sq(a: &Vector3, b: &Vector3) -> f32 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    dx * dx + dy * dy
}
