use raylib::prelude::*;

use libfinal::ambient::draw_fps_fi;
use libfinal::constantes::*;
use libfinal::engine::Engine;
use libfinal::image::ImageLf;
use libfinal::matem::*;
use libfinal::p5::color_p5::Color;
use libfinal::p5::image_p5::P5Image;
use libfinal::p5::vector_p5::*;
use libfinal::parametros::*;
use libfinal::shapes::*;
use libfinal::typography::{text, text_size};

// Constantes del sketch

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 200;
pub const ALTO: i32 = 200;

// Aqui vendrá el pseudocódigo javascript
#[derive(Clone, Copy, Debug)]
struct Celda {
    a: f32,
    b: f32,
}

impl Celda {
    pub fn new(a: f32, b: f32) -> Celda {
        Celda { a, b }
    }
}

pub struct Sketch {
    pub engine: Engine,
    // Variables globales del scketch
    grid: Vec<Vec<Celda>>,
    next: Vec<Vec<Celda>>,
    da: f32,
    db: f32,

    feed: f32,
    k: f32,
}

impl Sketch {
    pub fn new(rl: &mut RaylibHandle, th: &RaylibThread) -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);

        //let fluid = Fluid::new(0.2, 0.0, 0.0000001);
        let mut grid: Vec<Vec<Celda>> =
            vec![vec![Celda::new(1.0, 0.0); ANCHO as usize]; ALTO as usize];
        let next: Vec<Vec<Celda>> = vec![vec![Celda::new(1.0, 0.0); ANCHO as usize]; ALTO as usize];

        //        for x in 0..ANCHO as usize {
        //            for y in 0..ALTO as usize {
        //                grid[x][y] = (Celda::new(random(1.0), random(1.0)));
        //                //next.push(Celda::new());
        //            }
        //        }
        for i in 100..110 {
            for j in 100..110 {
                grid[i][j].b = 1.0;
            }
        }

        Sketch {
            engine,
            grid,
            next,
            da: 1.0,
            db: 0.5,

            feed: 0.055,
            k: 0.062,
        }
    }

    // Función setup() de javascript
    pub fn setup(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {}

    pub fn update(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) -> bool {
        for x in 1..(ANCHO - 1) as usize {
            for y in 1..(ALTO - 1) as usize {
                //                let spot = self.prev[i][j];
                //                let mut newspot = self.grid[i][j];
                //
                //                let a = spot.a;
                //                let b = spot.b;
                //
                //                let mut laplacea = 0.0;
                //                laplacea += a * -1.0;
                //                laplacea += self.prev[i + 1][j].a * 0.2;
                //                laplacea += self.prev[i - 1][j].a * 0.2;
                //                laplacea += self.prev[i][j + 1].a * 0.2;
                //                laplacea += self.prev[i][j - 1].a * 0.2;
                //                laplacea += self.prev[i - 1][j - 1].a * 0.05;
                //                laplacea += self.prev[i + 1][j - 1].a * 0.05;
                //                laplacea += self.prev[i - 1][j + 1].a * 0.05;
                //                laplacea += self.prev[i + 1][j + 1].a * 0.05;
                //
                //                let mut laplaceb = 0.0;
                //                laplaceb += b * -1.0;
                //                laplaceb += self.prev[i + 1][j].b * 0.2;
                //                laplaceb += self.prev[i - 1][j].b * 0.2;
                //                laplaceb += self.prev[i][j + 1].b * 0.2;
                //                laplaceb += self.prev[i][j - 1].b * 0.2;
                //                laplaceb += self.prev[i - 1][j - 1].b * 0.05;
                //                laplaceb += self.prev[i + 1][j - 1].b * 0.05;
                //                laplaceb += self.prev[i - 1][j + 1].b * 0.05;
                //                laplaceb += self.prev[i + 1][j + 1].b * 0.05;
                //
                //                newspot.a = a + (self.da * laplacea - a * b * b + self.feed * (1.0 - a)) * 1.0;
                //                newspot.b = b + (self.db * laplaceb + a * b * b - (self.k + self.feed) * b) * 1.0;
                //
                //                newspot.a = constrain(newspot.a, 0.0, 1.0);
                //                newspot.b = constrain(newspot.b, 0.0, 1.0);

                let a = self.grid[x][y].a;
                let b = self.grid[x][y].b;

                self.next[x][y].a =
                    a + self.da * self.laplace_a(x, y) - a * b * b + self.feed * (1.0 - a);

                self.next[x][y].b =
                    b + self.db * self.laplace_b(x, y) + a * b * b - (self.k + self.feed) * b;

                self.next[x][y].a = constrain(self.next[x][y].a, 0.0, 1.0);
                self.next[x][y].b = constrain(self.next[x][y].b, 0.0, 1.0);
            }
        }

        ImageLf::load_pixels(&mut self.engine.param, rl, th);

        if !self.engine.update(rl, th) {
            return false;
        }
        true
    }

    // Función draw() de javascript
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        {
            //            let mut d = rl.begin_drawing(&th);
            //            background(&mut self.engine, &mut d, 245); //RAYWHITE
        }

        for x in 0..ANCHO as usize {
            for y in 0..ALTO as usize {
                let pix = x + y * ANCHO as usize;

                let a = self.next[x][y].a;
                let b = self.next[x][y].b;
                let mut c = floor((a - b) * 255.0);
                c = constrain(c, 0.0, 255.0);

                //let color = Color::new(c as u8, c as u8, c as u8, 255);
                let color = raylib::prelude::Color::new(c as u8, c as u8, c as u8, 255);

                //self.engine.param.pixels[pix] = color;
                self.engine.param.pixels[pix] = color;
            }
        }

        ImageLf::update_pixels(&mut self.engine.param, d, ANCHO, ALTO);

        self.swap();
    }

    pub fn swap(&mut self) {
        let mut temp = vec![vec![Celda::new(0.0, 0.0); ANCHO as usize]; ALTO as usize];
        for i in 0..self.grid.len() {
            for j in 0..self.next.len() {
                temp[i][j] = self.grid[i][j].clone();
                self.grid[i][j] = self.next[i][j].clone();
                self.next[i][j] = temp[i][j].clone();
            }
        }
    }

    pub fn laplace_a(&mut self, x: usize, y: usize) -> f32 {
        //pub fn laplace_a(&mut self) -> f32 {
        let mut suma: f32 = 0.0;

        suma += self.grid[x][y].a * -1.0;
        suma += self.grid[x - 1][y].a * 0.2;
        suma += self.grid[x + 1][y].a * 0.2;
        suma += self.grid[x][y + 1].a * 0.2;
        suma += self.grid[x][y - 1].a * 0.2;
        suma += self.grid[x - 1][y - 1].a * 0.05;
        suma += self.grid[x + 1][y - 1].a * 0.05;
        suma += self.grid[x + 1][y + 1].a * 0.05;
        suma += self.grid[x - 1][y + 1].a * 0.05;

        suma
    }
    pub fn laplace_b(&mut self, x: usize, y: usize) -> f32 {
        //pub fn laplace_b(&mut self) -> f32 {
        let mut sumb: f32 = 0.0;

        sumb += self.grid[x][y].b * -1.0;
        sumb += self.grid[x - 1][y].b * 0.2;
        sumb += self.grid[x + 1][y].b * 0.2;
        sumb += self.grid[x][y + 1].b * 0.2;
        sumb += self.grid[x][y - 1].b * 0.2;
        sumb += self.grid[x - 1][y - 1].b * 0.05;
        sumb += self.grid[x + 1][y - 1].b * 0.05;
        sumb += self.grid[x + 1][y + 1].b * 0.05;
        sumb += self.grid[x - 1][y + 1].b * 0.05;

        sumb
    }

    pub fn key_pressed(&mut self) {
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
