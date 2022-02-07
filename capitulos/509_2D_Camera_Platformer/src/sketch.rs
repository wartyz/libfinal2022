use raylib::prelude::*;

use libfinal::constantes::*;
use libfinal::engine::Engine;
use libfinal::matem::{max, min};
use libfinal::p5::color_p5::{background, fill, Color};
use libfinal::parametros::*;
use libfinal::shapes::*;
use libfinal::typography::{text, text_size};

use crate::env_item::EnvItem;
use crate::player::Player;

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
    player: Player,

    camera: Camera2D,
    env_items: Vec<EnvItem>,

    camera_option: i32,
    camera_updaters_length: i32,
    camera_descriptions: Vec<String>,
}

impl Sketch {
    pub fn new() -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);
        let player = Player::new();

        let mut env_items = vec![];

        //Color LIGHTGRAY
        env_items.push(EnvItem::new(
            0.0,
            0.0,
            1000.0,
            400.0,
            false,
            Color::new(200, 200, 200, 255),
        ));
        //Color GRAY

        let c = Color::new(130, 130, 130, 255);
        env_items.push(EnvItem::new(300.0, 200.0, 400.0, 10.0, true, c));
        env_items.push(EnvItem::new(250.0, 300.0, 100.0, 10.0, true, c));
        env_items.push(EnvItem::new(650.0, 300.0, 100.0, 10.0, true, c));
        let env_items_length = env_items.len();

        let camera = Camera2D {
            target: player.position,
            offset: Vector2::new(ANCHO as f32 / 2.0, ALTO as f32 / 2.0),
            rotation: 0.0,
            zoom: 1.0,
        };
        let mut camera_descriptions = vec![];
        camera_descriptions.push(String::from("Follow player center"));
        camera_descriptions.push(String::from("Follow player center, but clamp to map edges"));
        camera_descriptions.push(String::from("Follow player center; smoothed"));
        camera_descriptions.push(String::from(
            "Follow player center horizontally; updateplayer center vertically after landing",
        ));
        camera_descriptions.push(String::from(
            "Player push camera on getting too close to screen edge",
        ));

        Sketch {
            engine,
            player,

            camera,
            env_items,
            camera_option: 0,
            camera_updaters_length: 5,

            camera_descriptions,
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
        let delta_time = rl.get_frame_time();
        self.player
            .update_player(&mut self.engine.param, &self.env_items, delta_time);

        self.camera.zoom += self.engine.param.mouse_rueda as f32 * 0.05;

        if self.camera.zoom > 3.0 {
            self.camera.zoom = 3.0;
        } else if self.camera.zoom < 0.25 {
            self.camera.zoom = 0.25;
        }

        if self.engine.param.key == CodigosTecla::R {
            self.camera.zoom = 1.0;
            self.player.position = Vector2::new(400.0, 280.0);
        }
        if self.engine.param.key == CodigosTecla::C {
            self.camera_option = (self.camera_option + 1) % self.camera_updaters_length;
        }

        match self.camera_option {
            0 => self.update_camera_center(self.engine.param.ancho, self.engine.param.alto),
            1 => self.update_camera_center_inside_map(
                rl,
                self.engine.param.ancho,
                self.engine.param.alto,
            ),
            2 => self.update_camera_center_smooth_follow(),
            3 => self.update_camera_even_out_on_landing(),
            4 => self.update_camera_player_bounds_push(),
            _ => {}
        }

        //// Player movement
        //        if self.engine.param.key == CodigosTecla::RightArrow {
        //            self.player.x += 2.0
        //        } else if self.engine.param.key == CodigosTecla::LeftArrow {
        //            self.player.x -= 2.0;
        //        }
        //
        //
        //// Camera target follows player
        //        self.camera.target = Vector2::new(self.player.x + 20.0, self.player.y + 20.0);
        //
        //// Camera rotation controls
        //        if self.engine.param.key == CodigosTecla::A {
        //            self.camera.rotation -= 1.0;
        //        } else if self.engine.param.key == CodigosTecla::S {
        //            self.camera.rotation += 1.0;
        //        }
        //
        //
        //// Limit camera rotation to 80 degrees (-40 to 40)
        //        if self.camera.rotation > 40.0 {
        //            self.camera.rotation = 40.0;
        //        } else if self.camera.rotation < -40.0 {
        //            self.camera.rotation = -40.0;
        //        }
        //
        //// Camera zoom controls
        ////camera.zoom += ((float)GetMouseWheelMove() * 0.05);
        //        self.camera.zoom += self.engine.param.mouse_rueda as f32 * 0.05;
        //
        //        if self.camera.zoom > 3.0 {
        //            self.camera.zoom = 3.0;
        //        } else if self.camera.zoom < 0.1 {
        //            self.camera.zoom = 0.1;
        //        }
        //
        //// Camera reset (zoom and rotation)
        //        if self.engine.param.key == CodigosTecla::R {
        //            self.camera.zoom = 1.0;
        //            self.camera.rotation = 0.0;
        //        }

        //        match self.engine.param.key {
        //            CodigosTecla::RightArrow => self.ball_position.x += 2.0,
        //            CodigosTecla::LeftArrow => self.ball_position.x -= 2.0,
        //            CodigosTecla::UpArrow => self.ball_position.y -= 2.0,
        //            CodigosTecla::DownArrow => self.ball_position.y += 2.0,
        //            _ => {}
        //        }
        //        self.ball_position = self.engine.param.mouse_posicion;
        //        match self.engine.param.mouse_boton {
        //            CodigosRaton::Izquierdo => self.ball_color = Color::MAROON,
        //            CodigosRaton::Medio => self.ball_color = Color::LIME,
        //            CodigosRaton::Derecho => self.ball_color = Color::DARKBLUE,
        //            _ => {}
        //        }

        if !self.engine.update(rl, th) {
            return false;
        }
        true
    }

    pub fn update_camera_center(&mut self, ancho: f32, alto: f32) {
        self.camera.offset = Vector2::new(ancho / 2.0, alto / 2.0);
        self.camera.target = self.player.position;
    }
    pub fn update_camera_center_inside_map(
        &mut self,
        rl: &mut RaylibHandle,
        ancho: f32,
        alto: f32,
    ) {
        self.camera.target = self.player.position;
        self.camera.offset = Vector2::new(ancho / 2.0, alto / 2.0);
        let mut min_x: f32 = 1000.0;
        let mut min_y: f32 = 1000.0;
        let mut max_x: f32 = -1000.0;
        let mut max_y: f32 = -1000.0;

        for ei in &self.env_items {
            min_x = min(ei.rect.x, min_x);
            max_x = max(ei.rect.x + ei.rect.width, max_x);
            min_y = min(ei.rect.y, min_y);
            max_y = max(ei.rect.y + ei.rect.height, max_y);
        }

        let maximo = rl.get_world_to_screen2D(Vector2::new(max_x, max_y), self.camera);
        let minimo = rl.get_world_to_screen2D(Vector2::new(min_x, min_y), self.camera);

        if maximo.x < ancho {
            self.camera.offset.x = ancho - (maximo.x - ancho / 2.0);
        }
        if maximo.y < alto {
            self.camera.offset.y = alto - (maximo.y - alto / 2.0);
        }
        if minimo.x > 0.0 {
            self.camera.offset.x = ancho / 2.0 - minimo.x;
        }
        if minimo.y > 0.0 {
            self.camera.offset.y = alto / 2.0 - minimo.y;
        }
    }
    pub fn update_camera_center_smooth_follow(&mut self) {}
    pub fn update_camera_even_out_on_landing(&mut self) {}
    pub fn update_camera_player_bounds_push(&mut self) {}
    // Función draw() de javascript
    pub fn draw(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {
        let mut d = rl.begin_drawing(&th);

        background(&mut self.engine, &mut d, 245); //LIGHTGRAY

        {
            // Modo 2D
            let mut d2 = d.begin_mode2D(self.camera);

            for i in 0..self.env_items.len() {
                fill(self.env_items[i].color, &mut self.engine.param);
                rect2d(
                    &mut d2,
                    &mut self.engine.param,
                    self.env_items[i].rect.x,
                    self.env_items[i].rect.y,
                    self.env_items[i].rect.width,
                    self.env_items[i].rect.height,
                );
            }

            fill(Color::new(255, 0, 0, 255), &mut self.engine.param); //RED
            rect2d(
                &mut d2,
                &mut self.engine.param,
                self.player.position.x - 20.0,
                self.player.position.y - 40.0,
                40.0,
                40.0,
            );
        } // Fin modo 2D

        text_size(&mut self.engine.param, 10);
        fill(Color::new(0, 0, 0, 255), &mut self.engine.param); //BLACK
        text(&mut self.engine.param, &mut d, "Controls:", 20.0, 20.0);

        fill(Color::new(80, 80, 80, 255), &mut self.engine.param); //DARKGRAY
        text(
            &mut self.engine.param,
            &mut d,
            "- Right/Left to move",
            40.0,
            40.0,
        );
        text(
            &mut self.engine.param,
            &mut d,
            "- Space to jump",
            40.0,
            60.0,
        );
        text(
            &mut self.engine.param,
            &mut d,
            "- Mouse Wheel to Zoom in-out, R to reset zoom",
            40.0,
            80.0,
        );
        text(
            &mut self.engine.param,
            &mut d,
            "- C to change camera mode",
            40.0,
            100.0,
        );
        fill(Color::new(0, 0, 0, 255), &mut self.engine.param); //BLACK
        text(
            &mut self.engine.param,
            &mut d,
            "Current camera mode:",
            20.0,
            120.0,
        );
        fill(Color::new(80, 80, 80, 255), &mut self.engine.param); //DARKGRAY

        let tt = &self.camera_descriptions[self.camera_option as usize];
        text(&mut self.engine.param, &mut d, &tt, 40.0, 140.0);

        //        let ancho = self.engine.param.ancho;
        //        let alto = self.engine.param.alto;
        //        stroke(Color::new(0, 0, 0, 255), &mut self.engine.param);
        //
        //        { // Modo 2D
        //            let mut d2 = d.begin_mode2D(self.camera);
        //
        //            fill(Color::new(80, 80, 80, 255), &mut self.engine.param); //DARKGREY
        //            rect2d(&mut d2, &mut self.engine.param, -6000.0, 320.0, 13000.0, 8000.0);
        //
        //            for i in 0..MAX_BUILDINGS {
        ////DrawRectangleRec(buildings[i], buildColors[i]);
        //                fill(self.build_colors[i], &mut self.engine.param);
        //                let rr = self.buildings[i];
        //                rect2d(&mut d2, &mut self.engine.param, rr.x, rr.y, rr.width, rr.height);
        //            }
        //
        //// Player
        //            fill(Color::new(255, 0, 0, 255), &mut self.engine.param); //RED
        //            rect2d(&mut d2, &mut self.engine.param, self.player.x, self.player.y,
        //                   self.player.width, self.player.height);
        //
        //            fill(Color::new(0, 255, 0, 255), &mut self.engine.param); //GREEN
        //            line2d(&mut d2, &mut self.engine.param, self.camera.target.x, -alto * 10.0,
        //                   self.camera.target.x, alto * 10.0);
        //
        //            line2d(&mut d2, &mut self.engine.param, -ancho * 10.0, self.camera.target.y,
        //                   ancho * 10.0, self.camera.target.y);
        //        }
        //
        //
        //        fill(Color::new(255, 0, 0, 255), &mut self.engine.param); //RED
        //        text_size(&mut self.engine.param, 20);
        //        text(&mut self.engine.param, &mut d, "SCREEN AREA", 640.0, 10.0);
        //
        //        rect(&mut d, &mut self.engine.param, 0.0, 0.0, ancho, 5.0);
        //        rect(&mut d, &mut self.engine.param, 0.0, 5.0, 5.0, alto - 10.0);
        //        rect(&mut d, &mut self.engine.param, ancho - 5.0, 5.0, 5.0, alto - 10.0);
        //        rect(&mut d, &mut self.engine.param, 0.0, alto - 5.0, ancho, 5.0);
        //
        //        fill(Color::new(102, 191, 255, 255), &mut self.engine.param); //Fade(SKYBLUE, 0.5f)
        //        rect(&mut d, &mut self.engine.param, 10.0, 10.0, 250.0, 113.0);
        //
        //        no_fill(&mut self.engine.param);
        //        stroke(Color::new(0, 0, 255, 255), &mut self.engine.param); //BLUE
        //        rect(&mut d, &mut self.engine.param, 10.0, 10.0, 250.0, 113.0);
        //
        //        text_size(&mut self.engine.param, 10);
        //        fill(Color::new(0, 0, 0, 255), &mut self.engine.param); //BLACK
        //        text(&mut self.engine.param, &mut d, "Free 2d camera controls:", 20.0, 20.0);
        //
        //        fill(Color::new(80, 80, 80, 255), &mut self.engine.param); //DARKGRAY
        //        text(&mut self.engine.param, &mut d, "- Right/Left to move Offset", 40.0, 40.0);
        //        text(&mut self.engine.param, &mut d, "- Mouse Wheel to Zoom in-out", 40.0, 60.0);
        //        text(&mut self.engine.param, &mut d, "- A / S to Rotate", 40.0, 80.0);
        //        text(&mut self.engine.param, &mut d, "- R to reset Zoom and Rotation", 40.0, 100.0);
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
