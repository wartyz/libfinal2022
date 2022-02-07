use std::convert::identity;
use std::f32::consts::PI;

use raylib::prelude::*;

use crate::boxy::Boxy;
use libfinal::engine::Engine;
use libfinal::events::mouse_is_pressed_inicio;
use libfinal::matem::{random, random_i32, random_usize};
use libfinal::p5::color_p5::{
    background, background3, fill, fill1, fill3, no_fill, no_stroke, stroke1, stroke2,
};
use libfinal::p5::color_p5::{stroke3, Color};
use libfinal::p5::vector3d_p5::Vector4;
use libfinal::p5::vector_p5::Vector3;
use libfinal::parametros::CodigosTecla;
use libfinal::parametros::CodigosTecla::*;
use libfinal::shapes::{
    box_shape, circle, cuadrado_2d_en_3d, cubo3d, draw_linea_3d, draw_rectangulo_3d,
};
use libfinal::structure::{pop, pop3d, push, push3d};
use libfinal::transform::translate;
use libfinal::transform3d::{
    apply_matrix3d, identity4x4, imprime_matriz_4x4, pop_matrix3d, push_matrix3d, rotate_x3d,
    rotate_y3d, rotate_z3d, scale1_3d, translate3d, Matrix4x4,
};
use libfinal::utiles::*;

// Constantes del sketch

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 400;
pub const ALTO: i32 = 400;

// Aqui vendrá el pseudocódigo javascript

pub struct Sketch {
    pub engine: Engine,
    angulo_camara: f32,
    distancia_camara: f32,
    // Variables globales del scketch
    a: f32,

    sponge: Vec<Boxy>,
}

impl Sketch {
    pub fn new(rl: &mut RaylibHandle, th: &RaylibThread) -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);
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
            angulo_camara: 0.0,
            distancia_camara,
            a: 0.0,

            sponge: vec![Boxy::new(0.0, 0.0, 0.0, 200.0)],
        }
    }

    // Función setup() de javascript
    pub fn setup(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {
        self.distancia_camara = 420.0;
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

        let p = &mut self.engine.param;

        camara_fija(p);
        dibuja_ejes(d, p, 120.0);
        stroke1(255.0, p);
        no_fill(p);
        //lights();

        translate(ANCHO as f32 / 2.0, ALTO as f32 / 2.0, p);
        rotate_x3d(self.a, p);
        rotate_y3d(self.a * 0.4, p);
        rotate_z3d(self.a * 0.1, p);

        //cubo3d(d, p, Vector4::default(), 200.0);
        for boxy in &mut self.sponge {
            boxy.show(d, p);
        }

        self.a += 0.01;

        //fill1(255.0, &mut self.engine.param);
    }

    pub fn mouse_pressed(&mut self) {
        dbg!("mouse pressed");
        let mut next: Vec<Boxy> = vec![];

        for b in self.sponge.iter() {
            let mut new_boxes = b.generate();
            next.append(&mut new_boxes);
        }

        //let mut next = &self.sponge.get(0).unwrap();

        self.sponge = next;
    }

    pub fn key_pressed(&mut self) {
        // if self.engine.param.key == CodigosTecla::SPACE {
        //     println!("En key pressed");
        //     self.started = true;
        //     //self.mover.start();
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
