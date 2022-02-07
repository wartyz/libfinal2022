use crate::sketch::{BACK, DOWN, FRONT, LEFT, RIGHT, UP};
use libfinal::matem::random_range;
use libfinal::p5::color_p5::{fill, fill1, stroke1, stroke3, Color};
use libfinal::p5::vector3d_p5::Vector4;
use libfinal::parametros::Parametros;
use libfinal::shapes::{box_shape, draw_rectangulo_3d, draw_triangulo_3d, stroke_weight};
use libfinal::transform::{pop_matrix3d, push_matrix3d, translate};
use libfinal::transform3d::translate3d;
use raylib::prelude::RaylibDrawHandle;

#[derive(Debug, Clone, Copy)]
pub struct Cubie {
    pos: Vector4,
    len: f32,
}

impl Default for Cubie {
    fn default() -> Self {
        Self {
            pos: Vector4::new(0.0, 0.0, 0.0, 0.0),
            len: 0.0,
        }
    }
}

impl Cubie {
    pub fn new(x: f32, y: f32, z: f32, len: f32) -> Cubie {
        let pos = Vector4::new(x, y, z, 0.0);
        Cubie { pos, len }
    }

    pub fn show(&self, d: &mut RaylibDrawHandle, param: &mut Parametros) {
        //fill1(255.0, param);
        //stroke1(0.0, param);
        stroke_weight(8.0, param);
        stroke3(0.0, 0.0, 0.0, param);
        push_matrix3d(param);
        translate3d(self.pos.x, self.pos.y, self.pos.z, param);
        // let posicion = self.pos;
        // let ancho = self.len;
        // let alto = self.len;
        // let profundidad = self.len;
        //fill1(random_range(0.0, 255.0), param);
        //box_shape(d, param, posicion, ancho, alto, profundidad);

        // para cuadrado1 en tres dimensiones --------- z-fixed ---------------
        fill(BACK, param);
        let r = self.len / 2.0;

        let v0 = Vector4::new(-r, -r, -r, 1.0);
        let v1 = Vector4::new(r, -r, -r, 1.0);
        let v2 = Vector4::new(r, r, -r, 1.0);
        let v3 = Vector4::new(-r, r, -r, 1.0);

        let c = Color::new(0, 255, 0, 255);
        //draw_triangulo_3d(d, param, v0, v1, v2, c);

        draw_rectangulo_3d(d, param, v0, v1, v2, v3);

        // fin cuadrado1 en tres dimensiones -----------------------

        // para cuadrado2 en tres dimensiones ------------------------
        fill(FRONT, param);
        let v0 = Vector4::new(-r, -r, r, 1.0);
        let v1 = Vector4::new(r, -r, r, 1.0);
        let v2 = Vector4::new(r, r, r, 1.0);
        let v3 = Vector4::new(-r, r, r, 1.0);

        //let c = Color::new(255, 255, 0, 255);
        //draw_triangulo_3d(d, param, v0, v1, v2, c);
        draw_rectangulo_3d(d, param, v0, v1, v2, v3);

        // fin cuadrado2 en tres dimensiones -----------------------

        // para cuadrado3 en tres dimensiones ------------ y-fixed ------------
        fill(DOWN, param);
        let v0 = Vector4::new(-r, -r, -r, 1.0);
        let v1 = Vector4::new(r, -r, -r, 1.0);
        let v2 = Vector4::new(r, -r, r, 1.0);
        let v3 = Vector4::new(-r, -r, r, 1.0);

        //let c = Color::new(255, 255, 0, 255);
        //draw_triangulo_3d(d, param, v0, v1, v2, c);
        draw_rectangulo_3d(d, param, v0, v1, v2, v3);

        // fin cuadrado3 en tres dimensiones -----------------------

        // para cuadrado4 en tres dimensiones ------------------------
        fill(UP, param);
        let v0 = Vector4::new(-r, r, -r, 1.0);
        let v1 = Vector4::new(r, r, -r, 1.0);
        let v2 = Vector4::new(r, r, r, 1.0);
        let v3 = Vector4::new(-r, r, r, 1.0);

        //let c = Color::new(255, 255, 0, 255);
        //draw_triangulo_3d(d, param, v0, v1, v2, c);
        draw_rectangulo_3d(d, param, v0, v1, v2, v3);

        // fin cuadrado4 en tres dimensiones -----------------------

        // para cuadrado3 en tres dimensiones ------------ x-fixed ------------
        fill(LEFT, param);
        let v0 = Vector4::new(-r, -r, -r, 1.0);
        let v1 = Vector4::new(-r, r, -r, 1.0);
        let v2 = Vector4::new(-r, r, r, 1.0);
        let v3 = Vector4::new(-r, -r, r, 1.0);

        //let c = Color::new(255, 255, 0, 255);
        //draw_triangulo_3d(d, param, v0, v1, v2, c);
        draw_rectangulo_3d(d, param, v0, v1, v2, v3);

        // fin cuadrado3 en tres dimensiones -----------------------

        // para cuadrado4 en tres dimensiones ------------------------
        fill(RIGHT, param);
        let v0 = Vector4::new(r, -r, -r, 1.0);
        let v1 = Vector4::new(r, r, -r, 1.0);
        let v2 = Vector4::new(r, r, r, 1.0);
        let v3 = Vector4::new(r, -r, r, 1.0);

        //let c = Color::new(255, 255, 0, 255);
        //draw_triangulo_3d(d, param, v0, v1, v2, c);
        draw_rectangulo_3d(d, param, v0, v1, v2, v3);

        // fin cuadrado4 en tres dimensiones -----------------------

        pop_matrix3d(param);
    }
}
