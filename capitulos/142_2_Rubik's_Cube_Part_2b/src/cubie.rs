use crate::face::Face;
// use crate::sketch::{BACK, DOWN, FRONT, LEFT, RIGHT, UP};
use libfinal::events::mouse_is_pressed_inicio;
use libfinal::matem::random_range;
use libfinal::p5::color_p5::{fill, fill1, fill3, no_fill, stroke1, stroke3, Color};
use libfinal::p5::vector3d_p5::Vector4;
use libfinal::parametros::Parametros;
use libfinal::shapes::{box_shape, cubo3d, draw_rectangulo_3d, draw_triangulo_3d, stroke_weight};
use libfinal::transform::{pop_matrix3d, push_matrix3d, translate, Matrix3x3};
use libfinal::transform3d::{apply_matrix3d, identity4x4, scale1_3d, translate3d, Matrix4x4};
use raylib::prelude::RaylibDrawHandle;
use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub struct Cubie {
    pub matrix: Matrix4x4,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    c: Color,
    faces: Vec<Face>,
}

impl Default for Cubie {
    fn default() -> Self {
        Self {
            matrix: identity4x4(),
            x: 0,
            y: 0,
            z: 0,
            c: Color::WHITE,
            faces: vec![],
        }
    }
}

impl Cubie {
    pub fn new(m: Matrix4x4, x: i32, y: i32, z: i32) -> Cubie {
        let mut faces = vec![Face::default(); 6];

        faces[0] = Face::new(
            Vector4::new(0.0, 0.0, -1.0, 1.0),
            Color::new(0, 0, 255, 255),
        );
        faces[1] = Face::new(Vector4::new(0.0, 0.0, 1.0, 1.0), Color::new(0, 255, 0, 255));
        faces[2] = Face::new(
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Color::new(255, 255, 255, 255),
        );
        faces[3] = Face::new(
            Vector4::new(0.0, -1.0, 0.0, 1.0),
            Color::new(255, 255, 0, 255),
        );
        faces[4] = Face::new(
            Vector4::new(1.0, 0.0, 0.0, 1.0),
            Color::new(255, 150, 0, 255),
        );
        faces[5] = Face::new(
            Vector4::new(-1.0, 0.0, 0.0, 1.0),
            Color::new(255, 0, 0, 255),
        );

        let c = Color::WHITE;
        Cubie {
            matrix: m,
            x,
            y,
            z,
            c,
            faces,
        }
    }

    pub fn turn_facesz(&mut self, dir: i32) {
        for f in &mut self.faces {
            f.turn_z(dir as f32 * PI / 2.0);
        }
    }

    pub fn turn_facesy(&mut self, dir: i32) {
        for f in &mut self.faces {
            f.turn_y(dir as f32 * PI / 2.0);
        }
    }

    pub fn turn_facesx(&mut self, dir: i32) {
        for f in &mut self.faces {
            f.turn_x(dir as f32 * PI / 2.0);
        }
    }

    pub fn update(&mut self, x: i32, y: i32, z: i32, param: &mut Parametros) {
        self.matrix = identity4x4();
        //dbg!(x, y, z);
        self.matrix.translate3d(x as f32, y as f32, z as f32);
        //apply_matrix3d(param, self.matrix);
        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub fn show(&self, d: &mut RaylibDrawHandle, param: &mut Parametros) {
        //fill1(255.0, param);
        no_fill(param);
        stroke1(0.0, param);
        stroke_weight(0.1, param);

        push_matrix3d(param);
        apply_matrix3d(param, self.matrix);

        //cubo3d(d, param, Vector4::new(0.0, 0.0, 0.0, 1.0), 1.0);
        for f in &self.faces {
            f.show(d, param);
        }

        pop_matrix3d(param);
    }
}
