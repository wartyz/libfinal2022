use raylib::prelude::*;

use libfinal::constantes::TWO_PI;
//use libfinal::dom::Slider;
use libfinal::engine::Engine;
//use libfinal::environment::frame_rate;
use libfinal::matem::{floor, random, random_usize};
use libfinal::p5::color_p5::{background, fill1, fill3, no_stroke, stroke1};
use libfinal::parametros::CodigosTecla;
//use libfinal::rendering::createcanvas;
use libfinal::shapes::{ellipse, line, stroke_weight};
use libfinal::structure::{pop, push};
use libfinal::transform::{push_matrix, rotate_z, translate};
use libfinal::typography::{text, text_size};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
use std::thread::current;

use crate::cell::Cell;

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 400;
pub const ALTO: i32 = 400;

// Aqui vendrá el pseudocódigo javascript

pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    angle: f32,
    //    slider: Slider,
    //cantidad_de_sliders: i32,
}

impl Sketch {
    pub fn new() -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);
        //let alto = engine.param.alto;
        //let ancho = engine.param.ancho;

        Sketch {
            engine,
            angle: 0.0,
            //            slider: Slider::new(0.0, alto - 20.0,
            //                                0.0, TWO_PI,
            //                                20.0, ancho / 2.0),
            //cantidad_de_sliders: 0,
        }
    }

    // Función setup() de javascript
    pub fn setup(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {
        //createcanvas(&mut self.engine, ANCHO, ALTO);
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
        stroke1(255.0, &mut self.engine.param);
        translate(200.0, ALTO as f32, &mut self.engine.param);
        self.branch(100.0, d);

        // Draw GUI controls
        //------------------------------------------------------------------------------
        self.angle = d.gui_slider_bar(
            Rectangle::new(0.0, 40.0, 120.0, 20.0),
            None,
            Some(rstr!("valor")),
            self.angle,
            0.0,
            PI as f32 * 2.0,
        );

        ////        self.angle = self.slider.value(self.engine);
        //        stroke1(255.0, &mut self.engine.param);
        //
        //        translate(200.0, self.engine.param.alto, &mut self.engine.param);
        //        //translate(200.0, ALTO as f32, &mut self.engine.param);
        //        self.branch(100.0, d);
    }
    pub fn branch(&mut self, len: f32, d: &mut RaylibDrawHandle) {
        line(d, &mut self.engine.param, 0.0, 0.0, 0.0, -len);
        translate(0.0, -len, &mut self.engine.param);

        if len > 4.0 {
            push(&mut self.engine);
            rotate_z(self.angle, &mut self.engine.param);
            self.branch(len * 0.67, d);
            pop(&mut self.engine);

            push(&mut self.engine);
            rotate_z(-self.angle, &mut self.engine.param);
            self.branch(len * 0.67, d);
            pop(&mut self.engine);
        }

        //line(d, &mut self.engine.param, 0.0, 0.0, 0.0, -len * 0.67);
        //        //line(&mut self.engine.canvas.as_mut().unwrap(), &mut self.engine.param, 0.0, 0.0, 0.0, -leng);
        //
        //
        //        translate(0.0, -leng, &mut self.engine.param);
        //
        //        if leng > 4.0 {
        //            push(&mut self.engine);
        //            //fi.RotateZporUnPuntoTrasladado(s.angle, s.Engine.Param)
        //            rotate_z(self.angle, &mut self.engine.param);
        //            self.branch(leng * 0.67, d);
        //            pop(&mut self.engine);
        //
        //            push(&mut self.engine);
        //            //fi.RotateZporUnPuntoTrasladado(-s.angle, s.Engine.Param)
        //            rotate_z(-self.angle, &mut self.engine.param);
        //            self.branch(leng * 0.67, d);
        //            pop(&mut self.engine);
        //        }

        //Line(0, 0, 0, -leng*0.67)
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
