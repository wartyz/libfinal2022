use raylib::prelude::*;

use libfinal::*;
use libfinal::engine::Engine;
use libfinal::matem::mapa;
use libfinal::transform::translate;

use crate::star::Star;

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 600;
pub const ALTO: i32 = 600;


// Aqui vendrá el pseudocódigo javascript
pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    stars: Vec<Star>,
    max_stars: i32,
    speed: f32,
}

//impl Default for Sketch {
//    fn default() -> Self {
//        Self::new(100.0, 200.0)
//    }
//}

impl Sketch {
    pub fn new() -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);

        Sketch {
            engine,
            stars: vec![],
            max_stars: 800,
            speed: 0.0,
        }
    }
    // Función setup() de javascript
    pub fn setup(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {
//    pub fn setup(&mut self) {

        //println!("En setup");
        //createcanvas(&mut self.engine, 800, 800);

        for _ in 0..self.max_stars {
            self.stars.push(Star::new(&self.engine.param));
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) -> bool {
        //pub fn update(&mut self) -> bool {
        //println!("En update");
        //       if !self.engine.update(rl, &th) {
        if !self.engine.update(rl, th) {
            return false;
        }
        true
    }

    // Función draw() de javascript

    pub fn draw(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {
        //pub fn draw(&mut self) {
        // Creamos el único RaylibDrawHandle
        let mut d = rl.begin_drawing(&th);
        //println!("en draw() self.engine.param.mousex = {}", self.engine.param.mousex);
        self.speed = mapa(
            self.engine.param.mouse_posicion.x as f32,
            0.0,
            self.engine.param.ancho as f32,
            0.0,
            50.0,
        );
        //println!("en draw() self.speed = {}", self.speed);
        p5::color_p5::background(&mut self.engine, &mut d, 55);


        /*
                d.draw_text("Hello, world! ", 12, 12, 20, Color::BLACK);
                d.draw_ellipse(200, 300, 200.0, 100.0, Color::BLUE);

                d.draw_ellipse_lines(300, 300, 200.0, 100.0, Color::BEIGE);
        */

        //shapes::ellipse(&mut self.engine, d, 20.0, 40.0, 200.0, 100.0);


        //shapes::line(&mut self.engine, d, 100.0, 20.0, 300.0 * 2.0, 400.0 * 2.0);


        //let mut d = self.engine.rl.begin_drawing(&self.engine.th);
        //shapes::line(&mut self.engine, 200.0, 20.0, 300.0 * 2.0, 400.0 * 2.0);

        //shapes::ellipse(&mut self.engine, d, 0.0, 0.0, 200.0, 100.0);
        translate(
            self.engine.param.ancho / 2.0,
            self.engine.param.alto / 2.0,
            &mut self.engine.param,
        );
        //shapes::ellipse(&mut self.engine, d, 0.0, 0.0, 200.0, 100.0);
        for i in 0..self.stars.len() {
            self.stars[i].update(&self.engine, self.speed);
            self.stars[i].show(&mut self.engine, &mut d);
        }


        //println!("En draw mousex = {:}", self.engine.param.mousex);
        //println!("En draw engine.param.ancho_pantalla = {:}", self.engine.param.ancho);
//        self.speed = mapa(
//            self.engine.param.mousex,
//            0.0,
//            self.engine.param.ancho,
//            0.0,
//            50.0,
//        );
//        background(&mut self.engine, 0);
//        //println!("En draw speed = {:?}", self.speed);
//
//        translate(
//            self.engine.param.ancho / 2.0,
//            self.engine.param.alto / 2.0,
//            &mut self.engine.param,
//        );
//
//        for i in 0..self.stars.len() {
//            self.stars[i].update(&mut self.engine.param, &self.speed);
//            self.stars[i].show(&mut self.engine);
//        }
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
