use raylib::prelude::*;

use libfinal::engine::Engine;
use libfinal::matem::{random, random_usize};
use libfinal::p5::color_p5::{background, fill1, fill3, no_stroke, stroke1};
use libfinal::p5::vector_p5::Vector3;
use libfinal::parametros::CodigosTecla;
use libfinal::shapes::{ellipse, stroke_weight};
use libfinal::typography::{text, text_size};

use crate::vehicle::Vehicle;

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 1280;
pub const ALTO: i32 = 720;

// Aqui vendrá el pseudocódigo javascript

pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    vehicles: Vec<Vehicle>,
    food: Vec<Vector3>,
    poison: Vec<Vector3>,
}

impl Sketch {
    //    pub fn new(ancho: f32, alto: f32) -> Sketch {
    //        let mut engine = Engine::new(ancho, alto);
    pub fn new() -> Sketch {
        let mut engine = Engine::new(ANCHO as f32, ALTO as f32);

        Sketch {
            engine,
            vehicles: vec![],
            food: vec![],
            poison: vec![],
        }
    }

    // Función setup() de javascript
    pub fn setup(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {
        //createcanvas(&mut self.engine, ANCHO, ALTO);

        for i in 0..200 {
            let x = random(ANCHO as f32);
            let y = random(ALTO as f32);
            self.vehicles.push(Vehicle::new(x, y));
        }

        for i in 0..100 {
            let x = random(ANCHO as f32);
            let y = random(ALTO as f32);

            self.food.push(Vector3::new(x, y, 0.0));
        }

        for i in 0..20 {
            let x = random(ANCHO as f32);
            let y = random(ALTO as f32);

            self.poison.push(Vector3::new(x, y, 0.0));
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) -> bool {
        if !self.engine.update(rl, th) {
            return false;
        }
        true
    }

    // Función draw() de javascript
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        background(&mut self.engine, d, 51);

        if random(1.0) < 0.05 {
            let x = random(ANCHO as f32);
            let y = random(ALTO as f32);

            self.food.push(Vector3::new(x, y, 0.0));
        }

        let target = Vector3::new(
            self.engine.param.mouse_posicion.x,
            self.engine.param.mouse_posicion.y,
            0.0,
        );

        //        fill1(127.0, &mut self.engine.param);
        //        stroke1(200.0, &mut self.engine.param);
        //        stroke_weight(2.0, &mut self.engine.param);
        //        ellipse(&mut self.engine.canvas.as_mut().unwrap(),
        //                &mut self.engine.param,
        //                target.x, target.y, 48.0, 48.0);

        for i in 0..self.food.len() {
            fill3(0.0, 255.0, 0.0, &mut self.engine.param);
            no_stroke(&mut self.engine.param);
            ellipse(
                &mut self.engine.param,
                d,
                self.food[i].x,
                self.food[i].y,
                8.0,
                8.0,
            );
            //            ellipse(&mut self.engine.canvas.as_mut().unwrap(),
            //                    &mut self.engine.param,
            //                    self.food[i].x, self.food[i].y, 8.0, 8.0);
        }

        for i in 0..self.poison.len() {
            fill3(255.0, 0.0, 0.0, &mut self.engine.param);
            no_stroke(&mut self.engine.param);
            ellipse(
                &mut self.engine.param,
                d,
                self.poison[i].x,
                self.poison[i].y,
                8.0,
                8.0,
            );
            //            ellipse(&mut self.engine.canvas.as_mut().unwrap(),
            //                    &mut self.engine.param,
            //                    self.poison[i].x, self.poison[i].y, 8.0, 8.0);
        }

        for i in (0..self.vehicles.len()).rev() {
            self.vehicles[i].behaviors(&mut self.food, &mut self.poison);
            self.vehicles[i].update();
            self.vehicles[i].display(&mut self.engine, d);

            if self.vehicles[i].dead() {
                self.vehicles.remove(i);
            }
        }

        //frame_rate(5);
    }

    pub fn key_pressed(&mut self) {
        //        if self.engine.param.key == CodigosTecla::A {
        //            self.left.mover(-10.0);
        //        } else if self.engine.param.key == CodigosTecla::Z {
        //            self.left.mover(10.0)
        //        } else if self.engine.param.key == CodigosTecla::J {
        //            self.right.mover(-10.0)
        //        } else if self.engine.param.key == CodigosTecla::M {
        //            self.right.mover(10.0)
        //        }
    }

    pub fn key_released(&mut self) {
        //        if self.engine.param.keyr == CodigosTecla::A {
        //            self.left.mover(0.0);
        //        } else if self.engine.param.keyr == CodigosTecla::Z {
        //            self.left.mover(0.0)
        //        } else if self.engine.param.keyr == CodigosTecla::J {
        //            self.right.mover(0.0)
        //        } else if self.engine.param.keyr == CodigosTecla::M {
        //            self.right.mover(0.0)
        //        }
    }
}
