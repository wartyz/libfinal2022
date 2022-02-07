use libfinal::matem::round;
use libfinal::p5::color_p5::{fill, fill1, no_stroke, Color};
use libfinal::p5::vector3d_p5::Vector4;
use libfinal::parametros::Parametros;
use libfinal::shapes::cuadrado_2d_en_3d;
use libfinal::transform::translate;
use libfinal::transform3d::{pop_matrix3d, push_matrix3d, rotate_x3d, rotate_y3d, translate3d};
use raylib::prelude::RaylibDrawHandle;
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Face {
    normal: Vector4,
    c: Color,
}

impl Default for Face {
    fn default() -> Self {
        Self {
            normal: Vector4::new(0.0, 0.0, 0.0, 1.0),
            c: Color::new(0, 0, 0, 255),
        }
    }
}

impl Face {
    pub fn new(normal: Vector4, c: Color) -> Self {
        Self { normal, c }
    }

    pub fn turn_z(&mut self, angle: f32) {
        let mut v2 = Vector4::default();

        v2.x = round(self.normal.x * angle.cos() - self.normal.y * angle.sin());
        v2.y = round(self.normal.x * angle.sin() + self.normal.y * angle.cos());
        v2.z = round(self.normal.z);
        self.normal = v2;
    }

    pub fn turn_y(&mut self, angle: f32) {
        let mut v2 = Vector4::default();

        v2.x = round(self.normal.x * angle.cos() - self.normal.z * angle.sin());
        v2.z = round(self.normal.x * angle.sin() + self.normal.z * angle.cos());
        v2.y = round(self.normal.y);
        self.normal = v2;
    }

    pub fn turn_x(&mut self, angle: f32) {
        let mut v2 = Vector4::default();

        v2.y = round(self.normal.y * angle.cos() - self.normal.z * angle.sin());
        v2.z = round(self.normal.y * angle.sin() + self.normal.z * angle.cos());
        v2.x = round(self.normal.x);
        self.normal = v2;
    }

    pub fn show(&self, d: &mut RaylibDrawHandle, param: &mut Parametros) {
        push_matrix3d(param);
        fill(self.c, param);
        //fill(Color::WHITE, param);
        no_stroke(param);
        //rectMode(CENTER);
        //dbg!(self.normal.x);
        // traslada caras puestas "en estrella" a las caras del cubo segun su normal
        translate3d(
            0.5 * self.normal.x,
            0.5 * self.normal.y,
            0.5 * self.normal.z,
            param,
        );
        if self.normal.x.abs() > 0.0 {
            rotate_y3d(PI / 2.0, param);
        } else if self.normal.y.abs() > 0.0 {
            rotate_x3d(PI / 2.0, param);
        }
        cuadrado_2d_en_3d(d, param, 0.0, 0.0, 1.0);
        pop_matrix3d(param);
    }
}
