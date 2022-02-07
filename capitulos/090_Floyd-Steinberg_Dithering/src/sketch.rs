//https://www.youtube.com/watch?v=0L2n8Tg2FwI     9:01

use raylib::prelude::*;

use libfinal::ambient::draw_fps_fi;
use libfinal::constantes::*;
use libfinal::engine::Engine;
use libfinal::matem::round;
use libfinal::p5::color_p5::Color;
use libfinal::p5::image_p5::P5Image;
use libfinal::parametros::Filtros::Gray;
use libfinal::parametros::*;
use libfinal::shapes::*;
use libfinal::typography::{text, text_size};

// Constantes del sketch

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 1024;
pub const ALTO: i32 = 512;

pub const COLS: usize = ANCHO as usize;
pub const ROWS: usize = ALTO as usize;

// Aqui vendrá el pseudocódigo javascript
pub const MAX_BUILDINGS: usize = 100;

pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    kitten: P5Image,
    // parrots: Image,
    //texture: Option<Texture2D>,
}

impl Sketch {
    pub fn new(rl: &mut RaylibHandle, th: &RaylibThread) -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);
        // Load image in CPU memory (RAM)
        let mut kitten = P5Image::load_image_fi("resources/imagenes/kitten.png");
        kitten.filter(Gray);

        //        // Crop an image piece
        //        cat.crop(Rectangle::new(100.0, 10.0, 280.0, 380.0));
        //        // Flip cropped image horizontally
        //        cat.flip_horizontal();
        //        // Resize flipped-cropped image
        //        cat.resize(150, 200);
        //
        //        // Load image in CPU memory (RAM)
        //        let mut parrots = P5Image::load_image_fi("resources/imagenes/parrots.png");

        Sketch { engine, kitten }
    }

    // Función setup() de javascript
    pub fn setup(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {
        //let mut d = rl.begin_drawing(th);
        self.kitten.image(0, 0, rl, th);

        // Draw one image over the other with a scaling of 1.5f
        //        self.parrots.draw(&self.cat,
        //                          Rectangle::new(0.0, 0.0, self.cat.width() as f32,
        //                                         self.cat.height() as f32),
        //                          Rectangle::new(30.0, 40.0, self.cat.width() as f32 * 1.5,
        //                                         self.cat.height() as f32 * 1.5),
        //                          Color::new(255, 255, 255, 255));

        //        // Crop resulting image
        //        self.parrots.crop(Rectangle::new(0.0, 50.0, self.parrots.width() as f32,
        //                                         self.parrots.height() as f32 - 10.0));
        //
        //
        //        // Draw on the image with a few image draw methods
        //
        //        self.parrots.draw_pixel(10, 10, Color::RAYWHITE);
        //        self.parrots.draw_circle(10, 10, 5, Color::RAYWHITE);
        //        self.parrots.draw_rectangle(5, 20, 10, 10, Color::RAYWHITE);
        //
        //        let font = rl.load_font(th, "resources/custom_jupiter_crash.png").expect("error font");
        //        self.parrots.draw_text_ex(Vector2::new(300.0, 230.0), &font, "PARROTS & CATS",
        //                                  12.0, -2.0, Color::WHITE);
        //
        //        let t = rl.load_texture_from_image(&th, &self.parrots).expect("error textura");
        //        self.texture = Some(t);
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) -> bool {
        if !self.engine.update(rl, th) {
            return false;
        }
        true
    }
    pub fn index(&mut self, x: i32, y: i32) -> usize {
        let anch = self.kitten.width;
        (x + y * anch) as usize
    }

    // Función draw() de javascript
    pub fn draw(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {
        // creo que esto hace de loadpixels()
        //let mut v_pix = self.kitten.get_image_data();
        // rellena el arreglo interno pixels de kitten
        self.kitten.load_pixels(&mut self.engine.param, rl, th);

        for y in 0..self.kitten.height - 1 {
            for x in 1..self.kitten.width - 1 {
                let indice = self.index(x, y);
                let pix = self.kitten.pixels[indice];

                let mut old_r = pix.r;
                let mut old_g = pix.g;
                let mut old_b = pix.b;

                let factor = 1.0;

                // o 0 o 255
                let new_r = (round(factor * old_r as f32 / 255.0) * 255.0 / factor) as u8;
                let new_g = (round(factor * old_g as f32 / 255.0) * 255.0 / factor) as u8;
                let new_b = (round(factor * old_b as f32 / 255.0) * 255.0 / factor) as u8;

                self.kitten.pixels[indice] = raylib::color::Color::new(new_r, new_g, new_b, 255);

                let err_r = old_r as f32 - new_r as f32;
                let err_g = old_g as f32 - new_g as f32;
                let err_b = old_b as f32 - new_b as f32;

                // 1
                let ind = self.index(x + 1, y);
                let c = self.kitten.pixels[ind];
                let mut r = c.r as f32;
                let mut g = c.g as f32;
                let mut b = c.b as f32;

                let rr = (r + err_r * 7.0 / 16.0) as u8;
                let gg = (g + err_g * 7.0 / 16.0) as u8;
                let bb = (b + err_b * 7.0 / 16.0) as u8;

                self.kitten.pixels[ind] = raylib::color::Color::new(rr, gg, bb, 255);

                //2
                let ind = self.index(x - 1, y + 1);
                let c = self.kitten.pixels[ind];
                r = c.r as f32;
                g = c.g as f32;
                b = c.b as f32;

                let rr = (r + err_r * 3.0 / 16.0) as u8;
                let gg = (g + err_g * 3.0 / 16.0) as u8;
                let bb = (b + err_b * 3.0 / 16.0) as u8;

                self.kitten.pixels[ind] = raylib::color::Color::new(rr, gg, bb, 255);

                //3
                let ind = self.index(x, y + 1);
                let c = self.kitten.pixels[ind];
                let mut r = c.r as f32;
                let mut g = c.g as f32;
                let mut b = c.b as f32;

                let rr = (r + err_r * 5.0 / 16.0) as u8;
                let gg = (g + err_g * 5.0 / 16.0) as u8;
                let bb = (b + err_b * 5.0 / 16.0) as u8;

                self.kitten.pixels[ind] = raylib::color::Color::new(rr, gg, bb, 255);

                //4
                let ind = self.index(x + 1, y + 1);
                let c = self.kitten.pixels[ind];
                let mut r = c.r as f32;
                let mut g = c.g as f32;
                let mut b = c.b as f32;

                let rr = (r + err_r * 1.0 / 16.0) as u8;
                let gg = (g + err_g * 1.0 / 16.0) as u8;
                let bb = (b + err_b * 1.0 / 16.0) as u8;

                self.kitten.pixels[ind] = raylib::color::Color::new(rr, gg, bb, 255);

                //                self.kitten.pixels[self.index(x + 1, y)] = 0;
                //                self.kitten.pixels[self.index(x - 1, y + 1)] = 0;
                //                self.kitten.pixels[self.index(x, y + 1)] = 0;
                //                self.kitten.pixels[self.index(x + 1, y + 1)] = 0;
            }
        }

        self.kitten.update_pixels(
            &mut self.engine.param,
            rl,
            th,
            self.kitten.width,
            self.kitten.height,
        );

        // Presenta imagen modificada
        self.kitten.image(512, 0, rl, th);
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
