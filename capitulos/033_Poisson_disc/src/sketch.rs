use raylib::prelude::*;

use libfinal::engine::Engine;
use libfinal::matem::{floor, random, random_range};
use libfinal::p5::color_p5::{background, colormode, stroke3};
use libfinal::p5::vector_p5::Vector3;
use libfinal::parametros::ColorMode;
//use libfinal::rendering::createcanvas;
use libfinal::shapes::{point, stroke_weight};

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 1200;
pub const ALTO: i32 = 800;

// Aqui vendrá el pseudocódigo javascript
pub struct Sketch {
    pub engine: Engine,
    r: f32,
    k: i32,
    grid: Vec<Option<Vector3>>,
    w: f32,
    active: Vec<Vector3>,
    cols: i32,
    rows: i32,
    ordered: Vec<Vector3>,
    // Variables globales del scketch
    //blob: Option<Blob>,
    //blobs: Vec<Blob>,
    //zoom: f32,
    //n: f32,
    //c: f32,
}

impl Sketch {
    pub fn new() -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);
        //let w = engine.param.ancho_pantalla;
        //let h = engine.param.alto_pantalla;
        let r = 4.0;
        Sketch {
            engine,
            r: r,
            k: 30,
            grid: vec![],
            w: r / 2.0_f32.sqrt(),
            active: vec![],
            cols: 0,
            rows: 0,
            ordered: vec![],
            //blob: None,
            //blobs: vec![],
            //zoom: 1.0,
            //n: 0.0,
            //c: 4.0,
        }
    }
    // Función setup() de javascript
    pub fn setup(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {
        //println!("En setup");
        //createcanvas(&mut self.engine, 400, 400);
        let mut d = rl.begin_drawing(&th);
        let w = self.engine.param.ancho;
        let h = self.engine.param.alto;

        background(&mut self.engine, &mut d, 0);
        stroke_weight(4.0, &mut self.engine.param);
        colormode(ColorMode::HSB, &mut self.engine.param);

        // STEP 0
        self.cols = floor(w / self.w) as i32;
        self.rows = floor(h / self.w) as i32;
        for _ in 0..(self.cols * self.rows) as usize {
            self.grid.push(None);
        }
        // STEP 1
        let x = w / 2.0;
        let y = h / 2.0;
        let i = floor(x / self.w) as i32;
        let j = floor(y / self.w) as i32;
        let pos = Some(Vector3::new(x, y, 0.0));
        self.grid[(i + j * self.cols) as usize] = pos.clone();
        self.active.push(Vector3::new(x, y, 0.0));
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) -> bool {
        //println!("En update");
        if !self.engine.update(rl, th) {
            return false;
        }
        true
    }

    // Función draw() de javascript
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        background(&mut self.engine, d, 0);
        for _ in 0..25 {
            if self.active.len() > 0 {
                let rand_index = floor(random(self.active.len() as f32)) as usize;
                let pos = self.active[rand_index].clone();
                let mut found = false;
                for _ in 0..self.k {
                    let mut sample = Vector3::random2d_s();
                    let m = random_range(self.r, 2.0 * self.r);
                    sample.set_mag(m);
                    sample.add(&pos);

                    let col = floor(sample.x / self.w) as i32;
                    let row = floor(sample.y / self.w) as i32;

                    if col > -1 && row > -1 && col < self.cols && row < self.rows {
                        let mut ok = true;
                        for i in -1..=1 {
                            for j in -1..=1 {
                                //println!("i = {} j = {} col = {} row = {}", i, j, col, row);

                                let index = (col + i) + (row + j) * self.cols as i32;
                                //println!("index = {} self.grid.len() = {}", index, self.grid.len());
                                // Aqui da error de indice out of bounds LO PONGO YO
                                if (index >= 0) || (index < (self.grid.len() - 1) as i32) {
                                    //println!("2index = {} self.grid.len() = {}", index, self.grid.len());

                                    let neighbor = self.grid[index as usize].clone();
                                    if neighbor.is_some() {
                                        let d = Vector3::dist_s(&sample, &neighbor.unwrap());
                                        if d < self.r {
                                            ok = false;
                                        }
                                    }
                                }
                            }
                        }
                        if ok {
                            found = true;
                            self.grid[(col + row * self.cols as i32) as usize] =
                                Some(sample.clone());
                            let sample2 = sample.clone();
                            self.active.push(sample);
                            self.ordered.push(sample2);
                            // Should we break?
                            //break;
                        }
                    }
                }
                if !found {
                    self.active.remove(rand_index);
                }
            }
        }
        //let p = &mut self.engine.param; // Nuevo invento ***********
        for i in 0..self.ordered.len() {
            //if self.ordered[i].is_some() {  // No entiendo
            stroke3(i as f32 % 360.0, 100.0, 100.0, &mut self.engine.param);
            stroke_weight(self.r * 0.5, &mut self.engine.param);
            point(
                d,
                &mut self.engine.param,
                self.ordered[i].x,
                self.ordered[i].y,
            );
            //            point(
            //                &mut self.engine.canvas.as_mut().unwrap(),
            //                p,
            //                self.ordered[i].x,
            //                self.ordered[i].y,
            //            );
            //}
        }
    }
}
