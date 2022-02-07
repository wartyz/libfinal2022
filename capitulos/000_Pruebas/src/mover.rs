use crate::cubie::Cubie;
use crate::turns::*;
use libfinal::parametros::Parametros;
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Mover {
    pub angle: f32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub(crate) dir: i32,
    animating: bool,
    finished: bool,
    speed: f32,
}

impl Mover {
    pub fn new(x: i32, y: i32, z: i32, dir: i32) -> Self {
        Self {
            angle: 0.0,
            x,
            y,
            z,
            dir,
            animating: false,
            finished: false,
            speed: 0.1,
        }
    }

    pub fn copia(&self) -> Mover {
        Mover::new(self.x, self.y, self.z, self.dir)
    }

    pub fn reverse(&mut self) {
        self.dir *= -1;
    }

    pub fn start(&mut self) {
        self.animating = true;
        self.finished = false;
        self.angle = 0.0;
    }

    pub fn finished(&self) -> bool {
        self.finished
    }

    pub fn update(&mut self, cube: &mut Vec<Cubie>, mover: Mover, param: &mut Parametros) {
        if self.animating {
            self.angle += self.dir as f32 * self.speed;
            if (self.angle).abs() > PI / 2.0 {
                self.angle = 0.0;
                self.animating = false;
                self.finished = true;
                if self.z.abs() > 0 {
                    turn_z(mover.z, mover.dir, cube, param);
                } else if self.x.abs() > 0 {
                    turn_x(mover.x, mover.dir, cube, param);
                } else if self.y.abs() > 0 {
                    turn_y(mover.y, mover.dir, cube, param);
                }
            }
        }
    }
}
