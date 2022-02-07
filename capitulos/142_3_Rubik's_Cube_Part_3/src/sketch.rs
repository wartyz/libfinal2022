use crate::cubie::Cubie;
use libfinal::engine::Engine;
use libfinal::p5::color_p5::{
    background, background3, fill, fill3, no_fill, no_stroke, stroke1, stroke2,
};
use libfinal::p5::color_p5::{stroke3, Color};
use libfinal::p5::vector3d_p5::Vector4;
use libfinal::p5::vector_p5::Vector3;
use libfinal::parametros::CodigosTecla::*;
use libfinal::shapes::{box_shape, circle, cuadrado_2d_en_3d, draw_linea_3d, draw_rectangulo_3d};
use libfinal::transform3d::{
    apply_matrix3d, identity4x4, imprime_matriz_4x4, pop_matrix3d, push_matrix3d, rotate_x3d,
    rotate_y3d, rotate_z3d, scale1_3d, translate3d, Matrix4x4,
};

use libfinal::events::mouse_is_pressed_inicio;
use libfinal::matem::{random, random_i32, random_usize};
use libfinal::parametros::CodigosTecla;

use crate::mover::Mover;
use crate::turns::turn_z;
use libfinal::structure::{pop, pop3d, push, push3d};
use raylib::prelude::*;
use std::convert::identity;
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
    counter: usize,
    all_moves: Vec<Mover>,
    sequence: Vec<Mover>,
    distancia_camara: f32,
    current_move: Mover,
}

