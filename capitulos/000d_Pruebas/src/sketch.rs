use libfinal::engine::Engine;
use libfinal::p5::color_p5::{
    background, background3, fill, fill3, no_fill, no_stroke, stroke1, stroke2,
};
use libfinal::p5::color_p5::{stroke3, Color};
use libfinal::p5::vector3d_p5::Vector4;
use libfinal::p5::vector_p5::Vector3;
use libfinal::parametros::CodigosTecla::*;
use libfinal::shapes::{
    box_shape, circle, cuadrado_2d_en_3d, draw_linea_3d, draw_rectangulo_3d, line, point, point3d,
    quad, stroke_weight,
};
use libfinal::transform3d::{
    apply_matrix3d, identity4x4, imprime_matriz_4x4, pop_matrix3d, push_matrix3d, rotate_x3d,
    rotate_y3d, rotate_z3d, scale1_3d, translate3d, Matrix4x4,
};

use libfinal::events::mouse_is_pressed_inicio;
use libfinal::matem::{random, random_i32, random_usize};
use libfinal::parametros::CodigosTecla;

use libfinal::structure::{pop, pop3d, push, push3d};
use libfinal::transform::{rotate_z, translate};
use raylib::prelude::*;
use std::convert::identity;
use std::f32::consts::PI;

// Constantes del sketch

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 800;
pub const ALTO: i32 = 800;

// Aqui vendrá el pseudocódigo javascript

pub struct Sketch {
    pub engine: Engine,
    // Variables globales del scketch
}

impl Sketch {
    pub fn new(rl: &mut RaylibHandle, th: &RaylibThread) -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);

        Sketch { engine }
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
        background(&mut self.engine, d, 220);

        stroke_weight(1.0, &mut self.engine.param);
        stroke3(0.0, 0.0, 0.0, &mut self.engine.param);
        for i in (0..800).step_by(20) {
            line(d, &mut self.engine.param, i as f32, 0.0, i as f32, 800.0);
        }
        for j in (0..800).step_by(20) {
            line(d, &mut self.engine.param, 0.0, j as f32, 800.0, j as f32);
        }

        stroke_weight(1.0, &mut self.engine.param);

        // ------------------------------------------------
        //no_stroke(&mut self.engine.param);
        no_fill(&mut self.engine.param);
        quad(
            d,
            &mut self.engine.param,
            38.0,
            31.0,
            86.0,
            20.0,
            69.0,
            63.0,
            30.0,
            76.0,
        );
    }

    // fn rotate_z_global(&mut self, x: i32, y: i32, _z: i32) {
    //     //self.dibuja_ejes(d);
    //     let mut t1 = identity4x4();
    //     t1.translate3d(x as f32, y as f32, 0.0);
    //
    //     let inv = t1.inverse3d();
    //
    //     let mut rot = identity4x4();
    //     rot.rotate_z3d(self.current_move.angle);
    //
    //     let resultado = inv * rot * t1;
    //
    //     apply_matrix3d(&mut self.engine.param, resultado);
    // }
    //
    // fn rotate_x_global(&mut self, _x: i32, y: i32, z: i32) {
    //     //self.dibuja_ejes(d);
    //     let mut t1 = identity4x4();
    //     t1.translate3d(0.0, y as f32, z as f32);
    //
    //     let inv = t1.inverse3d();
    //
    //     let mut rot = identity4x4();
    //     rot.rotate_x3d(self.current_move.angle);
    //
    //     let resultado = inv * rot * t1;
    //
    //     apply_matrix3d(&mut self.engine.param, resultado);
    // }
    //
    // fn rotate_y_global(&mut self, x: i32, _y: i32, z: i32) {
    //     //self.dibuja_ejes(d);
    //     let mut t1 = identity4x4();
    //     t1.translate3d(x as f32, 0.0, z as f32);
    //
    //     let inv = t1.inverse3d();
    //
    //     let mut rot = identity4x4();
    //     rot.rotate_y3d(-self.current_move.angle);
    //
    //     let resultado = inv * rot * t1;
    //
    //     apply_matrix3d(&mut self.engine.param, resultado);
    // }

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

    // fn mueve_camara_circular_eje_y(&mut self) {
    //     self.angulo_camara += 0.005;
    //     self.engine.param.camara.posx = self.distancia_camara * self.angulo_camara.cos();
    //     self.engine.param.camara.posz = self.distancia_camara * self.angulo_camara.sin();
    // }

    // fn mueve_camara_circular_eje_x(&mut self) {
    //     let r = 300.0;
    //     let angulo = self.r;
    //     self.r += 0.005;
    //     self.engine.param.camara.posy = r * angulo.cos();
    //     self.engine.param.camara.posz = r * angulo.sin();
    // }

    pub fn key_pressed(&mut self) {
        // if self.engine.param.key == CodigosTecla::SPACE {
        //     //println!("En key pressed");
        //     //self.started = true;
        //     self.current_move.start();
        // }

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
