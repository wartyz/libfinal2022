use crate::sketch::{BACK, DOWN, FRONT, LEFT, RIGHT, UP};
use libfinal::events::mouse_is_pressed_inicio;
use libfinal::matem::random_range;
use libfinal::p5::color_p5::{fill, fill1, fill3, stroke1, stroke3, Color};
use libfinal::p5::vector3d_p5::Vector4;
use libfinal::parametros::Parametros;
use libfinal::shapes::{box_shape, cubo3d, draw_rectangulo_3d, draw_triangulo_3d, stroke_weight};
use libfinal::transform::{pop_matrix3d, push_matrix3d, translate, Matrix3x3};
use libfinal::transform3d::{apply_matrix3d, identity4x4, scale1_3d, translate3d, Matrix4x4};
use raylib::prelude::RaylibDrawHandle;

#[derive(Debug, Clone, Copy)]
pub struct Cubie {
    pub(crate) matrix: Matrix4x4,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub highlight: bool,
}

impl Default for Cubie {
    fn default() -> Self {
        Self {
            matrix: identity4x4(),
            x: 0,
            y: 0,
            z: 0,
            highlight: false,
        }
    }
}

impl Cubie {
    pub fn new(m: Matrix4x4, x: i32, y: i32, z: i32) -> Cubie {
        Cubie {
            matrix: m,
            x,
            y,
            z,
            highlight: false,
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
        fill1(255.0, param);
        if self.highlight {
            fill3(255.0, 0.0, 0.0, param);
        }
        stroke1(0.0, param);
        stroke_weight(1.0, param);

        push_matrix3d(param);
        apply_matrix3d(param, self.matrix);
        // prueba no aplicar matrix sino la traslacion de la traslacion

        //dbg!(self.matrix);
        //box_shape(d, param, Vector4::new(0.0, 0.0, 0.0, 1.0), 1.0, 1.0, 1.0);

        cubo3d(d, param, Vector4::new(0.0, 0.0, 0.0, 1.0), 1.0);

        pop_matrix3d(param);
    }
}
