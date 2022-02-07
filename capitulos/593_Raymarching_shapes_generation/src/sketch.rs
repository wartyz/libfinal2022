use std::convert::identity;
use std::f32::consts::PI;

use raylib::camera::*;
use raylib::prelude::*;

use libfinal::engine::Engine;
use libfinal::events::mouse_is_pressed_inicio;
use libfinal::matem::{random, random_i32, random_usize};
use libfinal::p5::color_p5::{
    background, background3, fill, fill1, fill3, no_fill, no_stroke, stroke1, stroke2,
};
use libfinal::p5::color_p5::{stroke3, Color};
use libfinal::p5::vector3d_p5::Vector4;
use libfinal::p5::vector_p5::Vector3;
use libfinal::parametros::CodigosTecla;
use libfinal::parametros::CodigosTecla::*;
use libfinal::shapes::{
    box_shape, circle, cuadrado_2d_en_3d, cubo3d, draw_linea_3d, draw_rectangulo_3d,
};
use libfinal::structure::{pop, pop3d, push, push3d};
use libfinal::transform::translate;
use libfinal::transform3d::{
    apply_matrix3d, identity4x4, imprime_matriz_4x4, pop_matrix3d, push_matrix3d, rotate_x3d,
    rotate_y3d, rotate_z3d, scale1_3d, translate3d, Matrix4x4,
};
use libfinal::utiles::*;

// Constantes del sketch

// Ancho y alto de la pantalla
pub const ANCHO: i32 = 800;
pub const ALTO: i32 = 450;

// Aqui vendrá el pseudocódigo javascript

pub struct Sketch {
    pub engine: Engine,
    //angulo_camara: f32,
    //distancia_camara: f32,
    // Variables globales del scketch
    view_eye_loc: i32,
    view_center_loc: i32,
    run_time_loc: i32,
    resolution_loc: i32,
    shader: Shader,
    camara: Camera3D,
    run_time: f32,
    resolution: [f32; 2],
}

impl Sketch {
    pub fn new(rl: &mut RaylibHandle, th: &RaylibThread) -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);

        let mut camara = Camera3D::perspective(
            raylib::prelude::Vector3::new(2.5, 2.5, 3.0), // posición
            raylib::prelude::Vector3::new(0.0, 0.0, 0.7), // objetivo
            raylib::prelude::Vector3::new(0.0, 1.0, 0.0), // up
            65.0,
        );

        rl.set_camera_mode(camara, CameraMode::CAMERA_FREE);

        // let mut shader = rl
        //     .load_shader(th, None, Some("h:/Proyectos_rust/pruebasB_the_coding_train_raylib3/libfinal2022/capitulos/593_Raymarching_shapes_generation/src/shaders/raymarching.fs"))
        //     .unwrap();

        let mut shader = rl
            .load_shader(
                th,
                None,
                Some("capitulos/593_Raymarching_shapes_generation/src/shaders/raymarching.fs"),
            )
            .unwrap();

        // Get shader locations for required uniforms
        let view_eye_loc: i32 = shader.get_shader_location("viewEye");
        let view_center_loc: i32 = shader.get_shader_location("viewCenter");
        let run_time_loc: i32 = shader.get_shader_location("runTime");
        let resolution_loc: i32 = shader.get_shader_location("resolution");

        let mut resolution: [f32; 2] = [ANCHO as f32, ALTO as f32];
        shader.set_shader_value(resolution_loc, resolution);

        let mut run_time: f32 = 0.0;

        rl.set_target_fps(60); // Set our game to run at 60 frames-per-second

        //----------------------------------------------------------------------------------

        // let distancia_camara = Vector4::dist_s(
        //     &Vector4::default(),
        //     &Vector4::new(
        //         engine.param.camara.posx,
        //         engine.param.camara.posy,
        //         engine.param.camara.posz,
        //         1.0,
        //     ),
        // );

        Sketch {
            engine,
            //angulo_camara: 0.0,
            //distancia_camara,
            view_eye_loc,
            view_center_loc,
            run_time_loc,
            resolution_loc,
            shader,
            camara,
            run_time,
            resolution,
        }
    }

    // Función setup() de javascript
    pub fn setup(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {
        //self.distancia_camara = 420.0;
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) -> bool {
        if !self.engine.update(rl, th) {
            return false;
        }
        true
    }

    // Función draw() de javascript
    pub fn draw(&mut self, rl: &mut RaylibHandle, th: &RaylibThread) {
        // Obtenemos d aqui

        // background(&mut self.engine, &mut d, 51);

        // let p = &mut self.engine.param;
        //
        // camara_fija(p);
        // dibuja_ejes(&mut d, p, 120.0);
        // stroke1(255.0, p);
        // no_fill(p);

        //--------------------------------------------------------------------------------------
        // Check if screen is resized
        //----------------------------------------------------------------------------------
        if rl.is_window_resized() {
            let screen_width: i32 = rl.get_screen_width();
            let screen_height: i32 = rl.get_screen_height();
            self.resolution = [screen_width as f32, screen_height as f32];
            self.shader
                .set_shader_value(self.resolution_loc, self.resolution);
        }
        // Update
        //----------------------------------------------------------------------------------
        rl.update_camera(&mut self.camara); // Update camera

        let camera_pos: [f32; 3] = [
            self.camara.position.x,
            self.camara.position.y,
            self.camara.position.z,
        ];
        let camera_target: [f32; 3] = [
            self.camara.target.x,
            self.camara.target.y,
            self.camara.target.z,
        ];

        let delta_time = rl.get_frame_time();
        self.run_time += delta_time;

        // Set shader required uniform values
        self.shader.set_shader_value(self.view_eye_loc, camera_pos);
        self.shader
            .set_shader_value(self.view_center_loc, camera_target);
        self.shader
            .set_shader_value(self.run_time_loc, self.run_time);
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(th);

        d.clear_background(raylib::prelude::Color::RAYWHITE);

        // We only draw a white full-screen rectangle,
        // frame is generated in shader using raymarching
        let mut d = d.begin_shader_mode(&self.shader);
        d.draw_rectangle(0, 0, ANCHO, ALTO, raylib::prelude::Color::WHITE);
        //shader.end_shader_mode(&shader);

        d.draw_text(
            "Programa de Shader.",
            ANCHO - 280,
            ALTO - 20,
            10,
            raylib::prelude::Color::BLACK,
        );

        //rl.end_drawing();
        //----------------------------------------------------------------------------------

        // translate(ANCHO as f32 / 2.0, ALTO as f32 / 2.0, p);
        // rotate_x3d(self.a, p);
        // rotate_y3d(self.a * 0.4, p);
        // rotate_z3d(self.a * 0.1, p);
        //
        // //cubo3d(d, p, Vector4::default(), 200.0);
        // for boxy in &mut self.sponge {
        //     boxy.show(d, p);
        // }
        //
        // self.a += 0.01;

        //fill1(255.0, &mut self.engine.param);
    }

    pub fn mouse_pressed(&mut self) {
        dbg!("mouse pressed");
    }

    pub fn key_pressed(&mut self) {
        // if self.engine.param.key == CodigosTecla::SPACE {
        //     println!("En key pressed");
        //     self.started = true;
        //     //self.mover.start();
        // }

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
