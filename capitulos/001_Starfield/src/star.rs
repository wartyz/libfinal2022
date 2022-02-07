use raylib::prelude::*;

use libfinal::engine::Engine;
use libfinal::matem::*;
use libfinal::p5::color_p5::{fill, no_stroke, stroke1, Color};
use libfinal::parametros::Parametros;
use libfinal::shapes::*;

//use libfinal::utiles;
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
    pub fn new(param: &Parametros) -> Star {
        let z = random(param.ancho / 2.0);
        //let z = 45.0;
        println!("en new de Star z = {:?}", z);
        Star {
            x: random_range(-param.ancho, param.ancho),
            y: random_range(-param.alto, param.alto),

            z,
            pz: z,
        }
    }

    pub fn update(&mut self, engine: &Engine, speed: f32) {
        //println!("mainstate.speed en update={:?}", mainstate.speed);
        self.z = self.z - speed;

        //println!("speed en star.update={:?}", speed);
        //println!("self.z en star.update={:?}", self.z);
        if self.z < 1.0 {
            //println!("self.z en star.update < 1.0 ={:?}", self.z);

            self.z = engine.param.ancho / 2.0;
            self.x = random_range(-engine.param.ancho / 2.0, engine.param.ancho / 2.0);
            self.y = random_range(-engine.param.alto / 2.0, engine.param.alto / 2.0);

            self.pz = self.z;
        }
    }

    pub fn show(mut self, engine: &mut Engine, d: &mut RaylibDrawHandle) {
        //pub fn show(mut self, engine: &mut Engine) {
        //let mut d = rl.begin_drawing((&th));
        fill(Color::new(255, 255, 0, 255), &mut engine.param);
        no_stroke(&mut engine.param);

        let sx = mapa(self.x / self.z, 0.0, 1.0, 0.0, engine.param.ancho / 2.0);
        let sy = mapa(self.y / self.z, 0.0, 1.0, 0.0, engine.param.alto / 2.0);

        //println!("self.z en star.show = {:?}", self.z);
        //println!("sx en star.show = {:?}", sx);

        let r = mapa(self.z, 0.0, engine.param.ancho / 2.0, 16.0, 0.0);

        //let mut cv = engine.canvas.clone().as_ref();
        //let mut cv = utiles::kk2(engine);
        ellipse(&mut engine.param, d, sx, sy, r, r);

        //        ellipse(
        //            engine.canvas.as_mut().unwrap(),
        //            &mut engine.param,
        //            sx,
        //            sy,
        //            r,
        //            r,
        //        );

        let px = mapa(self.x / self.pz, 0.0, 1.0, 0.0, engine.param.ancho / 2.0);
        let py = mapa(self.y / self.pz, 0.0, 1.0, 0.0, engine.param.alto / 2.0);

        self.pz = self.z;

        stroke1(255.0, &mut engine.param);
        //println!("en star-show");

        line(d, &mut engine.param, px, py, sx, sy);

        //let mut d = &mut engine.rl.begin_drawing(&engine.th);
        //shapes::line(engine, 120.0, 25.0, 35.0 * 2.5, 40.0 * 2.5);
        //shapes::line(engine, px, py, sx * 2.0, sy * 2.0);
        //line(&mut d, &mut engine.param, px, py, sx * 2.0, sy * 2.0);
    }
}
