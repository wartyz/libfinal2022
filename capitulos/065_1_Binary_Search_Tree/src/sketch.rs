use raylib::prelude::*;

use libfinal::ambient::draw_fps_fi;
use libfinal::constantes::*;
use libfinal::engine::Engine;
use libfinal::matem::*;
use libfinal::p5::color_p5::*;
use libfinal::p5::image_p5::P5Image;
use libfinal::p5::vector_p5::*;
use libfinal::parametros::*;
use libfinal::shapes::*;
use libfinal::typography::{text, text_size};

use crate::node::Node;
use crate::tree::Tree;

// Constantes del sketch


//pub const N: i32 = 128;
//pub const SCALE: i32 = 4;

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 400;
pub const ALTO: i32 = 300;

// Aqui vendrá el pseudocódigo javascript

pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    tree: Tree,

}

impl Sketch {
    pub fn new(rl: &mut RaylibHandle, th: &RaylibThread) -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);


        Sketch {
            engine,
            tree: Tree::new(),

        }
    }

    // Función setup() de javascript
    pub fn setup(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {
        // self.tree.add_node(Node::new(5.0));
        self.tree.add_value(5.0);

        self.tree.add_value(3.0);

        dbg!(&self.tree);
    }

    pub fn add_value(&mut self, v: f32) {}

    pub fn update(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) -> bool {
        if !self.engine.update(rl, th) {
            return false;
        }
        true
    }


    // Función draw() de javascript
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        {
//            let mut d = rl.begin_drawing(&th);
//            background(&mut self.engine, &mut d, 245); //RAYWHITE
        }
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