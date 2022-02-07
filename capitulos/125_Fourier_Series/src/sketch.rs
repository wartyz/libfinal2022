//https://www.youtube.com/watch?v=Mm2eYfj0SgA

use raylib::prelude::*;

use libfinal::ambient::draw_fps_fi;
use libfinal::constantes::*;
use libfinal::engine::Engine;
use libfinal::matem::*;
use libfinal::p5::color_p5::*;
use libfinal::p5::image_p5::P5Image;
use libfinal::parametros::*;
use libfinal::parametros::Filtros::Gray;
use libfinal::shapes::*;
use libfinal::transform::translate;
use libfinal::typography::{text, text_size};

// Constantes del sketch

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 600;
pub const ALTO: i32 = 400;

// Aqui vendrá el pseudocódigo javascript

pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    time: f32,
    wave: Vec<f32>,
    slider: usize,
}

impl Sketch {
    pub fn new(rl: &mut RaylibHandle, th: &RaylibThread) -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);
        // Load image in CPU memory (RAM)


        Sketch {
            engine,
            time: 0.0,
            wave: vec![],
            slider: 0,
        }
    }

    // Función setup() de javascript
    pub fn setup(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {}

    pub fn update(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) ->
    bool {
        if !self.engine.update(rl, th) {
            return false;
        }
        true
    }


    // Función draw() de javascript
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {


        // Draw GUI controls
        //------------------------------------------------------------------------------
        self.slider = d.gui_slider_bar(
            Rectangle::new(10.0, 40.0, 120.0, 20.0),
            None,
            Some(rstr!("Slider")),
            self.slider as f32,
            1.0,
            100.0,
        ) as usize;


        background(&mut self.engine, d, 0);
        translate(150.0, 200.0, &mut self.engine.param);


        let mut x = 0.0;
        let mut y = 0.0;

        for i in 0..self.slider {
            let prevx = x;
            let prevy = y;

            let n = i as f32 * 2.0 + 1.0;
            let radius: f32 = 75.0 * 4.0 / (n * PI as f32);
            x += radius * cos(n * self.time);
            y += radius * sin(n * self.time);


            stroke2(255.0, 100.0, &mut self.engine.param);
            no_fill(&mut self.engine.param);
            ellipse(&mut self.engine.param, d, prevx, prevy, radius * 2.0, radius * 2.0);


            //fill1(255.0, &mut self.engine.param);
            stroke1(255.0, &mut self.engine.param);
            line(d, &mut self.engine.param, prevx, prevy, x, y);
            //ellipse(&mut self.engine.param, &mut d, x, y, 8.0, 8.0);
        }
        self.wave.insert(0, y); // Inserta al inicio

        translate(200.0, 0.0, &mut self.engine.param);
        line(d, &mut self.engine.param, x - 200.0, y, 0.0, self.wave[0]);

        begin_shape(ModosBeginShape::NadaShape);
        no_fill(&mut self.engine.param);
        for i in 0..self.wave.len() {
            //line(&mut d, &mut self.engine.param, 0.0, 0.0, x, y);
            vertex(i as f32, self.wave[i], &mut self.engine.param);
        }
        end_shape(d, &mut self.engine.param, ModosBeginShape::NadaShape);

        self.time += 0.05;

        if self.wave.len() > 250 {
            self.wave.pop();
        }
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
