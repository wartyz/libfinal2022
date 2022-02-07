use raylib::prelude::*;

use libfinal::constantes::TWO_PI;
use libfinal::engine::Engine;
use libfinal::matem::{random, random_i32_range, random_range, random_usize};
use libfinal::p5::color_p5::{fill1, fill2};
use libfinal::shapes::ellipse;
use libfinal::transform::{pop_matrix, push_matrix, rotate_z, translate};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Planet {
    radius: f32,
    distance: f32,
    planets: Vec<Planet>,
    angle: f32,
    orbitspeed: f32,
}

impl Planet {
    pub fn new(radius: f32, distance: f32, orbitspeed: f32, angle: f32) -> Planet {
        Planet {
            radius: radius,
            distance: distance,
            planets: vec![],
            angle: angle,
            orbitspeed: orbitspeed,
        }
    }

    pub fn orbit(&mut self) {
        self.angle += self.orbitspeed;

        for p in &mut self.planets {
            p.orbit();
        }
    }

    pub fn spawn_moons(&mut self, total: usize, level: f32) {
        for i in 0..total {
            let r = self.radius / (level * 2.0);
            let d = random_range(50.0, 150.0);
            let o = random_range(-0.1, 0.1);
            let a = random(TWO_PI);

            self.planets.push(Planet::new(r, d / level, o, a));

            //let p = Rc::new(RefCell::new(Planet::new(r, d / level, o)));
            //self.planets.push(Some(p));
            //let mut p = self.planets[i].as_ref().unwrap().borrow_mut();
            if level < 3.0 {
                let num = random_usize(4);
                self.planets[i].spawn_moons(num, level + 1.0);
            }
        }
    }

    pub fn show(&mut self, engine: &mut Engine, d: &mut RaylibDrawHandle) {
        push_matrix(&mut engine.param);
        fill2(255.0, 100.0, &mut engine.param);
        //rotate_z_por_un_punto(self.angle, self.distance, 0.0, &mut engine.param);
        // rotate_z(self.angle, &mut engine.param);

        //rotate_z_por_un_punto(self.angle, ANCHO as f32 / 2.0, ALTO as f32 / 2.0, &mut engine.param);

        //println!("en show engine.param.matriz_total = {:#?}", engine.param.matriz_total);
        //        let xx = engine.param.matriz_total.data[6];
        //        let yy = engine.param.matriz_total.data[7];

        //rotate_z_por_un_punto_trasladado(self.angle, &mut engine.param);

        rotate_z(self.angle, &mut engine.param);

        translate(self.distance, 0.0, &mut engine.param);
        ellipse(
            &mut engine.param,
            d,
            0.0,
            0.0,
            self.radius * 2.0,
            self.radius * 2.0,
        );

        for i in 0..self.planets.len() {
            let mut p = self.planets[i].clone();
            p.show(engine, d);
        }
        pop_matrix(&mut engine.param);
    }
}
