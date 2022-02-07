use libfinal::engine::Engine;
use libfinal::p5::color_p5::{background, fill1};
use libfinal::parametros::CodigosTecla;
use libfinal::rendering::createcanvas;
use libfinal::typography::{text, text_size};

//use crate::posibles::Posibles;
use crate::sudoku::Sudoku;

//use libfinal::p5::sound::SoundFile;

// Ancho y alto de la pantalla
pub const ANCHO: u32 = 600;
pub const ALTO: u32 = 400;

// Aqui vendrá el pseudocódigo javascript

pub struct Sketch {
    pub engine: Engine,
    // Variables globales del scketch
}

impl Sketch {
    pub fn new() -> Sketch {
        let engine = Engine::new();
        //let w = engine.param.ancho;
        //let h = engine.param.alto;

        Sketch { engine }
    }

    // Función setup() de javascript
    pub fn setup(&mut self) {
        createcanvas(&mut self.engine, ANCHO, ALTO);
        //let b = [true; 9];
        //let b = [true, true, true, true, true, true, true, true, true];
        //        let mut p = Posibles::new(b);
        //        p.elimina(5);
        //        p.elimina(3);
        //        p.elimina(6);
        //p.str();
        // println!("esta activo el 2 -> {}", p.activo(2));
        //println!("num activos -> {}", p.num_activos());

        let mut s = Sudoku::inicializa();
        //s.new("4.....8.5.3..........7......2.....6.....8.4......1.......6.3.7.5..2.....1.4......");
        //s.new("003020600900305001001806400008102900700000008006708200002609500800203009005010300");
        s.new("000000000000000000000000000000000000000000000000000000000000000000000000000000000");
        s.escribe();
    }

    pub fn update(&mut self) -> bool {
        if !self.engine.update() {
            return false;
        }
        true
    }

    // Función draw() de javascript
    pub fn draw(&mut self) {
        background(&mut self.engine, 0);
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
