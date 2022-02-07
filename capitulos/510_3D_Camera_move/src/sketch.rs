use raylib::prelude::*;

use libfinal::ambient::draw_fps_fi;
use libfinal::constantes::*;
use libfinal::engine::Engine;
use libfinal::matem::min;
use libfinal::p5::color_p5::{background, fill, Color};
use libfinal::p5::vector3d_p5::Vector4;
use libfinal::parametros::*;
use libfinal::shapes::*;
use libfinal::typography::{text, text_size};

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 1200;
pub const ALTO: i32 = 800;

// Constantes del sketch
pub const COLS: usize = ANCHO as usize;
pub const ROWS: usize = ALTO as usize;

// Aqui vendrá el pseudocódigo javascript
pub const MAX_BUILDINGS: usize = 100;

pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    //player: Player,
    camera: Camera3D,
    cube_position: Vector3,
    //    env_items: Vec<EnvItem>,
    //
    //    camera_option: i32,
    //    camera_updaters_length: i32,
    //    camera_descriptions: Vec<String>,
}

impl Sketch {
    pub fn new() -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);
        //        let player = Player::new();
        //
        //        let mut env_items = vec![];
        //
        //        //Color LIGHTGRAY
        //        env_items.push(EnvItem::new(0.0, 0.0, 1000.0, 400.0, false, Color::new(200, 200, 200,
        //                                                                               255)));
        //        //Color GRAY
        //
        //        let c = Color::new(130, 130, 130, 255);
        //        env_items.push(EnvItem::new(300.0, 200.0, 400.0, 10.0, true, c));
        //        env_items.push(EnvItem::new(250.0, 300.0, 100.0, 10.0, true, c));
        //        env_items.push(EnvItem::new(650.0, 300.0, 100.0, 10.0, true, c));
        //        let env_items_length = env_items.len();

        let mut camera = Camera3D::perspective(
            Vector3::new(0.0, 10.0, 10.0), // posición
            Vector3::new(0.0, 0.0, 0.0),   // objetivo
            Vector3::new(0.0, 1.0, 0.0),   // up
            45.0,
        );

        //        let mut camera_descriptions = vec![];
        //        camera_descriptions.push(String::from("Follow player center"));
        //        camera_descriptions.push(String::from("Follow player center, but clamp to map edges"));
        //        camera_descriptions.push(String::from("Follow player center; smoothed"));
        //        camera_descriptions.push(
        //            String::from(
        //                "Follow player center horizontally; updateplayer center vertically after landing"));
        //        camera_descriptions.push(
        //            String::from("Player push camera on getting too close to screen edge"));
        let cube_position = Vector3::new(0.0, 0.0, 0.0);

        Sketch {
            engine,
            cube_position,
            camera,
        }
    }

    // Función setup() de javascript
    pub fn setup(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {
        //        //createcanvas(&mut self.engine, ANCHO, ALTO);
        //        for j in 0..self.rows {
        //            for i in 0..self.cols {
        //                self.grid[j][i] = random_usize(2);
        //            }
        //        }
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

        {
            // Modo 3D
            //let mut d3 = d.begin_mode3D(self.camera);
            fill(Color::new(255, 0, 0, 255), &mut self.engine.param); //RED
            let position1 = Vector4::new(0.0, 0.0, 0.0, 0.0);
            box_shape(&mut d, &mut self.engine.param, position1, 2.0, 2.0, 2.0);

            let position2 = Vector4::new(0.0, 0.0, 0.0, 0.0);
            fill(Color::new(190, 33, 55, 255), &mut self.engine.param); //MAROON
            box_hilos_shape(&mut d, &mut self.engine.param, position2, 2.0, 2.0, 2.0);

            //grid(&mut d3, 10.0, 1.0);
        } // Fin modo 3D

        fill(Color::new(80, 80, 80, 255), &mut self.engine.param); //DARKGRAY
        text_size(&mut self.engine.param, 20);
        text(
            &mut self.engine.param,
            &mut d,
            "Welcome to the third dimension!",
            10.0,
            40.0,
        );

        draw_fps_fi(&mut d, 10.0, 10.0);
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
