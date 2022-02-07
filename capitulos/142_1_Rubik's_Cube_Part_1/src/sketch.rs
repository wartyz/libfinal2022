use crate::cubie::Cubie;
use libfinal::engine::Engine;
use libfinal::p5::color_p5::Color;
use libfinal::p5::color_p5::{
    background, background3, fill, fill3, no_fill, no_stroke, stroke1, stroke2,
};
use libfinal::p5::vector3d_p5::Vector4;
use libfinal::p5::vector_p5::Vector3;
use libfinal::shapes::{box_shape, circle};
use libfinal::transform3d::{rotate_x3d, rotate_y3d, translate3d};
use raylib::prelude::*;

// Constantes del sketch

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 600;
pub const ALTO: i32 = 600;

// Aqui vendrá el pseudocódigo javascript

//                                      UP,     DOWN,    RIGHT,     LEFT,    FRONT,    BACK
//static COLORS: &'static [u32] = &[0xFFFFFF, 0xFFFF00, 0xFF0000, 0xFFA500, 0x00FF00, 0x0000FF];
pub const UP: Color = Color {
    r: 0xff,
    g: 0xff,
    b: 0xff,
    a: 0xff,
};
pub const DOWN: Color = Color {
    r: 0xff,
    g: 0xff,
    b: 0x00,
    a: 0xff,
};
pub const RIGHT: Color = Color {
    r: 0xff,
    g: 0xa5,
    b: 0x00,
    a: 0xff,
};
pub const LEFT: Color = Color {
    r: 0xff,
    g: 0x00,
    b: 0x00,
    a: 0xff,
};
pub const FRONT: Color = Color {
    r: 0x00,
    g: 0xff,
    b: 0x00,
    a: 0xff,
};
pub const BACK: Color = Color {
    r: 0x00,
    g: 0x00,
    b: 0xff,
    a: 0xff,
};

pub struct Sketch {
    pub engine: Engine,
    // Variables globales del scketch
    len: usize,
    dim: usize,
    cubie: Vec<Vec<Vec<Cubie>>>,
}

impl Sketch {
    pub fn new(rl: &mut RaylibHandle, th: &RaylibThread) -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);
        let dim = 3;

        let box3d = Cubie::default();
        let mut cube = vec![vec![vec![box3d; 3]; 3]; 3];
        let len = 50;
        for i in 0..dim {
            for j in 0..dim {
                for k in 0..dim {
                    let x = len * i;
                    let y = len * j;
                    let z = len * k;
                    cube[i][j][k] = Cubie::new(x as f32, y as f32, z as f32, len as f32);
                }
            }
        }
        Sketch {
            engine,
            len,
            dim,
            cubie: cube,
        }
    }

    // Función setup() de javascript
    pub fn setup(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {}

    pub fn update(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) -> bool {
        if !self.engine.update(rl, th) {
            return false;
        }
        true
    }

    // Función draw() de javascript
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        background(&mut self.engine, d, 51);
        rotate_y3d(0.01, &mut self.engine.param);
        rotate_x3d(0.01, &mut self.engine.param);
        for i in 0..self.dim {
            for j in 0..self.dim {
                for k in 0..self.dim {
                    let x = self.len * i;
                    let y = self.len * j;
                    let z = self.len * k;
                    self.cubie[i][j][k].show(d, &mut self.engine.param);
                }
            }
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
