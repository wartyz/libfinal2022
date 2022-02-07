use raylib::prelude::*;

use libfinal::engine::Engine;

use libfinal::matem::lerp;
use libfinal::p5::color_p5::*;
use libfinal::parametros::{ColorMode, ModosBeginShape, Parametros};
use libfinal::shapes::{begin_shape, bezier, end_shape, line, point, stroke_weight, vertex};
use libfinal::transform::translate;

// Constantes del sketch

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 600;
pub const ALTO: i32 = 600;

// Aqui vendrá el pseudocódigo javascript

pub struct Sketch {
    pub engine: Engine,
    // Variables globales del scketch
    p0: Vector3,
    p1: Vector3,
    p2: Vector3,
    p3: Vector3,
}

impl Sketch {
    pub fn new(rl: &mut RaylibHandle, th: &RaylibThread) -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);
        let p0 = Vector3::new(0.0, 300.0, 1.0); // anclaje
        let p1 = Vector3::new(300.0, 0.0, 1.0); // control
        let p2 = Vector3::new(400.0, 0.0, 1.0); // control
        let p3 = Vector3::new(600.0, 300.0, 1.0); // anclaje
        Sketch {
            engine,
            p0,
            p1,
            p2,
            p3,
        }
    }

    // Función setup() de javascript
    pub fn setup(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {}

    pub fn update(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) -> bool {
        if !self.engine.update(rl, th) {
            return false;
        }
        true
    }

    // Función draw() de javascript
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        background(&mut self.engine, d, 0);

        // p1 es el punto de control
        self.p1.x = self.engine.param.mouse_posicion.x;
        self.p1.y = self.engine.param.mouse_posicion.y;

        //stroke1(255.0, &mut self.engine.param);
        stroke_weight(4.0, &mut self.engine.param);

        let delta = 0.05;
        let paso = (1.0 / delta) as usize;

        colormode(ColorMode::HSB, &mut self.engine.param);

        no_fill(&mut self.engine.param); // No la uso
                                         //begin_shape(ModosBeginShape::Lines);
        for t in 0..=paso {
            let t = t as f32 * delta;

            stroke3(t * 360.0, 255.0, 255.0, &mut self.engine.param);
            //line(d, pm, v1.x, v1.y, v2.x, v2.y);
            let v = Self::cubic(
                d,
                &mut self.engine.param,
                self.p0,
                self.p1,
                self.p2,
                self.p3,
                t,
            );

            //vertex(v.x, v.y, &mut self.engine.param);
        }
        //end_shape(d, pm, ModosBeginShape::Lines);
        /*stroke1(255.0, &mut self.engine.param);
        translate(
            self.engine.param.ancho / 2.0,
            self.engine.param.alto / 2.0,
            &mut self.engine.param,
        );
        bezier(
            d,
            &mut self.engine.param,
            0.0,
            0.0,
            100.0,
            200.0,
            200.0,
            -200.0,
            300.0,
            0.0,
        );*/
    }
    pub fn cubic(
        d: &mut RaylibDrawHandle,
        param: &mut Parametros,
        p0: Vector3,
        p1: Vector3,
        p2: Vector3,
        p3: Vector3,
        t: f32,
    ) -> Vector3 {
        let v1 = Self::quadratic(p0, p1, p2, t);
        let v2 = Self::quadratic(p1, p2, p3, t);

        let x = lerp(v1.x, v2.x, t);
        let y = lerp(v1.y, v2.y, t);
        line(d, param, v1.x, v1.y, v2.x, v2.y);
        Vector3::new(x, y, 1.0)
    }
    pub fn quadratic(p0: Vector3, p1: Vector3, p2: Vector3, t: f32) -> Vector3 {
        let x1 = lerp(p0.x, p1.x, t);
        let y1 = lerp(p0.y, p1.y, t);
        let x2 = lerp(p1.x, p2.x, t);
        let y2 = lerp(p1.y, p2.y, t);
        // hacemos una curva bezier cuadratica
        let x = lerp(x1, x2, t);
        let y = lerp(y1, y2, t);
        Vector3::new(x, y, 1.0)
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
