use libfinal::p5::color_p5::{fill1, fill2, no_stroke};
use libfinal::p5::vector3d_p5::Vector4;
use libfinal::parametros::Parametros;
use libfinal::shapes::cubo3d;
use libfinal::transform::translate;
use libfinal::transform3d::{pop_matrix3d, push_matrix3d, translate3d};
use raylib::prelude::RaylibDrawHandle;

pub struct Boxy {
    pos: Vector4,
    r: f32,
}

impl Boxy {
    pub fn new(x: f32, y: f32, z: f32, r: f32) -> Self {
        let pos = Vector4::new(x, y, z, 1.0);
        Self { pos, r }
    }

    pub fn generate(&self) -> Vec<Boxy> {
        let mut boxes: Vec<Boxy> = vec![];
        for x in -1..2_i8 {
            for y in -1..2_i8 {
                for z in -1..2_i8 {
                    let sum = x.abs() + y.abs() + z.abs();

                    let new_r = self.r / 3.0;
                    if sum > 1 {
                        let b = Boxy::new(
                            self.pos.x + x as f32 * new_r,
                            self.pos.y + y as f32 * new_r,
                            self.pos.z + z as f32 * new_r,
                            new_r,
                        );
                        boxes.push(b);
                    }
                }
            }
        }
        boxes
    }

    pub fn show(&mut self, d: &mut RaylibDrawHandle, param: &mut Parametros) {
        push_matrix3d(param);
        translate3d(self.pos.x, self.pos.y, self.pos.z, param);
        no_stroke(param);
        fill2(255.0, 250.0, param);
        cubo3d(d, param, Vector4::default(), self.r);
        pop_matrix3d(param);
    }
}
