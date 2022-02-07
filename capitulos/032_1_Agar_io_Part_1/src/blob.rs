use raylib::prelude::*;

use libfinal::engine::Engine;
use libfinal::matem::lerp;
use libfinal::p5::color_p5::fill1;
use libfinal::p5::vector_p5::Vector3;
use libfinal::parametros::Parametros;
use libfinal::shapes::ellipse;

pub struct Blob {
    pub pos: Vector3,
    pub r: f32,
    pub vel: Vector3,
}

impl Blob {
    pub fn new(engine: &mut Engine, x: f32, y: f32, r: f32) -> Blob {
        //println!("en Blob::new   w = {:#?}", w);
        Blob {
            pos: Vector3::new(x, y, 0.0),
            r: r,
            vel: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn update(&mut self, param: &mut Parametros) {
        let mut newvel = Vector3::new(
            param.mouse_posicion.x - param.ancho / 2.0,
            param.mouse_posicion.y - param.alto / 2.0,
            0.0,
        );
        //println!("1111En Blob::update   vel = {:#?}", vel);
        //vel.sub(&self.pos);
        //println!("2222En Blob::update   vel = {:#?}", vel);
        newvel.set_mag(3.0);
        self.vel.lerp(newvel, 0.2);
        //println!("33333En Blob::update   vel = {:#?}", vel);

        self.pos.add(&self.vel);
    }

    pub fn eats(&mut self, other: &mut Blob) -> bool {
        let d = Vector3::dist_s(&self.pos, &other.pos);
        if d < (self.r + other.r) {
            let sum = PI as f32 * self.r * self.r + PI as f32 * other.r * other.r;
            self.r = (sum / PI as f32).sqrt();
            //self.r += other.r;
            return true;
        } else {
            return false;
        }
    }

    pub fn show(&mut self, engine: &mut Engine, d: &mut RaylibDrawHandle) {
        fill1(255.0, &mut engine.param);
        //println!("en blob:show   matriz_total = {:#?}", engine.param.matriz_total);

        ellipse(
            &mut engine.param,
            d,
            self.pos.x,
            self.pos.y,
            self.r * 2.0,
            self.r * 2.0,
        );
        //        ellipse(engine.canvas.as_mut().unwrap(), &mut engine.param,
        //                self.pos.x, self.pos.y, self.r * 2.0, self.r * 2.0);
    }
}
