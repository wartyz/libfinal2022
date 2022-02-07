use raylib::prelude::*;

use libfinal::constantes::TWO_PI;
use libfinal::engine::Engine;
use libfinal::matem::{constrain, floor, lerp, random, random_range};
use libfinal::p5::color_p5::{background, colormode, no_fill, stroke1, stroke2, stroke3};
use libfinal::parametros::{ColorMode, ModosBeginShape, Parametros};
//use libfinal::rendering::createcanvas;
use libfinal::shapes::{begin_shape, ellipse, end_shape, point, stroke_weight, vertex};
use libfinal::transform::{scale1, translate};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

use crate::orbit::Orbit;

//use crate::orbit::SingleLink;

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 600;
pub const ALTO: i32 = 600;

// Aqui vendrá el pseudocódigo javascript

pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    path: Vec<Vector2>,
    angle: f32,

    contador: usize,
    todos: Vec<Option<Orbit>>,

    resolution: f32,
    //
    //sun: Orbit,
    end: usize,
}

impl Sketch {
    pub fn new() -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);
        //let w = engine.param.ancho_pantalla;
        //let h = engine.param.alto_pantalla;
        let resolution: f32 = 10.0;

        Sketch {
            engine,

            path: vec![],
            contador: 0,
            todos: vec![],
            angle: 0.0,

            resolution,
            //sun: Orbit::new3(300.0, 300.0, 100.0, Some(0), None),
            end: 0,
        }
    }
    // Función setup() de javascript
    pub fn setup(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {
        /* -----------------------------------------------------------*/
        // Crea sol
        let sun = Orbit::new3(300.0, 300.0, 150.0, 0, self.resolution, Some(0), None);

        // Mete el sol en el arrglo todos
        self.todos.push(Some(sun));
        /* -----------------------------------------------------------*/

        for i in 0..10 {
            self.contador += 1;

            // Le dice al padre que tiene un hijo
            let mut s = self.todos[i].unwrap();
            s.child = Some(self.contador);
            self.todos[i] = Some(s);

            // Crea el hijo
            let padre_de = self.todos[i].unwrap();
            let orb1 = padre_de.add_child(self.resolution, Some(self.contador));

            // Mete el hijo creado en el arreglo todos
            self.todos.push(Some(orb1));
        }

        self.end = self.contador;
        println!("En setup");
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
        background(&mut self.engine, d, 51);
        //for _ in 0..self.resolution as usize {
        self.todos[0].unwrap().show(&mut self.engine, d);

        let mut next = Some(0);
        while next.is_some() {
            let indice = next.unwrap();
            //dbg!(indice);

            let indice_padre = self.todos[indice].unwrap().parent;
            let mut padre = None;
            if indice_padre.is_none() {
                padre = None;
            } else {
                padre = self.todos[indice_padre.unwrap()];
            }

            // Ojo al hacer unwrap() sacamos una copia, hay que volverla a meter con Some()
            self.todos[indice] = Some(self.todos[indice].unwrap().update(
                padre,
                d,
                &mut self.engine.param,
            ));

            self.todos[indice].unwrap().show(&mut self.engine, d);
            next = self.todos[indice].unwrap().child;
            //dbg!(next);
        }

        let obj_end = self.todos[self.end].unwrap();
        self.path.push(Vector2::new(obj_end.x, obj_end.y));
        //}

        begin_shape(ModosBeginShape::NadaShape);
        stroke3(255.0, 0.0, 255.0, &mut self.engine.param);
        stroke_weight(2.0, &mut self.engine.param);
        for pos in &self.path {
            vertex(pos.x, pos.y, &mut self.engine.param);
        }
        end_shape(d, &mut self.engine.param, ModosBeginShape::NadaShape);
    }
}
