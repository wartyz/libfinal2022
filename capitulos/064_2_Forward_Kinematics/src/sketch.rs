use raylib::prelude::*;

use libfinal::constantes::TWO_PI;
use libfinal::engine::Engine;
use libfinal::p5::color_p5::background;
//use libfinal::rendering::createcanvas;
use std::cell::RefCell;
use std::rc::Rc;

use crate::segment::Segment;

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 1200;
pub const ALTO: i32 = 800;


// Aqui vendrá el pseudocódigo javascript


pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    //seg1: Option<Rc<RefCell<Segment>>>,
    //seg2: Option<Rc<RefCell<Segment>>>,
    tentacle: Option<Rc<RefCell<Segment>>>,
}

impl Sketch {
    pub fn new() -> Sketch {
        let mut engine = Engine::new(ANCHO as f32, ALTO as f32);
        //let w = engine.param.ancho_pantalla;
        //let h = engine.param.alto_pantalla;

//        let seg1 = Segment::new4(300.0, 200.0, 100.0,
//                                 -45.0f32.to_radians());
//        let seg2 = Segment::new3(Some(Rc::new(RefCell::new(seg1.clone()))), 100.0,
//                                 -0.0f32.to_radians());


//        let seg1 = Segment::new4(300.0, 200.0, 100.0, -45.0f32.to_radians());
//        let seg1a = Rc::new(RefCell::new(seg1));
        //let seg1b = Rc::new(seg1a);


//        let seg2 = Segment::new3(Some(seg1a.clone()), 100.0, -45.0f32.to_radians());
//        let seg2a = Rc::new(RefCell::new(seg2));
        //let seg2b = Rc::new(seg2a);


        Sketch {
            engine,
            tentacle: None,
            //seg2: Some(seg2a),
        }
    }
    // Función setup() de javascript
    pub fn setup(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {
        //println!("En setup");
        //createcanvas(&mut self.engine, 600, 400);
        let mut t = 0.0;
        let mut len = 50.0;
        let seg1 = Segment::new4(300.0, 200.0, len, 0.0f32.to_radians(), t);
        self.tentacle = Some(Rc::new(RefCell::new(seg1)));

        let mut current = self.tentacle.clone();
        for i in 0..20 {
            t += 0.1;
            len *= 0.75;
            let next = Segment::new3(current.clone(), len, 0.0f32.to_radians(), t);


            current.as_ref().unwrap().borrow_mut().child = Some(Rc::new(RefCell::new(next.clone())));
            current = Some(Rc::new(RefCell::new(next)));
        }
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

        let mut next = self.tentacle.clone();
        while next.is_some() {
            let kk = next.as_ref().unwrap();

            kk.borrow_mut().wiggle();
            kk.borrow_mut().update();
            kk.borrow_mut().show(&mut self.engine, d);
            next = next.unwrap().borrow_mut().child.clone();
        }
    }
}