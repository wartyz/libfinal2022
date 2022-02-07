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
use libfinal::matem::{random, random_i32, random_usize};
use libfinal::parametros::CodigosTecla;
use raylib::prelude::*;
use std::f32::consts::PI;

// Constantes del sketch

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 600;
pub const ALTO: i32 = 600;

// Aqui vendrá el pseudocódigo javascript

pub struct Sketch {
    pub engine: Engine,
    // Variables globales del scketch
    dim: usize,
    cube: Vec<Cubie>,
    angulo_camara: f32,
    started: bool,
    counter: i32,
    all_moves: Vec<String>,
    sequence: String,
    distancia_camara: f32,
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
                    //matrix.scale1_3d(50.0); // lo pongo aqui
                    matrix.translate3d(x as f32, y as f32, z as f32);

                    //dbg!(matrix);
                    cube[index] = Cubie::new(matrix, x, y, z);
                    index += 1;
                }
            }
        }

        let mut sequence = "f".to_string();
        let all_moves = vec![
            "f".to_string(),
            "b".to_string(),
            "u".to_string(),
            "d".to_string(),
            "l".to_string(),
            "r".to_string(),
        ];

        for _ in 0..100 {
            // Numero de movimientos para "aleatorizar" el cubo
            let r = random_usize(all_moves.len());
            if random(1.0) < 0.5 {
                sequence += &all_moves[r];
            } else {
                sequence += &all_moves[r].to_uppercase();
            }
        }

        for i in sequence.len() - 1..=0 {
            // let mover = self.sequence.chars().nth(self.counter as usize).unwrap();
            let next_move = Self::flip_case(sequence.chars().nth(i).unwrap().to_string());
            sequence += &next_move;
        }
        let distancia_camara = Vector4::dist_s(
            &Vector4::default(),
            &Vector4::new(
                engine.param.camara.posx,
                engine.param.camara.posy,
                engine.param.camara.posz,
                1.0,
            ),
        );
        Sketch {
            engine,
            dim,
            cube,
            angulo_camara: 0.0,
            started: false,
            counter: 0,
            all_moves,
            sequence,
            distancia_camara,
        }
    }

    fn flip_case(c: String) -> String {
        let s = "".to_string() + &c;

        if s == s.to_lowercase() {
            return s.to_uppercase();
        } else {
            return s.to_lowercase();
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

    fn turn_z(&mut self, index: i32, dir: i32) {
        for i in 0..self.cube.len() {
            //let mut qb = self.cube[i].clone();

            //println!("self.cube[i].z");
            if self.cube[i].z == index {
                let mut matrix = identity4x4();
                matrix.rotate_z3d(dir as f32 * PI / 2.0);
                matrix.translate3d(self.cube[i].x as f32, self.cube[i].y as f32, 0.0);
                //imprime_matriz_4x4(matrix);
                //self.cube[i].matrix = self.cube[i].matrix * matrix; // ----> lo pongo yo
                let zz = self.cube[i].z;
                self.cube[i].update(
                    matrix.data[3].round() as i32,
                    matrix.data[7].round() as i32,
                    zz,
                    &mut self.engine.param,
                );
                self.cube[i].turn_facesz(dir);
            }
        }
    }

    fn turn_y(&mut self, index: i32, dir: i32) {
        for i in 0..self.cube.len() {
            //let mut qb = self.cube[i].clone();

            //println!("self.cube[i].z");
            if self.cube[i].y == index {
                let mut matrix = identity4x4();
                matrix.rotate_z3d(dir as f32 * PI / 2.0);
                matrix.translate3d(self.cube[i].x as f32, self.cube[i].z as f32, 0.0);
                //imprime_matriz_4x4(matrix);
                self.cube[i].matrix = self.cube[i].matrix * matrix; // ----> lo pongo yo
                let yy = self.cube[i].y;
                self.cube[i].update(
                    matrix.data[3].round() as i32,
                    yy,
                    matrix.data[7].round() as i32,
                    &mut self.engine.param,
                );
                self.cube[i].turn_facesy(dir);
            }
        }
    }

    fn turn_x(&mut self, index: i32, dir: i32) {
        for i in 0..self.cube.len() {
            //let mut qb = self.cube[i].clone();

            //println!("self.cube[i].z");
            if self.cube[i].x == index {
                let mut matrix = identity4x4();
                matrix.rotate_z3d(dir as f32 * PI / 2.0);
                matrix.translate3d(self.cube[i].y as f32, self.cube[i].z as f32, 0.0);
                //imprime_matriz_4x4(matrix);
                self.cube[i].matrix = self.cube[i].matrix * matrix; // ----> lo pongo yo
                let xx = self.cube[i].x;
                self.cube[i].update(
                    xx,
                    matrix.data[3].round() as i32,
                    matrix.data[7].round() as i32,
                    &mut self.engine.param,
                );
                self.cube[i].turn_facesx(dir);
            }
        }
    }
    // Función draw() de javascript
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        background(&mut self.engine, d, 51);

        self.mueve_camara_circular_eje_y();
        //self.mueve_camara_circular_eje_x();
        if self.started {
            if self.engine.param.framecount % 1 == 0 {
                if self.counter < self.sequence.len() as i32 {
                    let mover = self.sequence.chars().nth(self.counter as usize).unwrap();

                    //let mover = self.sequence.charAt(self.counter);
                    self.apply_move(mover);
                    self.counter += 1;
                }
            }
        }

        //scale1_3d(50.0, &mut self.engine.param);
        for i in 0..self.cube.len() {
            self.cube[i].show(d, &mut self.engine.param);
        }
    }

    fn mueve_camara_circular_eje_y(&mut self) {
        self.angulo_camara += 0.005;
        self.engine.param.camara.posx = self.distancia_camara * self.angulo_camara.cos();
        self.engine.param.camara.posz = self.distancia_camara * self.angulo_camara.sin();
    }

    // fn mueve_camara_circular_eje_x(&mut self) {
    //     let r = 300.0;
    //     let angulo = self.r;
    //     self.r += 0.005;
    //     self.engine.param.camara.posy = r * angulo.cos();
    //     self.engine.param.camara.posz = r * angulo.sin();
    // }

    pub fn apply_move(&mut self, mover: char) {
        println!("mover = {}", mover);
        match mover {
            'f' => self.turn_z(1, 1),
            'F' => self.turn_z(1, -1),
            'b' => self.turn_z(-1, 1),
            'B' => self.turn_z(-1, -1),
            'u' => self.turn_y(1, 1),
            'U' => self.turn_y(1, -1),
            'd' => self.turn_y(-1, 1),
            'D' => self.turn_y(-1, -1),
            'l' => self.turn_x(-1, 1),
            'L' => self.turn_x(-1, -1),
            'r' => self.turn_x(1, 1),
            'R' => self.turn_x(1, -1),
            _ => {}
        }
    }

    pub fn key_pressed(&mut self) {
        if self.engine.param.key == CodigosTecla::SPACE {
            //println!("En key pressed");
            self.started = true;
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
