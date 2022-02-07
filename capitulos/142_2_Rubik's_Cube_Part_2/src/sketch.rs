use crate::cubie::Cubie;
use libfinal::engine::Engine;
use libfinal::p5::color_p5::Color;
use libfinal::p5::color_p5::{
    background, background3, fill, fill3, no_fill, no_stroke, stroke1, stroke2,
};
use libfinal::p5::vector3d_p5::Vector4;
use libfinal::p5::vector_p5::Vector3;
use libfinal::parametros::CodigosTecla::*;
use libfinal::shapes::{box_shape, circle};
use libfinal::transform3d::{
    identity4x4, imprime_matriz_4x4, rotate_x3d, rotate_y3d, scale1_3d, translate3d, Matrix4x4,
};

use libfinal::events::mouse_is_pressed_inicio;
use raylib::prelude::*;
use std::f32::consts::PI;

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
    //len: usize,
    dim: usize,
    cube: Vec<Cubie>,
    r: f32,
}

impl Sketch {
    pub fn new(rl: &mut RaylibHandle, th: &RaylibThread) -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);
        let dim = 3;
        let mut index = 0;

        let cubie = Cubie::default();
        let mut cube = vec![cubie; dim * dim * dim];
        //let len = 50;
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    let mut matrix: Matrix4x4 = identity4x4();
                    matrix.scale1_3d(50.0);
                    matrix.translate3d(x as f32, y as f32, z as f32);

                    //dbg!(matrix);
                    cube[index] = Cubie::new(matrix, x, y, z);
                    index += 1;
                }
            }
        }
        Sketch {
            engine,
            //len,
            dim,
            cube,
            r: 0.0,
        }
    }

    // Función setup() de javascript
    pub fn setup(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {
        self.cube[0].highlight = true;
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) -> bool {
        if !self.engine.update(rl, th) {
            return false;
        }
        true
    }

    fn turnz(&mut self) {
        for i in 0..self.cube.len() {
            let mut qb = self.cube[i];

            //println!("self.cube[i].z");
            if self.cube[i].z == -1 {
                let mut matrix = identity4x4();
                matrix.rotate_z3d(PI / 2.0);
                matrix.translate3d(qb.x as f32, qb.y as f32, 0.0);
                //imprime_matriz_4x4(matrix);
                self.cube[i].matrix = self.cube[i].matrix * matrix;
                qb.update(
                    matrix.data[8].round() as i32,
                    matrix.data[12].round() as i32,
                    qb.z,
                    &mut self.engine.param,
                );
            }
        }
    }
    // Función draw() de javascript
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        background(&mut self.engine, d, 51);
        //rotate_y3d(0.01, &mut self.engine.param);

        self.mueve_camara_circular_eje_y();
        //self.mueve_camara_circular_eje_x();
        //rotate_y3d(self.r, &mut self.engine.param);
        //scale1_3d(50.0, &mut self.engine.param);
        for i in 0..self.cube.len() {
            //dbg!(self.cube[i]);

            if mouse_is_pressed_inicio(&mut self.engine.param) {
                self.turnz();
            }
            self.cube[i].show(d, &mut self.engine.param);
        }
    }
    fn mueve_camara_circular_eje_y(&mut self) {
        let r = 300.0;
        let angulo = self.r;
        self.r += 0.005;
        self.engine.param.camara.posx = r * angulo.cos();
        self.engine.param.camara.posz = r * angulo.sin();
    }

    fn mueve_camara_circular_eje_x(&mut self) {
        let r = 300.0;
        let angulo = self.r;
        self.r += 0.005;
        self.engine.param.camara.posy = r * angulo.cos();
        self.engine.param.camara.posz = r * angulo.sin();
    }
    pub fn key_pressed(&mut self) {
        if self.engine.param.key == N1 {
            println!("En key pressed");
            self.turnz();
        }

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
