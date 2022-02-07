use raylib::prelude::*;

use libfinal::ambient::draw_fps_fi;
use libfinal::constantes::*;
use libfinal::engine::Engine;

use libfinal::image::*;
use libfinal::matem::*;
use libfinal::p5::color_p5::*;
use libfinal::p5::vector_p5::Vector3;
use libfinal::parametros::*;
use libfinal::shapes::*;
use libfinal::typography::{text, text_size};

use crate::snowflake::Snowflake;

// Constantes del sketch

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 400;
pub const ALTO: i32 = 400;

// Aqui vendrá el pseudocódigo javascript

pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    //image: Image,
    snow: Vec<Snowflake>,
    gravity: Vector3,

    spritesheet: Image,
    textures: Vec<Image>,
}

impl Sketch {
    pub fn new(rl: &mut RaylibHandle, th: &RaylibThread) -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);

        Sketch {
            engine,
            //image: ImageLf::load_image_fi("resources/imagenes/flakes32.png"),
            snow: vec![],
            gravity: Vector3::new(0.0, 0.03, 0.0),

            spritesheet: ImageLf::load_image_fi("resources/imagenes/flakes32.png"),
            textures: vec![],
        }
    }

    // Función setup() de javascript
    pub fn setup(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {
        //self.spritesheet.push(ImageLf::load_image_fi("resources/imagenes/flakes32.png")),

        for x in (0..self.spritesheet.width()).step_by(32) {
            for y in (0..self.spritesheet.height()).step_by(32) {
                let img = self.spritesheet.get(x as f32, y as f32, 32f32, 32f32);
                self.textures.push(img);
            }
        }

        for i in 0..300 {
            let x = random(ANCHO as f32);
            let y = random(ALTO as f32);
            let r = random_i32_range(0, self.textures.len() as i32) as usize;

            self.snow
                .push(Snowflake::new2(x, y, &self.textures[r].clone(), rl, th));
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) -> bool {
        if !self.engine.update(rl, th) {
            return false;
        }
        true
    }

    // Función draw() de javascript
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        //        {
        //            let mut d = rl.begin_drawing(&th);
        //            background(&mut self.engine, &mut d, 245); //RAYWHITE
        //        }

        //ImageLf::image(rl, th, &self.textures.image, 0.0, 0.0);

        // Ojo tenemos que ejecutar esto antes por cuestiones de prestamo
        //        let _ = rl.load_texture(&th, "resources/imagenes/flakes32.png")
        //            .expect("Error 1 en carga de texturas");
        //        let textu = rl.load_texture_from_image(th, &self.spritesheet)
        //            .expect("Error 2 en carga de texturas");
        //let t = rl.load_texture_from_image(th, im).unwrap();
        //
        //        // Creamos el único RaylibDrawHandle
        //        let mut d = rl.begin_drawing(th);

        //        background(&mut self.engine, &mut d, 0);

        //ImageLf::image(&mut d, &textu, 0.0, 0.0);
        //self.snow.push(Snowflake::new());

        //d.draw_texture(&textu, 0, 0, Color::WHITE);

        for i in 0..self.snow.len() {
            self.snow[i].apply_force(self.gravity);
            self.snow[i].update(self.gravity);

            {
                background(&mut self.engine, d, 0);
                self.snow[i].render(d, &mut self.engine);
            }
        }

        //ImageLf::image(&mut d, textu, 0.0, 0.0);

        //        for i in (0..self.snow.len() - 1).rev() {
        //            if self.snow[i].off_screen() {
        //                // En video snow.splice(i,1);
        //                self.snow.swap_remove(i);
        //            }
        //        }

        //dbg!(self.snow.len());
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
