use raylib::prelude::*;

use libfinal::constantes::TWO_PI;
use libfinal::engine::Engine;
use libfinal::matem::radians;
use libfinal::p5::color_p5::background;
//use libfinal::rendering::createcanvas;
use std::cell::RefCell;
use std::rc::Rc;

use crate::segment::Segment;

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 600;
pub const ALTO: i32 = 400;


// Aqui vendrá el pseudocódigo javascript


pub struct Sketch {
    pub engine: Engine,
    tentacle: Segment,
    //seg2: Segment,
    segments: Vec<Segment>,
    //current: usize,

    // Variables globales del scketch
    //seg1: Option<Rc<RefCell<Segment>>>,
    //seg2: Option<Rc<RefCell<Segment>>>,
    //tentacle: Option<Rc<RefCell<Segment>>>,
}

impl Sketch {
    pub fn new() -> Sketch {
        let mut engine = Engine::new(ANCHO as f32, ALTO as f32);
        let mut segments = vec![];
        let mut t = 0.0;
        let mut len = 50.0;

        // El segmento tentacle es el primero
        let tentacle = Segment::new4(ANCHO as f32 / 2.0, ALTO as f32, len, 0.0, t);
        segments.push(tentacle);


        //let mut current = 0;
        for current in 1..20 {
            t += 0.1;
            len = len * 0.75;
            let next = Segment::new2(current, tentacle.b.x, tentacle.b.y, len, 0.0, t);

            segments.push(next);
            segments[current - 1].child = Some(current);
        }


        // Le indicamos que el parent es el elemento 0 en segments
        //let seg2 = Segment::new2(0, tentacle.b.x, tentacle.b.y, 100.0, radians(-45.0));
        //segments.push(seg2);

        /*--------------------------------------------------------*/
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
            tentacle,
            //seg2,
            segments,
            //tentacle: None,
            //seg2: Some(seg2a),
        }
    }
    // Función setup() de javascript
    pub fn setup(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {
        //println!("En setup");
        //createcanvas(&mut self.engine, 600, 400);
//        let mut t = 0.0;
//        let mut len = 50.0;
//        let seg1 = Segment::new4(300.0, 200.0, len, 0.0f32.to_radians(), t);
//        self.tentacle = Some(Rc::new(RefCell::new(seg1)));
//
//        let mut current = self.tentacle.clone();
//        for i in 0..20 {
//            t += 0.1;
//            len *= 0.75;
//            let next = Segment::new3(current.clone(), len, 0.0f32.to_radians(), t);
//
//
//            current.as_ref().unwrap().borrow_mut().child = Some(Rc::new(RefCell::new(next.clone())));
//            current = Some(Rc::new(RefCell::new(next)));
//        }
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
        //d.draw_fps(10, 10);
        background(&mut self.engine, d, 51);


        self.segments[0] = self.segments[0].wiggle();
        self.segments[0] = self.segments[0].update(self.segments[0]);
        self.segments[0].show(&mut self.engine, d);

        let mut i = 1;
        //for i in 1..5 {
        while self.segments[i].child.is_some() {
            self.segments[i] = self.segments[i].wiggle();
            self.segments[i] = self.segments[i].update(self.segments[i - 1]);
            self.segments[i].show(&mut self.engine, d);
            i += 1;
        }
//
//        self.segments[1] = self.segments[1].wiggle();
//        self.segments[1] = self.segments[1].update(self.segments[0]);
//        self.segments[1].show(&mut self.engine, d);
//
//
//        self.segments[2] = self.segments[2].wiggle();
//        self.segments[2] = self.segments[2].update(self.segments[1]);
//        self.segments[2].show(&mut self.engine, d);

        /* ---------------------------------------------------------------------------*/
//        let mut next = self.tentacle.clone();
//        while next.is_some() {
//            let kk = next.as_ref().unwrap();
////            next.as_ref().unwrap().borrow_mut().wiggle();
////            next.as_ref().unwrap().borrow_mut().update();
////            next.as_ref().unwrap().borrow_mut().show(&mut self.engine, &mut d);
////            next = next.unwrap().borrow_mut().child.clone();
//            kk.borrow_mut().wiggle();
//            kk.borrow_mut().update();
//            kk.borrow_mut().show(&mut self.engine, d);
//            next = next.unwrap().borrow_mut().child.clone();
//        }
//
////        self.seg1.as_ref().unwrap().borrow_mut().wiggle();
////        self.seg1.as_ref().unwrap().borrow_mut().update();
////
////
////        self.seg2.as_ref().unwrap().borrow_mut().update();
////        self.seg2.as_ref().unwrap().borrow_mut().wiggle();
////        self.seg2.as_ref().unwrap().borrow_mut().show(&mut self.engine);
    }
}