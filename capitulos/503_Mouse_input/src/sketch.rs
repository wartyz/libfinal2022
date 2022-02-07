use raylib::prelude::*;

use libfinal::constantes::*;
use libfinal::engine::Engine;
use libfinal::matem::{random, random_i32, random_usize};
use libfinal::p5::color_p5::{fill, fill3, Color};
use libfinal::p5::image_p5::P5Image;
use libfinal::parametros::*;
use libfinal::shapes::{circle, ellipse, rect, stroke_weight};
use libfinal::typography::{text, text_size};

//use crate::vehicle::Vehicle;

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 1200;
pub const ALTO: i32 = 800;

// Constantes del sketch
pub const COLS: usize = ANCHO as usize;
pub const ROWS: usize = ALTO as usize;

// Aqui vendrá el pseudocódigo javascript

pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    //    current: [[Color_fi; ANCHO as usize]; ALTO as usize],
    //    previous: [[Color_fi; ANCHO as usize]; ALTO as usize],
    ball_color: Color,
    ball_position: Vector2,
}

impl Sketch {
    pub fn new() -> Sketch {
        let mut engine = Engine::new(ANCHO as f32, ALTO as f32);
        //let ball_position = Vector2::new(ancho / 2.0, alto / 2.0);
        let ball_color = Color::new(0, 0, 0, 255);
        let ball_position = Vector2::new(ANCHO as f32 / 2.0, ALTO as f32 / 2.0);

        //        for c in 0..COLS {
        //            for r in 0..ROWS {
        //                current.push(Color::new(0, 0, 0, 255));
        //                previous.push(Color::new(0, 0, 0, 255));
        //            }
        //        }

        //        let mut current = [[Color_fi::new(0, 0, 0, 255); COLS]; ROWS];
        //        let mut previous = [[Color_fi::new(0, 0, 0, 255); COLS]; ROWS];

        Sketch {
            engine,
            ball_color,
            ball_position,
        }
    }

    // Función setup() de javascript
    pub fn setup(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {
        //        //createcanvas(&mut self.engine, ANCHO, ALTO);
        //        for j in 0..self.rows {
        //            for i in 0..self.cols {
        //                self.grid[j][i] = random_usize(2);
        //            }
        //        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) -> bool {
        //        match self.engine.param.key {
        //            CodigosTecla::RightArrow => self.ball_position.x += 2.0,
        //            CodigosTecla::LeftArrow => self.ball_position.x -= 2.0,
        //            CodigosTecla::UpArrow => self.ball_position.y -= 2.0,
        //            CodigosTecla::DownArrow => self.ball_position.y += 2.0,
        //            _ => {}
        //        }
        self.ball_position = self.engine.param.mouse_posicion;
        match self.engine.param.mouse_boton {
            CodigosRaton::Izquierdo => self.ball_color = Color::MAROON,
            CodigosRaton::Medio => self.ball_color = Color::LIME,
            CodigosRaton::Derecho => self.ball_color = Color::DARKBLUE,
            _ => {}
        }

        if !self.engine.update(rl, th) {
            return false;
        }
        true
    }

    // Función draw() de javascript
    pub fn draw(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {
        let mut d = rl.begin_drawing(&th);

        //background(&mut self.engine, &mut d, 150);

        fill3(0.0, 102.0, 153.0, &mut self.engine.param);
        text(
            &mut self.engine.param,
            &mut d,
            "Congrats! You created your first window!",
            190.0,
            200.0,
        );

        //println!("en draw: self.ball_color = {:#?}", self.ball_color);
        fill(self.ball_color, &mut self.engine.param);
        circle(
            &mut self.engine.param,
            &mut d,
            self.ball_position.x,
            self.ball_position.y,
            50.0,
        );
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
