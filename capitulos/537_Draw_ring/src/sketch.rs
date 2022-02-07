use raylib::prelude::*;

use libfinal::ambient::draw_fps_fi;
use libfinal::constantes::*;
use libfinal::engine::Engine;
use libfinal::matem::min;
use libfinal::p5::color_p5::{background, fill, stroke, Color};
use libfinal::parametros::*;
use libfinal::shapes::*;
use libfinal::typography::{text, text_size};

//use crate::env_item::EnvItem;
//use crate::player::Player;

// Constantes del sketch
// Ancho y alto de la pantalla
pub const ANCHO: i32 = 800;
pub const ALTO: i32 = 600;

pub const COLS: usize = ANCHO as usize;
pub const ROWS: usize = ALTO as usize;

// Aqui vendrá el pseudocódigo javascript
pub const MAX_BUILDINGS: usize = 100;

pub struct Sketch {
    pub engine: Engine,

    // Variables globales del scketch
    center: Vector2,
    inner_radius: f32,
    outer_radius: f32,

    start_angle: f32,
    end_angle: f32,
    segments: i32,

    draw_ring: bool,
    draw_ring_lines: bool,
    draw_circle_lines: bool,
}

impl Sketch {
    pub fn new() -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);

        let center = Vector2::new((ANCHO as f32 - 300.0) / 2.0, ALTO as f32 / 2.0);
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

        Sketch {
            engine,
            center,
            inner_radius: 80.0,
            outer_radius: 190.0,

            start_angle: 0.0,
            end_angle: 360.0,
            segments: 0,

            draw_ring: true,
            draw_ring_lines: false,
            draw_circle_lines: false,
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

        stroke(Color::new(200, 200, 200, 255), &mut self.engine.param); //LIGHTRAY
        let alt = self.engine.param.alto;
        let anch = self.engine.param.ancho;
        line(&mut d, &mut self.engine.param, 500.0, 0.0, 500.0, alt);

        fill(Color::new(100, 100, 100, 255), &mut self.engine.param);
        rect(
            &mut d,
            &mut self.engine.param,
            500.0,
            0.0,
            anch - 500.0,
            alt,
        );

        if self.draw_ring {
            d.draw_ring(
                self.center,
                self.inner_radius,
                self.outer_radius,
                self.start_angle,
                self.end_angle,
                self.segments,
                raylib::color::Color::fade(&raylib::color::Color::MAROON, 0.3),
            );
        }
        if self.draw_ring_lines {
            d.draw_ring_lines(
                self.center,
                self.inner_radius,
                self.outer_radius,
                self.start_angle,
                self.end_angle,
                self.segments,
                raylib::color::Color::fade(&raylib::color::Color::BLACK, 0.4),
            );
        }
        if self.draw_circle_lines {
            d.draw_circle_sector_lines(
                self.center,
                self.outer_radius,
                self.start_angle,
                self.end_angle,
                self.segments,
                raylib::color::Color::fade(&raylib::color::Color::BLACK, 0.4),
            );
        }

        // Draw GUI controls
        //------------------------------------------------------------------------------
        self.start_angle = d.gui_slider_bar(
            Rectangle::new(600.0, 40.0, 120.0, 20.0),
            Some(rstr!("StarAngle")),
            None,
            self.start_angle as f32,
            -450.0,
            450.0,
        );

        self.end_angle = d.gui_slider_bar(
            Rectangle::new(600.0, 70.0, 120.0, 20.0),
            Some(&rstr!("EndAngle")),
            None,
            self.end_angle as f32,
            -450.0,
            450.0,
        );

        self.inner_radius = d.gui_slider_bar(
            Rectangle::new(600.0, 140.0, 120.0, 20.0),
            Some(&rstr!("InnerRadius")),
            None,
            self.inner_radius,
            0.0,
            100.0,
        );

        self.outer_radius = d.gui_slider_bar(
            Rectangle::new(600.0, 170.0, 120.0, 20.0),
            Some(&rstr!("OuterRadius")),
            None,
            self.outer_radius,
            0.0,
            200.0,
        );

        self.segments = d.gui_slider_bar(
            Rectangle::new(600.0, 240.0, 120.0, 20.0),
            Some(&rstr!("Segments")),
            None,
            self.segments as f32,
            0.0,
            100.0,
        ) as i32;

        self.draw_ring = d.gui_check_box(
            Rectangle::new(600.0, 320.0, 20.0, 20.0),
            Some(&rstr!("Draw Ring")),
            self.draw_ring,
        );

        self.draw_ring_lines = d.gui_check_box(
            Rectangle::new(600.0, 350.0, 20.0, 20.0),
            Some(&rstr!("Draw RingLines")),
            self.draw_ring_lines,
        );

        self.draw_circle_lines = d.gui_check_box(
            Rectangle::new(600.0, 380.0, 20.0, 20.0),
            Some(&rstr!("Draw CircleLines")),
            self.draw_circle_lines,
        );

        //------------------------------------------------------------------------------
        fill(Color::new(80, 80, 80, 255), &mut self.engine.param); //DARKGREY

        text(
            &mut self.engine.param,
            &mut d,
            "MANUAL : AUTO",
            600.0,
            270.0,
        );

        //            DrawText(TextFormat("MODE: %s", (segments >= 4)? "MANUAL" : "AUTO"), 600, 270, 10, (segments >= 4)? MAROON : DARKGRAY);
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
