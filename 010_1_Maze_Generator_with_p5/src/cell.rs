use raylib::prelude::*;

use libfinal::engine::Engine;
use libfinal::p5::color_p5::{no_fill, stroke1};
use libfinal::shapes::{line, rect};

pub struct Cell {
    // columna
    i: usize,
    // fila
    j: usize,
    walls: [bool; 4],
}

impl Cell {
    pub fn new(i: usize, j: usize) -> Cell {
        Cell {
            i,
            j,
            walls: [true, true, true, true],
        }
    }

    pub fn show(&mut self, w: f32, engine: &mut Engine, d: &mut RaylibDrawHandle) {
        //let cv = &mut engine.canvas.as_mut().unwrap();
        let pr = &mut engine.param;

        let x = self.i as f32 * w;
        let y = self.j as f32 * w;
        stroke1(255.0, pr);
        if self.walls[0] {
            line(d, &mut engine.param, x, y, x + w, y);
        }
        if self.walls[1] {
            line(d, &mut engine.param, x + w, y, x + w, y + w);
        }
        if self.walls[2] {
            line(d, &mut engine.param, x + w, y + w, x, y + w);
        }
        if self.walls[3] {
            line(d, &mut engine.param, x, y + w, x, y);
        }
        //no_fill(&mut engine.param);
        //rect( &mut engine.canvas.as_mut().unwrap(), &mut engine.param, x, y, w, w);
    }
}