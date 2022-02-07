use libfinal::engine::Engine;
use libfinal::p5::color_p5::background;
use libfinal::p5::image_p5::P5Image;
use raylib::prelude::*;

#[warn(unused_variables)]

// Constantes del sketch
// Ancho y alto de la pantalla
pub const ANCHO: i32 = 1200;
pub const ALTO: i32 = 800;

pub const COLS: usize = ANCHO as usize;
pub const ROWS: usize = ALTO as usize;

// Aqui vendrá el pseudocódigo javascript
pub const MAX_BUILDINGS: usize = 100;

pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    cat: P5Image,
    parrots: P5Image,
    texture: Option<Texture2D>,
}

impl Sketch {
    pub fn new(rl: &mut RaylibHandle, th: &RaylibThread) -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);
        // Load image in CPU memory (RAM)
        let mut cat = P5Image::load_image_fi("resources/imagenes/cat.png");
        // Crop an image piece
        cat.img.crop(Rectangle::new(100.0, 10.0, 280.0, 380.0));
        // Flip cropped image horizontally
        cat.img.flip_horizontal();
        // Resize flipped-cropped image
        cat.img.resize(150, 200);

        // Load image in CPU memory (RAM)
        let parrots = P5Image::load_image_fi("resources/imagenes/parrots.png");

        Sketch {
            engine,
            cat,
            parrots,
            texture: None,
        }
    }

    // Función setup() de javascript
    pub fn setup(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {
        self.parrots.img.draw(
            &self.cat.img,
            Rectangle::new(
                0.0,
                0.0,
                self.cat.img.width() as f32,
                self.cat.img.height() as f32,
            ),
            Rectangle::new(
                30.0,
                40.0,
                self.cat.img.width() as f32 * 1.5,
                self.cat.img.height() as f32 * 1.5,
            ),
            raylib::color::Color::new(255, 255, 255, 255),
        );
        // Crop resulting image
        self.parrots.img.crop(Rectangle::new(
            0.0,
            50.0,
            self.parrots.img.width() as f32,
            self.parrots.img.height() as f32 - 10.0,
        ));

        // Draw on the image with a few image draw methods

        self.parrots
            .img
            .draw_pixel(10, 10, raylib::color::Color::RAYWHITE);
        self.parrots
            .img
            .draw_circle(10, 10, 5, raylib::color::Color::RAYWHITE);
        self.parrots
            .img
            .draw_rectangle(5, 20, 10, 10, raylib::color::Color::RAYWHITE);

        let font = rl
            .load_font(th, "resources/custom_jupiter_crash.png")
            .expect("error font");
        self.parrots.img.draw_text_ex(
            &font,
            "PARROTS & CATS",
            raylib::prelude::Vector2::new(12.0, 10.0),
            2.0,
            -2.0,
            raylib::color::Color::WHITE,
        );

        let t = rl
            .load_texture_from_image(&th, &self.parrots.img)
            .expect("error textura");
        self.texture = Some(t);
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) -> bool {
        if !self.engine.update(rl, th) {
            return false;
        }
        true
    }

    // Función draw() de javascript
    pub fn draw(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {
        let mut d = rl.begin_drawing(&th);

        background(&mut self.engine, &mut d, 245); //RAYWHITE

        let w = &self.texture.as_ref();
        let w = w.unwrap().width();
        let h = &self.texture.as_ref();
        let h = h.unwrap().height();

        // as_ref() convierte &Option<T> en Option<&T>
        let t: &Texture2D = &self.texture.as_ref().unwrap();

        d.draw_texture(
            t,
            self.engine.param.ancho as i32 / 2 - w / 2,
            (self.engine.param.alto as i32 / 2 - h / 2) - 40,
            raylib::color::Color::WHITE,
        )
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
