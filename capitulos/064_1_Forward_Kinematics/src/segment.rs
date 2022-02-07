use raylib::prelude::*;

use libfinal::engine::Engine;
use libfinal::matem::{mapa, noise1, noise2, random};
use libfinal::p5::color_p5::stroke1;
use libfinal::shapes::{line, stroke_weight};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Copy)]
pub struct Segment {
    pub a: Vector2,
    len: f32,
    angle: f32,
    self_angle: f32,

    t: f32,

    parent: Option<usize>,
    pub child: Option<usize>,

    pub b: Vector2,
}

impl Segment {
    // p es el parent   px y py son x e y de parent
    pub fn new2(p: usize, px: f32, py: f32, len: f32, angle: f32, t: f32) -> Segment {
        let parent = Some(p);
        let child = None;
        let a = Vector2::new(px, py);

        // En video funcion calculate_b()
        let dx = len * angle.cos();
        let dy = len * angle.sin();
        let b = Vector2::new(a.x + dx, a.y + dy);
        Segment {
            a,
            len,
            angle,
            self_angle: angle,
            t,
            parent,
            child,
            b,
        }
    }

    pub fn new3(x: f32, y: f32, len: f32, angle: f32) -> Segment {
        Segment::new4(x, y, len, angle, 0.0)
    }

    pub fn new4(x: f32, y: f32, len: f32, angle: f32, t: f32) -> Segment {
        let a = Vector2::new(x, y);

        // En video funcion calculate_b()
        let dx = len * angle.cos();
        let dy = len * angle.sin();
        let b = Vector2::new(a.x + dx, a.y + dy);

        Segment {
            a,
            len,
            angle,
            self_angle: 0.0,
            t,
            parent: None,
            child: None,
            b,
        }
    }

    pub fn wiggle(mut self) -> Segment {
        //self.self_angle = self.self_angle + 0.01;

        let maxangle = 0.1; // pongo yo la decima parte que el video
        let minangle = -0.1;
        //dbg!(noise1(self.xoff));
        //self.self_angle = mapa(noise1(self.xoff), 0.0, 1.0, maxangle, minangle);
        self.self_angle = mapa((self.t).sin(), -1.0, 1.0, maxangle, minangle);
        //self.self_angle = -0.41;
        self.t += 0.03;

        //        //  / 10.0 lo pongo yo
        //        self.self_angle = mapa(noise1(self.xoff) / 10.0, -1.0, 1.0, maxangle, minangle);
        //        self.xoff += 0.03;
        //        //self.self_angle += 0.001;
        self
    }

    // pb es b de parent
    pub fn update(mut self, seg_parent: Segment) -> Segment {
        self.angle = self.self_angle;
        if self.parent.is_some() {
            self.a = seg_parent.b;
            //self.angle = 0.0;
            self.angle += seg_parent.angle;
        } else {
            self.angle += -PI as f32 / 2.0;
        }

        //self.angle = self.angle + 0.01;
        //dbg!(self.angle);

        // En video funcion calculate_b()
        let dx = self.len * self.angle.cos();
        let dy = self.len * self.angle.sin();
        self.b = Vector2::new(self.a.x + dx, self.a.y + dy);
        self

        //        self.angle = self.self_angle;
        //        match &self.parent {
        //            Some(s) => {
        //                self.a = self.parent.clone().unwrap().borrow().b;
        //
        //                self.angle += self.parent.clone().unwrap().borrow().angle;
        //            }
        //            None => {
        //                self.angle += -PI as f32 / 2.0;
        //            }
        //        }
        //
        //
        //        self.b = Segment::calculate_b(self.a, self.len, self.angle);
    }

    //   pub fn calculate_b(a: Vector2Fi, len: f32, angle: f32) -> Vector2Fi {
    //        let dx = len * angle.cos();
    //        let dy = len * angle.sin();
    //
    //
    //        Vector2Fi::new(a.x + dx, a.y + dy)
    // }

    pub fn show(&mut self, engine: &mut Engine, d: &mut RaylibDrawHandle) {
        stroke1(255.0, &mut engine.param);
        stroke_weight(4.0, &mut engine.param);
        line(d, &mut engine.param, self.a.x, self.a.y, self.b.x, self.b.y);
        //line(engine.canvas.as_mut().unwrap(), &mut engine.param, self.a.x, self.a.y, self.b.x,
        //      self.b.y);
    }
}
