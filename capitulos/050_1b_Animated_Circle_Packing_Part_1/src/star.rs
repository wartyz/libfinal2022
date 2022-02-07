use raylib::prelude::*;

use libfinal::engine::Engine;
use libfinal::matem::*;
use libfinal::p5::color_p5::{fill, no_stroke, stroke, Color};
use libfinal::parametros::Parametros;
use libfinal::shapes::*;
use libfinal::utiles;

//use libfinal::constantes::{WIDTH, HEIGHT};
//use libfinal::MainState;

#[derive(Debug, Copy, Clone)]
pub struct Star {
    x: f32,
    y: f32,
    z: f32,
    pz: f32,
}

impl Star {
    pub fn new(param: &mut Parametros) -> Star {
        let z = random(param.ancho);
        println!("en new de Star z = {:?}", z);
        Star {
            x: random_range(-param.ancho, param.ancho),
            y: random_range(-param.alto, param.alto),
            z: z,
            pz: z,
        }
    }

    pub fn update(mut self, param: &mut Parametros, speed: &f32) {
        //println!("mainstate.speed en update={:?}", mainstate.speed);
        self.z = self.z - speed.clone();
        //println!("speed en star.update={:?}", speed);
        println!("self.z en star.update={:?}", self.z);
        if self.z < 1.0 {
            //println!("self.z en star.update < 1.0 ={:?}", self.z);
            self.z = param.ancho;
            self.x = random_range(-param.ancho, param.ancho);
            self.y = random_range(-param.alto, param.alto);

            self.pz = self.z;
        }
    }

    pub fn show(mut self, engine: &mut Engine, d: &mut RaylibDrawHandle) {
        fill(Color::new(255, 255, 255, 255), &mut engine.param);
        no_stroke(&mut engine.param);

        let sx = mapa(self.x / self.z, 0.0, 1.0, 0.0, engine.param.ancho);
        let sy = mapa(self.y / self.z, 0.0, 1.0, 0.0, engine.param.alto);

        println!("self.z en star.show = {:?}", self.z);
        //println!("sx en star.show = {:?}", sx);

        let r = mapa(self.z, 0.0, engine.param.ancho, 16.0, 0.0);

        //let mut cv = engine.canvas.clone().as_ref();
        //let mut cv = utiles::kk2(engine);
        ellipse(&mut engine.param, d, sx, sy, r, r);
        //ellipse(engine.canvas.as_mut().unwrap(), &mut engine.param, sx, sy, r, r);

        let px = mapa(self.x / self.pz, 0.0, 1.0, 0.0, engine.param.ancho);
        let py = mapa(self.y / self.pz, 0.0, 1.0, 0.0, engine.param.alto);

        self.pz = self.z;

        stroke(Color::new(255, 255, 255, 255), &mut engine.param);
        line(d, &mut engine.param, px, py, sx * 2.0, sy * 2.0);

        //line(&mut engine.canvas.as_mut().unwrap(), &mut engine.param, px, py, sx * 2.0, sy * 2.0);
    }
}
