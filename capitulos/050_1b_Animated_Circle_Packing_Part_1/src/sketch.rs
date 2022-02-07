use raylib::prelude::*;

use libfinal::engine::Engine;
use libfinal::matem::*;
use libfinal::p5::color_p5::{background, brightness, Color};
use libfinal::p5::image_p5::P5Image;
use libfinal::p5::vector_p5::Vector3;

use crate::circle::Circle;

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 900;
pub const ALTO: i32 = 400;

// Aqui vendrá el pseudocódigo javascript
pub struct Sketch {
    pub engine: Engine,
    // Variables globales del scketch
    circles: Vec<Circle>,
    imagen: Image,
    spots: Vec<Vector2>,
}

impl Sketch {
    pub fn new(rl: &mut RaylibHandle, th: &RaylibThread) -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);
        let imagen = P5Image::load_image_fi("resources/imagenes/2017.png").img;
        Sketch {
            engine,
            circles: vec![],
            imagen,
            spots: vec![],
        }
    }
    // Función setup() de javascript
    pub fn setup(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {
        //createcanvas(&mut self.engine, 900, 400);
        //self.img.load_pixels(); // Por ahora no hace nada

        //println!("spots.len() = {:?}", self.spots.len());

        self.circles.push(Circle::new(200.0, 200.0));
        //        for i in 0..(self.max_stars) {
        //            self.stars.push(Star::new(&mut self.engine.param));
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
        let mut v_pix = self.imagen.get_image_data();

        let anch = self.imagen.width();
        let alt = self.imagen.height();

        //for x in (0..self.img.image_width as usize * 4).step_by(4) {
        for x in 0..anch {
            //            for y in 0..self.img.image_height as usize {
            for y in 0..alt {
                //println!(" p = {:?}", self.img.image_pixels[y as usize]);

                let index = x + y * anch;
                let r = v_pix[index as usize].r;
                let g = v_pix[index as usize].g;
                let b = v_pix[index as usize].b;
                let a = v_pix[index as usize].a;

                // o 0 o 255
                v_pix[index as usize].r = (round(r as f32 / 255.0) * 255.0) as u8;
                v_pix[index as usize].g = (round(g as f32 / 255.0) * 255.0) as u8;
                v_pix[index as usize].b = (round(b as f32 / 255.0) * 255.0) as u8;
                v_pix[index as usize].a = (round(a as f32 / 255.0) * 255.0) as u8;

                //                let index = x + y * self.img.image_width;
                //                let r = self.img.image_pixels[index];
                //                let g = self.img.image_pixels[index + 1];
                //                let b = self.img.image_pixels[index + 2];
                //                let a = self.img.image_pixels[index + 3];

                let br = brightness(Color::new(r, g, b, a));

                if br > 1.0 {
                    self.spots.push(Vector2::new(x as f32, y as f32));
                }
            }
        }

        //P5Image::update_pixels(&mut self.engine.param, rl, th, v_pix, anch, alt);

        background(&mut self.engine, d, 0);

        let total = 10;
        let mut count = 0;
        let mut attempts = 0;

        while count < total {
            let new_c = self.new_circle();

            if new_c.is_some() {
                self.circles.push(new_c.unwrap());
                count += 1;
            }
            attempts += 1;
            if attempts > 1000 {
                println!("FINISHED");
                break;
            }
        }

        for i in 0..self.circles.len() {
            if self.circles[i].growing {
                if self.circles[i].edges(&mut self.engine.param) {
                    self.circles[i].growing = false;
                } else {
                    for other in &self.circles {
                        if self.circles[i] != *other {
                            let dd = Vector3::dist_s(
                                &Vector3::new(self.circles[i].x, self.circles[i].y, 0.0),
                                &Vector3::new(other.x, other.y, 0.0),
                            );
                            if dd < self.circles[i].r + other.r {
                                self.circles[i].growing = false;
                                break;
                            }
                        }
                    }
                }
            }
            self.circles[i].show(&mut self.engine, d);
            self.circles[i].grow();
        }
    }

    // Función del sketch
    pub fn new_circle(&mut self) -> Option<Circle> {
        let r = random_usize(self.spots.len());
        let spot = self.spots.get(r).unwrap();

        let x = spot.x;
        let y = spot.y;

        let mut valid = true;
        for c in &self.circles {
            let d = Vector3::dist_s(&Vector3::new(x, y, 0.0), &Vector3::new(c.x, c.y, 0.0));
            if d < c.r + 2.0 {
                valid = false;
                break;
            }
        }

        if valid {
            return Some(Circle::new(x, y));
        }
        None
    }
}
