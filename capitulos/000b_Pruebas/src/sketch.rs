use libfinal::constantes::TWO_PI;
//use libfinal::dom::Slider;
use libfinal::engine::Engine;
//use libfinal::environment::frame_rate;
use libfinal::matem::{floor, random, random_usize};
use libfinal::p5::color_p5::{background, fill1, fill3, no_stroke, stroke1};
use libfinal::p5::vector_p5::Vector2Fi;
use libfinal::parametros::CodigosTecla;
use libfinal::rendering::createcanvas;
use libfinal::shapes::{ellipse, line, rect, stroke_weight};
use libfinal::structure::{pop, push};
use libfinal::transform::{rotate_z, translate};
use libfinal::typography::{text, text_size};

//use libfinal::p5::sound::SoundFile;

// Ancho y alto de la pantalla
pub const ANCHO: u32 = 400;
pub const ALTO: u32 = 400;

// Aqui vendrá el pseudocódigo javascript

pub struct Sketch {
    pub engine: Engine,
    // Variables globales del scketch
    //angle: f32,
    //    slider: Slider,
    //cantidad_de_sliders: i32,
}

impl Sketch {
    pub fn new() -> Sketch {
        let mut engine = Engine::new();
        let alto = engine.param.alto;
        let ancho = engine.param.ancho;

        Sketch {
            engine,
            //angle: 0.0,
            //            slider: Slider::new(0.0, alto - 20.0,
            //                                0.0, TWO_PI,
            //                                20.0, ancho / 2.0),
            //cantidad_de_sliders: 0,
        }
    }

    // Función setup() de javascript
    pub fn setup(&mut self) {
        createcanvas(&mut self.engine, ANCHO, ALTO);
    }

    pub fn update(&mut self) -> bool {
        if !self.engine.update() {
            return false;
        }
        true
    }

    // Función draw() de javascript
    pub fn draw(&mut self) {
        background(&mut self.engine, 51);
        stroke1(255.0, &mut self.engine.param);

        translate(200.0, 100.0, &mut self.engine.param);
        rect(
            &mut self.engine.canvas.as_mut().unwrap(),
            &mut self.engine.param,
            0.0,
            0.0,
            40.0,
            80.0,
        );

        //        self.angle = self.slider.value(self.engine);
        //        stroke1(255.0, &mut self.engine.param);
        //        translate(200.0, ALTO as f32, &mut self.engine.param);
        //        self.branch(100.0);
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
