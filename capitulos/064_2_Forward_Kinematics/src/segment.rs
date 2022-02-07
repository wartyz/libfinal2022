use raylib::prelude::*;

use libfinal::engine::Engine;
use libfinal::matem::{mapa, noise1, noise2, random};
use libfinal::p5::color_p5::stroke1;
use libfinal::p5::vector_p5::Vector3;
use libfinal::shapes::{line, stroke_weight};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Segment {
    a: Vector3,
    len: f32,
    angle: f32,
    self_angle: f32,

    xoff: f32,

    pub parent: Option<Rc<RefCell<Segment>>>,
    pub child: Option<Rc<RefCell<Segment>>>,

    b: Vector3,
}

impl Segment {
    pub fn new3(parent: Option<Rc<RefCell<Segment>>>, len: f32, angle: f32, t: f32) -> Segment {
        //let a = Vector2::new(parent.clone().unwrap().b.x, parent.clone().unwrap().b.y);
        //let a = parent.clone().unwrap().b;
        //        let a1 = parent.clone();
        //        let a2 = a1.unwrap();
        //        let a3 = a2.borrow();
        //        let a = a3.b;
        let a = parent.clone().unwrap().borrow_mut().b;
        Segment {
            a,
            len,
            angle,
            self_angle: angle,
            xoff: random(1000.0),

            parent,
            child: None,

            b: Segment::calculate_b(a, len, angle),
        }
    }

    pub fn new4(x: f32, y: f32, len: f32, angle: f32, t: f32) -> Segment {
        let a = Vector3::new(x, y, 0.0);
        Segment {
            a,
            len,
            angle,
            self_angle: angle,
            xoff: random(1000.0),

            parent: None,
            child: None,

            b: Segment::calculate_b(a, len, angle),
        }
    }

    pub fn wiggle(&mut self) {
        let maxangle = 1.0;
        let minangle = -1.0;
        //  / 10.0 lo pongo yo
        self.self_angle = mapa(noise1(self.xoff) / 10.0, -1.0, 1.0, maxangle, minangle);
        self.xoff += 0.03;
        //self.self_angle += 0.001;
    }

    pub fn update(&mut self) {
        self.angle = self.self_angle;
        match &self.parent {
            Some(s) => {
                self.a = self.parent.clone().unwrap().borrow().b;

                self.angle += self.parent.clone().unwrap().borrow().angle;
            }
            None => {
                self.angle += -PI as f32 / 2.0;
            }
        }

        self.b = Segment::calculate_b(self.a, self.len, self.angle);
    }

    pub fn calculate_b(a: Vector3, len: f32, angle: f32) -> Vector3 {
        let dx = len * angle.cos();
        let dy = len * angle.sin();

        Vector3::new(a.x + dx, a.y + dy, 0.0)
    }

    pub fn show(&mut self, engine: &mut Engine, d: &mut RaylibDrawHandle) {
        stroke1(255.0, &mut engine.param);
        stroke_weight(4.0, &mut engine.param);
        line(d, &mut engine.param, self.a.x, self.a.y, self.b.x, self.b.y);
        //line(engine.canvas.as_mut().unwrap(), &mut engine.param, self.a.x, self.a.y, self.b.x,
        //      self.b.y);
    }
}
