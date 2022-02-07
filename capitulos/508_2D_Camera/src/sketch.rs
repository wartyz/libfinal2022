use raylib::prelude::*;

use libfinal::constantes::*;
use libfinal::engine::Engine;
use libfinal::matem::*;
use libfinal::p5::color_p5::{background, fill, no_fill, stroke, Color};
use libfinal::parametros::*;
use libfinal::shapes::*;
use libfinal::typography::{text, text_size};

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
    player: Rectangle,
    buildings: Vec<Rectangle>,
    build_colors: Vec<Color>,
    camera: Camera2D,
}

impl Sketch {
    pub fn new() -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);
        let player = Rectangle::new(400.0, 280.0, 40.0, 40.0);

        let mut buildings: Vec<Rectangle> = vec![];
        let mut build_colors = vec![];

        let mut spacing = 0.0;

        for _ in 0..MAX_BUILDINGS {
            let anch = random_range(50.0, 200.0);
            let alt = random_range(100.0, 800.0);
            let y = ANCHO as f32 - 130.0 - alt;
            let x = -6000.0 + spacing;

            let rt = Rectangle::new(x, y, anch, alt);
            buildings.push(rt);

            spacing += rt.width;

            let r = random_u8_range(200, 240);
            let g = random_u8_range(200, 240);
            let b = random_u8_range(200, 250);
            let cl = Color::new(r, g, b, 255);
            build_colors.push(cl);
        }

        let camera = Camera2D {
            target: Vector2::new(player.x + 20.0, player.y + 20.0),
            // offset: Vector2::new(player.x, player.y),
            offset: Vector2::new(ANCHO as f32 / 2.0, ALTO as f32 / 2.0),
            rotation: 0.0,
            zoom: 1.0,
        };

        Sketch {
            engine,
            player,
            buildings,
            build_colors,
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
        // Player movement
        if self.engine.param.key == CodigosTecla::RightArrow {
            self.player.x += 2.0
        } else if self.engine.param.key == CodigosTecla::LeftArrow {
            self.player.x -= 2.0;
        }

        // Camera target follows player
        self.camera.target = Vector2::new(self.player.x + 20.0, self.player.y + 20.0);

        // Camera rotation controls
        if self.engine.param.key == CodigosTecla::A {
            self.camera.rotation -= 1.0;
        } else if self.engine.param.key == CodigosTecla::S {
            self.camera.rotation += 1.0;
        }

        // Limit camera rotation to 80 degrees (-40 to 40)
        if self.camera.rotation > 40.0 {
            self.camera.rotation = 40.0;
        } else if self.camera.rotation < -40.0 {
            self.camera.rotation = -40.0;
        }

        // Camera zoom controls
        //camera.zoom += ((float)GetMouseWheelMove() * 0.05);
        self.camera.zoom += self.engine.param.mouse_rueda as f32 * 0.05;

        if self.camera.zoom > 3.0 {
            self.camera.zoom = 3.0;
        } else if self.camera.zoom < 0.1 {
            self.camera.zoom = 0.1;
        }

        // Camera reset (zoom and rotation)
        if self.engine.param.key == CodigosTecla::R {
            self.camera.zoom = 1.0;
            self.camera.rotation = 0.0;
        }

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

    // Función draw() de javascript
    pub fn draw(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {
        let mut d = rl.begin_drawing(&th);

        background(&mut self.engine, &mut d, 245); //RAYWHITE
        let ancho = self.engine.param.ancho;
        let alto = self.engine.param.alto;
        stroke(Color::new(0, 0, 0, 255), &mut self.engine.param);

        {
            // Modo 2D
            let mut d2 = d.begin_mode2D(self.camera);

            fill(Color::new(80, 80, 80, 255), &mut self.engine.param); //DARKGREY
            rect2d(
                &mut d2,
                &mut self.engine.param,
                -6000.0,
                320.0,
                13000.0,
                8000.0,
            );

            for i in 0..MAX_BUILDINGS {
                //DrawRectangleRec(buildings[i], buildColors[i]);
                fill(self.build_colors[i], &mut self.engine.param);
                let rr = self.buildings[i];
                rect2d(
                    &mut d2,
                    &mut self.engine.param,
                    rr.x,
                    rr.y,
                    rr.width,
                    rr.height,
                );
            }

            // Player
            fill(Color::new(255, 0, 0, 255), &mut self.engine.param); //RED
            rect2d(
                &mut d2,
                &mut self.engine.param,
                self.player.x,
                self.player.y,
                self.player.width,
                self.player.height,
            );

            fill(Color::new(0, 255, 0, 255), &mut self.engine.param); //GREEN
            line2d(
                &mut d2,
                &mut self.engine.param,
                self.camera.target.x,
                -alto * 10.0,
                self.camera.target.x,
                alto * 10.0,
            );

            line2d(
                &mut d2,
                &mut self.engine.param,
                -ancho * 10.0,
                self.camera.target.y,
                ancho * 10.0,
                self.camera.target.y,
            );
        }

        fill(Color::new(255, 0, 0, 255), &mut self.engine.param); //RED
        text_size(&mut self.engine.param, 20);
        text(&mut self.engine.param, &mut d, "SCREEN AREA", 640.0, 10.0);

        rect(&mut d, &mut self.engine.param, 0.0, 0.0, ancho, 5.0);
        rect(&mut d, &mut self.engine.param, 0.0, 5.0, 5.0, alto - 10.0);
        rect(
            &mut d,
            &mut self.engine.param,
            ancho - 5.0,
            5.0,
            5.0,
            alto - 10.0,
        );
        rect(&mut d, &mut self.engine.param, 0.0, alto - 5.0, ancho, 5.0);

        fill(Color::new(102, 191, 255, 255), &mut self.engine.param); //Fade(SKYBLUE, 0.5f)
        rect(&mut d, &mut self.engine.param, 10.0, 10.0, 250.0, 113.0);

        no_fill(&mut self.engine.param);
        stroke(Color::new(0, 0, 255, 255), &mut self.engine.param); //BLUE
        rect(&mut d, &mut self.engine.param, 10.0, 10.0, 250.0, 113.0);

        text_size(&mut self.engine.param, 10);
        fill(Color::new(0, 0, 0, 255), &mut self.engine.param); //BLACK
        text(
            &mut self.engine.param,
            &mut d,
            "Free 2d camera controls:",
            20.0,
            20.0,
        );

        fill(Color::new(80, 80, 80, 255), &mut self.engine.param); //DARKGRAY
        text(
            &mut self.engine.param,
            &mut d,
            "- Right/Left to move Offset",
            40.0,
            40.0,
        );
        text(
            &mut self.engine.param,
            &mut d,
            "- Mouse Wheel to Zoom in-out",
            40.0,
            60.0,
        );
        text(
            &mut self.engine.param,
            &mut d,
            "- A / S to Rotate",
            40.0,
            80.0,
        );
        text(
            &mut self.engine.param,
            &mut d,
            "- R to reset Zoom and Rotation",
            40.0,
            100.0,
        );
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
