use raylib::prelude::*;

use libfinal::engine::Engine;
use libfinal::p5::color_p5::{background, fill1};
use libfinal::parametros::CodigosTecla;
//use libfinal::rendering::createcanvas;
use libfinal::typography::{text, text_size};

use crate::paddle::Paddle;
use crate::puck::Puck;

//use libfinal::p5::sound::SoundFile;

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 1200;
pub const ALTO: i32 = 800;

// Aqui vendrá el pseudocódigo javascript

pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    puck: Puck,

    //ding: SoundFile,
    left: Paddle,
    right: Paddle,

    left_score: i32,
    right_score: i32,
}

impl Sketch {
    pub fn new() -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);
        //let w = engine.param.ancho;
        //let h = engine.param.alto;

        Sketch {
            engine,
            puck: Puck::new(),

            //ding: SoundFile::new(String::from("resources/sonidos/22.wav")),
            left: Paddle::new(true),
            right: Paddle::new(false),
            left_score: 0,
            right_score: 0,
        }
    }

    // Función setup() de javascript
    pub fn setup(&mut self, _rl: &mut RaylibHandle, _th: &RaylibThread) {
        //println!("En setup");
        //createcanvas(&mut self.engine, ANCHO, ALTO);

        //        self.puck = Puck::new();
        //        self.left = Paddle::new(true, 0.0);
        //        self.right = Paddle::new(false, ANCHO as f32);
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) -> bool {
        //println!("En update");
        if !self.engine.update(rl, th) {
            return false;
        }
        true
    }

    // Función draw() de javascript
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        background(&mut self.engine, d, 0);

        //self.puck.check_paddle(left);
        self.puck.check_paddle_right(self.right);
        self.puck.check_paddle_left(self.left);

        self.left.show(&mut self.engine, d);
        self.right.show(&mut self.engine, d);
        self.left.update();
        self.right.update();

        self.puck.update();
        self.puck.edges(&mut self.left_score, &mut self.right_score);
        self.puck.show(&mut self.engine, d);

        fill1(255.0, &mut self.engine.param);
        text_size(&mut self.engine.param, 32);
        text(
            &mut self.engine.param,
            d,
            &self.left_score.to_string(),
            32.0,
            40.0,
        );
        text(
            &mut self.engine.param,
            d,
            &self.right_score.to_string(),
            (ANCHO - 64) as f32,
            40.0,
        );
    }

    pub fn key_pressed(&mut self) {
        if self.engine.param.key == CodigosTecla::A {
            self.left.mover(-10.0);
        } else if self.engine.param.key == CodigosTecla::Z {
            self.left.mover(10.0)
        } else if self.engine.param.key == CodigosTecla::J {
            self.right.mover(-10.0)
        } else if self.engine.param.key == CodigosTecla::M {
            self.right.mover(10.0)
        }
    }

    pub fn key_released(&mut self) {
        if self.engine.param.keyr == CodigosTecla::A {
            self.left.mover(0.0);
        } else if self.engine.param.keyr == CodigosTecla::Z {
            self.left.mover(0.0)
        } else if self.engine.param.keyr == CodigosTecla::J {
            self.right.mover(0.0)
        } else if self.engine.param.keyr == CodigosTecla::M {
            self.right.mover(0.0)
        }
    }
}
