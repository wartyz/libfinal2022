use raylib::prelude::*;

use libfinal::engine::Engine;
use libfinal::matem::random_range;
use libfinal::p5::color_p5::*;
use libfinal::shapes::ellipse;

pub struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    alpha: f32,
}

impl Particle {
    pub fn new() -> Particle {
        Particle {
            x: 300.0,
            y: 380.0,
            vx: random_range(-1.0, 1.0),
            vy: random_range(-5.0, -1.0),
            alpha: 255.0,
        }
    }
    pub fn finished(&mut self) -> bool {
        self.alpha <= 0.0
    }

    pub fn update(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        self.alpha -= 5.0;
        //if p.alpha < 0 { // lo pongo yo
        //    p.alpha = 0
        //}
    }

    pub fn show(&mut self, engine: &mut Engine, d: &mut RaylibDrawHandle) {
        //fi.Stroke1(255, engine.Param)
        no_stroke(&mut engine.param);
        //fi.Stroke1(255, engine.Param)
        fill2(255.0, self.alpha, &mut engine.param);
        ellipse(&mut engine.param, d, self.x, self.y, 16.0, 16.0);
    }
}