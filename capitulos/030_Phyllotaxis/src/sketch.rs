use raylib::prelude::*;

use libfinal::engine::Engine;
//use libfinal::rendering::createcanvas;
use libfinal::p5::color_p5::{background, colormode, fill1, fill3, fill4, no_stroke};
use libfinal::parametros::{ColorMode, Parametros};
use libfinal::shapes::{ellipse, line};

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 1200;
pub const ALTO: i32 = 800;


// Aqui vendrá el pseudocódigo javascript
pub struct Sketch {
    pub engine: Engine,
    // Variables globales del scketch
    n: f32,
    c: f32,
}

impl Sketch {
    pub fn new() -> Sketch {
        let mut engine = Engine::new(ANCHO as f32, ALTO as f32);
        Sketch {
            engine,
            n: 0.0,
            c: 4.0,
        }
    }
    // Función setup() de javascript
    //pub fn setup(&mut self, d: &mut RaylibDrawHandle) {
    pub fn setup(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {
        //createcanvas(&mut self.engine, 400, 400);
        let mut d = rl.begin_drawing(&th);
        background(&mut self.engine, &mut d, 0);
        colormode(ColorMode::HSB, &mut self.engine.param);
    }

    //pub fn update(&mut self, d: &mut RaylibDrawHandle) -> bool {
    pub fn update(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) -> bool {
        //println!("En update");
        if !self.engine.update(rl, th) {
            return false;
        }
        true
    }

    // Función draw() de javascript
    //pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        let a = self.n * 137.5;

        let r = self.c * self.n.sqrt();

        let x = r * a.cos() + self.engine.param.ancho / 2.0;
        let y = r * a.sin() + self.engine.param.alto / 2.0;

        //fill1(255, &mut self.engine.param);
        fill3((a - r) % 255.0, 255.0, 255.0, &mut self.engine.param);
        no_stroke(&mut self.engine.param);
        ellipse(&mut self.engine.param, d, x, y, 4.0, 4.0);
        //line(&mut self.engine, &mut d, x, y, 4.0, 4.0);
        //ellipse(&mut self.engine.canvas.as_mut().unwrap(), &mut self.engine.param, x, y, 4.0, 4
        // .0);


        self.n += 1.0;
    }
}