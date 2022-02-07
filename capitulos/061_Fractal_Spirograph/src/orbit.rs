use raylib::prelude::*;

use libfinal::engine::Engine;
use libfinal::matem::{pow, radians, random, random_range};
use libfinal::p5::color_p5::{no_fill, stroke1, stroke2};
use libfinal::parametros::Parametros;
use libfinal::shapes::{ellipse, stroke_weight};
use std::cell::RefCell;
use std::rc::Rc;

//pub type SingleLink = Option<Rc<RefCell<Orbit>>>;

#[derive(Clone, Copy, Debug)]
pub struct Orbit {
    k: f32,
    pub x: f32,
    pub y: f32,
    r: f32,

    //parent: Option<usize>,
    //child: Option<usize>,
    n: i32,
    pub mipos: Option<usize>,
    pub parent: Option<usize>,
    pub child: Option<usize>,
    speed: f32,
    angle: f32,
}

impl Orbit {
    pub fn new3(x: f32, y: f32, r: f32, n: i32, resolution: f32,
                mipos: Option<usize>, parent: Option<usize>) -> Orbit {
        Orbit::new5(x, y, r, n, resolution, mipos, parent)
    }
    pub fn new5(x: f32, y: f32, r: f32, n: i32, resolution: f32, mipos: Option<usize>, parent: Option<usize>) ->

    Orbit {
        let k: f32 = -4.0;
        //let speed = radians(k.powf(n - 1.0));
        let speed = radians(k.powi(n - 1)) / resolution;
        Orbit {
            k,
            x,
            y,
            r,
            n,
            mipos,
            parent,
            child: None,
            speed,
            angle: -PI as f32 / 2.0,
        }
    }

    pub fn update(mut self, parent: Option<Orbit>, d: &mut RaylibDrawHandle,
                  param: &mut Parametros) -> Orbit {
        if parent.is_some() {
            self.angle += self.speed;
            //dbg!(self.angle);
            let rsum = self.r + parent.unwrap().r;
            self.x = parent.unwrap().x + rsum * self.angle.cos();
            self.y = parent.unwrap().y + rsum * self.angle.sin();

            //ellipse(param, d, x, y, self.r * 2.0, self.r * 2.0);
        }
        self
    }

    pub fn show(&mut self, engine: &mut Engine, d: &mut RaylibDrawHandle) {
        stroke2(255.0, 100.0, &mut engine.param);
        stroke_weight(2.0, &mut engine.param);
        no_fill(&mut engine.param);
        ellipse(&mut engine.param, d, self.x, self.y, self.r * 2.0, self.r * 2.0);
//        ellipse(&mut engine.canvas.as_mut().unwrap(),
//                &mut engine.param,
//                self.x,
//                self.y,
//                self.r * 2.0,
//                self.r * 2.0);
    }

    pub fn add_child(&self, resolution: f32, nuevapos: Option<usize>) -> Orbit {
        let newr = self.r / 3.0;
        let newx = self.x + self.r + newr;
        let newy = self.y;
        let ch = Orbit::new5(newx, newy, newr, self.n + 1, resolution, nuevapos, self.mipos);

        ch
    }
}
//    pub fn new4(x: f32, y: f32, r: f32, n: i32, resolution: f32) -> Orbit {
//        Orbit::new5(x, y, r, n, &None, resolution)
//    }
//    pub fn new5(x: f32, y: f32, r: f32, n: i32, p: &SingleLink, resolution: f32) -> Orbit {
//        let k = -4.0;

//        Orbit {
//            k,
//            x,
//            y,
//            r,
//            n,
////            parent: p.clone(),
////            child: None,
//            speed: radians(pow(k, (n - 1)) / resolution),
//            angle: -PI as f32 / 2.0,
//        }


//
//pub fn show(&mut self, engine: &mut Engine, d: &mut RaylibDrawHandle) {
//    stroke1(255.0, &mut engine.param);
//    stroke_weight(2.0, &mut engine.param);
//    no_fill(&mut engine.param);
//    ellipse(&mut engine.param, d, self.x, self.y, self.r * 2.0, self.r * 2.0);
////        ellipse(&mut engine.canvas.as_mut().unwrap(),
////                &mut engine.param,
////                self.x,
////                self.y,
////                self.r * 2.0,
////                self.r * 2.0);
//}
//}