impl Sketch {
    pub fn new(rl: &mut RaylibHandle, th: &RaylibThread) -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);
        let dim = 3;
        let mut index = 0;
        let counter = 0;

        let cubie = Cubie::default();
        let mut cube = vec![cubie; dim * dim * dim];
        //let len = 50;
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    let mut matrix: Matrix4x4 = identity4x4();
                    //matrix.scale1_3d(50.0); // lo pongo aqui
                    matrix.translate3d(x as f32, y as f32, z as f32);

                    // puntos en un cubo con centro en (0,0,0)
                    cube[index] = Cubie::new(matrix, x, y, z);
                    index += 1;
                }
            }
        }

        let mut sequence = vec![];
        let all_moves = vec![
            Mover::new(0, 1, 0, 1),
            Mover::new(0, 1, 0, -1),
            Mover::new(0, -1, 0, 1),
            Mover::new(0, -1, 0, -1),
            Mover::new(1, 0, 0, 1),
            Mover::new(1, 0, 0, -1),
            Mover::new(-1, 0, 0, 1),
            Mover::new(-1, 0, 0, -1),
            Mover::new(0, 0, 1, 1),
            Mover::new(0, 0, 1, -1),
            Mover::new(0, 0, -1, 1),
            Mover::new(0, 0, -1, -1),
        ];

        for _ in 0..30 {
            // Numero de movimientos para "aleatorizar" el cubo
            let r = random_usize(all_moves.len());
            let m = all_moves.get(r).unwrap(); // ¿¿¿ copy() en js
            sequence.push(*m);
        }
        //dbg!(&sequence);
        dbg!(sequence.len());
        for i in (0..sequence.len()).rev() {
            let mut next_move = sequence.get(i).unwrap().copia(); // -----¿¿¿copy()
            next_move.reverse();
            sequence.push(next_move);
        }
        // pba --------------------------------------------------------------------
        //sequence = vec![Mover::new(1, 1, -1, -1), Mover::new(1, 1, -1, 1)];
        dbg!(sequence.len());
        let current_move = sequence.get(counter).unwrap().clone();

        let distancia_camara = Vector4::dist_s(
            &Vector4::default(),
            &Vector4::new(
                engine.param.camara.posx,
                engine.param.camara.posy,
                engine.param.camara.posz,
                1.0,
            ),
        );
        //let mut mover = all_moves[3];

        Sketch {
            engine,
            dim,
            cube,
            angulo_camara: 0.0,
            started: false,
            counter,
            all_moves,
            sequence,
            distancia_camara,
            current_move,
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
        // rotate_x3d(-0.5, &mut self.engine.param);
        // rotate_y3d(0.4, &mut self.engine.param);
        // rotate_z3d(0.1, &mut self.engine.param);
        //if self.started {
        self.current_move
            .update(&mut self.cube, self.current_move, &mut self.engine.param);

        if self.current_move.finished() {
            if self.counter < self.sequence.len() - 1 {
                self.counter += 1;
                self.current_move = self.sequence.get(self.counter).unwrap().clone();
                self.current_move.start();
            }
        }
        //}

        self.mueve_camara_circular_eje_y();
        //
        //self.mueve_camara_circular_eje_x();

        // Si hago escala no se modifican lados---TODO: check
        //scale1_3d(50.0, &mut self.engine.param);
        for i in 0..self.cube.len() {
            push_matrix3d(&mut self.engine.param);
            self.dibuja_ejes(d);

            // Solo un grupo  de 9 cubis
            if (self.cube[i].z).abs() > 0 && self.cube[i].z == self.current_move.z {
                //rotate_z3d(self.mover.angle, &mut self.engine.param);

                self.rotate_z_global(self.cube[i].x, self.cube[i].y, self.cube[i].z);
            } else if (self.cube[i].x).abs() > 0 && self.cube[i].x == self.current_move.x {
                self.rotate_x_global(self.cube[i].x, self.cube[i].y, self.cube[i].z);
            } else if (self.cube[i].y).abs() > 0 && self.cube[i].y == self.current_move.y {
                self.rotate_y_global(self.cube[i].x, self.cube[i].y, self.cube[i].z);
            }

            self.cube[i].show(d, &mut self.engine.param);

            pop_matrix3d(&mut self.engine.param);
        }
    }

    fn rotate_z_global(&mut self, x: i32, y: i32, _z: i32) {
        //self.dibuja_ejes(d);
        let mut t1 = identity4x4();
        t1.translate3d(x as f32, y as f32, 0.0);

        let inv = t1.inverse3d();

        let mut rot = identity4x4();
        rot.rotate_z3d(self.current_move.angle);

        let resultado = inv * rot * t1;

        apply_matrix3d(&mut self.engine.param, resultado);
    }

    fn rotate_x_global(&mut self, _x: i32, y: i32, z: i32) {
        //self.dibuja_ejes(d);
        let mut t1 = identity4x4();
        t1.translate3d(0.0, y as f32, z as f32);

        let inv = t1.inverse3d();

        let mut rot = identity4x4();
        rot.rotate_x3d(self.current_move.angle);

        let resultado = inv * rot * t1;

        apply_matrix3d(&mut self.engine.param, resultado);
    }

    fn rotate_y_global(&mut self, x: i32, _y: i32, z: i32) {
        //self.dibuja_ejes(d);
        let mut t1 = identity4x4();
        t1.translate3d(x as f32, 0.0, z as f32);

        let inv = t1.inverse3d();

        let mut rot = identity4x4();
        rot.rotate_y3d(-self.current_move.angle);

        let resultado = inv * rot * t1;

        apply_matrix3d(&mut self.engine.param, resultado);
    }

    fn dibuja_ejes(&mut self, d: &mut RaylibDrawHandle) {
        let p = &mut self.engine.param;
        // Eje x ----------------- ROJO -------------------------------
        let va = Vector4::new(-20.0, 0.0, 0.0, 1.0);
        let vb = Vector4::new(20.0, 0.0, 0.0, 1.0);
        stroke3(255.0, 0.0, 0.0, p);
        draw_linea_3d(d, p, va, vb);

        // Cuadrado eje X positivo ------- NARANJA --------------------
        fill3(255.0, 150.0, 0.0, p);
        let var = Vector4::new(3.0, -0.5, 0.5, 1.0);
        let vbr = Vector4::new(3.0, -0.5, -0.5, 1.0);
        let vcr = Vector4::new(3.0, 0.5, -0.5, 1.0);
        let vdr = Vector4::new(3.0, 0.5, 0.5, 1.0);
        draw_rectangulo_3d(d, p, var, vbr, vcr, vdr);

        // Eje y ---------------------- AZUL --------------------------
        let va = Vector4::new(0.0, -20.0, 0.0, 1.0);
        let vb = Vector4::new(0.0, 20.0, 0.0, 1.0);
        stroke3(0.0, 0.0, 255.0, p);
        draw_linea_3d(d, p, va, vb);

        // Cuadrado eje y positivo -------- BLANCO --------------------
        fill3(255.0, 255.0, 255.0, p);
        let var = Vector4::new(-0.5, 3.0, 0.5, 1.0);
        let vbr = Vector4::new(0.5, 3.0, 0.5, 1.0);
        let vcr = Vector4::new(0.5, 3.0, -0.5, 1.0);
        let vdr = Vector4::new(-0.5, 3.0, -0.5, 1.0);
        draw_rectangulo_3d(d, p, var, vbr, vcr, vdr);

        // Eje z -------------------- ALLO -----------------------------
        let va = Vector4::new(0.0, 0.0, -20.0, 1.0);
        let vb = Vector4::new(0.0, 0.0, 20.0, 1.0);
        stroke3(255.0, 255.0, 0.0, p);
        draw_linea_3d(d, p, va, vb);

        // Cuadrado eje z positivo ----------- VERDE -------------------
        fill3(0.0, 255.0, 0.0, p);
        let var = Vector4::new(-0.5, -0.5, 3.0, 1.0);
        let vbr = Vector4::new(0.5, -0.5, 3.0, 1.0);
        let vcr = Vector4::new(0.5, 0.5, 3.0, 1.0);
        let vdr = Vector4::new(-0.5, 0.5, 3.0, 1.0);
        draw_rectangulo_3d(d, p, var, vbr, vcr, vdr);
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
            'f' => turn_z(1, 1, &mut self.cube, &mut self.engine.param),
            'F' => turn_z(1, -1, &mut self.cube, &mut self.engine.param),
            'b' => turn_z(-1, 1, &mut self.cube, &mut self.engine.param),
            'B' => turn_z(-1, -1, &mut self.cube, &mut self.engine.param),
            // 'u' => turn_y(1, 1),
            // 'U' => turn_y(1, -1),
            // 'd' => turn_y(-1, 1),
            // 'D' => turn_y(-1, -1),
            // 'l' => turn_x(-1, 1),
            // 'L' => turn_x(-1, -1),
            // 'r' => turn_x(1, 1),
            // 'R' => turn_x(1, -1),
            _ => {}
        }
    }

    pub fn key_pressed(&mut self) {
        if self.engine.param.key == CodigosTecla::SPACE {
            //println!("En key pressed");
            //self.started = true;
            self.current_move.start();
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
