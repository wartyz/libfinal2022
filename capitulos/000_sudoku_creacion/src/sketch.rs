use libfinal::engine::Engine;
use libfinal::matem::random_usize;
use libfinal::p5::color_p5::{background, fill1};
use libfinal::parametros::CodigosTecla;
use libfinal::rendering::createcanvas;
use libfinal::typography::{text, text_size};

use crate::ayuda_sudoku::Azar;
use crate::posibles::Posibles;
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

        Sketch { engine }
    }

    // Función setup() de javascript
    pub fn setup(&mut self) {
        createcanvas(&mut self.engine, ANCHO, ALTO);

        let mut s = Sudoku::new();
        //let mut azar = Azar::new();

        loop {
            s.rellena_posibles();
            //println!("{}", s.cuentaceros());
            for fila in 0..9 {
                // grupos totales
                for columna in 0..9_usize {
                    let mut elegido = 0;

                    if let Some(i) = s.getaleatorio(fila, columna) {
                        elegido = i;

                        s.borra_n_en_fila(elegido, fila); // h
                        s.borra_n_en_columna(elegido, columna); // v
                        s.borra_n_en_cuadrado(elegido, fila, columna);
                        s.pone_en_acabado(elegido, fila, columna);
                    } else {}
                }

                if s.cuentaceros() == 0 && !s.hayrepetidos() {
                    break;
                }
            }
        }
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
