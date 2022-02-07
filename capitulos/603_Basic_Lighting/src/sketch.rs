use std::convert::identity;
use std::f32::consts::PI;

use raylib::camera::*;
use raylib::prelude::*;

use crate::light::{Light, LightType};
use libfinal::engine::Engine;
use libfinal::events::mouse_is_pressed_inicio;
use libfinal::lights_camera::lights;
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
    model_a: Model,
    model_b: Model,
    model_c: Model,
    //view_eye_loc: i32,
    //view_center_loc: i32,
    //run_time_loc: i32,
    //resolution_loc: i32,
    shader: WeakShader,
    camara: Camera3D,
    //run_time: f32,
    //resolution: [f32; 2],
    texture: Texture2D,
    lights: [Light; 4],
    angle: f32,
}

impl Sketch {
    pub fn new(rl: &mut RaylibHandle, th: &RaylibThread) -> Sketch {
        let engine = Engine::new(ANCHO as f32, ALTO as f32);

        let mut camara = Camera3D::perspective(
            raylib::prelude::Vector3::new(2.0, 2.0, 6.0), // posición
            raylib::prelude::Vector3::new(0.0, 0.5, 0.0), // objetivo
            raylib::prelude::Vector3::new(0.0, 1.0, 0.0), // up
            45.0,
        );
        let mut model_a;
        let mut model_b;
        let mut model_c;

        unsafe {
            // Carga modelos
            model_a = rl
                .load_model_from_mesh(th, Mesh::gen_mesh_cube(th, 1.0, 1.0, 1.0).make_weak())
                .unwrap();
            model_b = rl
                .load_model_from_mesh(th, Mesh::gen_mesh_torus(th, 0.4, 1.0, 16, 32).make_weak())
                .unwrap();
            model_c = rl
                .load_model_from_mesh(th, Mesh::gen_mesh_sphere(th, 0.5, 32, 32).make_weak())
                .unwrap();
        }

        // Carga textura de modelos
        let texture = rl
            .load_texture(
                th,
                "capitulos/603_Basic_Lighting/texturas/texel_checker.png",
            )
            .unwrap();

        // Asigna textura a material por defecto de los tres modelos
        model_a.materials_mut()[0].set_material_texture(
            raylib::consts::MaterialMapIndex::MATERIAL_MAP_ALBEDO,
            &texture,
        );
        model_b.materials_mut()[0].set_material_texture(
            raylib::consts::MaterialMapIndex::MATERIAL_MAP_ALBEDO,
            &texture,
        );
        model_c.materials_mut()[0].set_material_texture(
            raylib::consts::MaterialMapIndex::MATERIAL_MAP_ALBEDO,
            &texture,
        );

        // Carga shader
        let mut shader = unsafe {
            rl.load_shader(
                th,
                Some(&format!(
                    "capitulos/603_Basic_Lighting/shaders/base_lighting.vs"
                )),
                Some(&format!("capitulos/603_Basic_Lighting/shaders/lighting.fs")),
            )
            .unwrap()
            .make_weak()
        };

        // Obtenemos las localizaciones de variables de shader
        // Matriz Model del VS
        shader.locs_mut()[raylib::consts::ShaderLocationIndex::SHADER_LOC_MATRIX_MODEL as usize] =
            shader.get_shader_location("matModel");
        // Parece ser que es el vector "View"
        shader.locs_mut()[raylib::consts::ShaderLocationIndex::SHADER_LOC_VECTOR_VIEW as usize] =
            shader.get_shader_location("viewPos");
        // dbg!(
        //     shader.locs_mut()
        //         [raylib::consts::ShaderLocationIndex::SHADER_LOC_MATRIX_MODEL as usize]
        // );
        // Creamos los valores de la luz ambiente
        let ambient_loc = shader.get_shader_location("ambient");
        shader.set_shader_value(ambient_loc, Vector4::new(0.2, 0.2, 0.2, 1.0).to_v4_raylib());

        let mut angle: f32 = 6.292; // Angulo de la camara

        // Todos los modelos usan el mismo shader
        model_a.materials_mut()[0].shader = *shader;
        model_b.materials_mut()[0].shader = *shader;
        model_c.materials_mut()[0].shader = *shader;

        // Creamos 4 puntos de luz

        let l0 = Light::new(
            LightType::LightPoint,
            Vector4::new(4.0, 2.0, 4.0, 1.0),
            Vector4::default(),
            Color::WHITE,
            &mut shader,
        );

        let l1 = Light::new(
            LightType::LightPoint,
            Vector4::new(4.0, 2.0, 4.0, 1.0),
            Vector4::default(),
            Color::RED,
            &mut shader,
        );

        let l2 = Light::new(
            LightType::LightPoint,
            Vector4::new(0.0, 4.0, 2.0, 1.0),
            Vector4::default(),
            Color::GREEN,
            &mut shader,
        );

        let l3 = Light::new(
            LightType::LightPoint,
            Vector4::new(0.0, 4.0, 2.0, 1.0),
            Vector4::default(),
            Color::BLUE,
            &mut shader,
        );

        let mut lights = [l0, l1, l2, l3];

        // Ponemos una camara orbital (gira alrededor del objetivo)
        rl.set_camera_mode(camara, CameraMode::CAMERA_ORBITAL);

        rl.set_target_fps(60); // Hacemos que funcione a 60 frames por segundo
                               //--------------------------------------------------------------------------------------

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
            model_a,
            model_b,
            model_c,
            shader,
            camara,
            texture,
            lights,
            angle,
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
        //background(&mut self.engine, &mut d, 51);
        // capture texture ???????????
        let _ = self.texture;
        rl.update_camera(&mut self.camara); // Actualiza la camara

        // Pone las luces en diferentes orbitas
        self.angle -= 0.02;
        let a = self.angle;
        self.lights[0].position.x = a.cos() * 4.0;
        self.lights[0].position.z = a.sin() * 4.0;
        self.lights[1].position.x = (-a * 0.6).cos() * 4.0;
        self.lights[1].position.z = (-a * 0.6).sin() * 4.0;
        self.lights[2].position.y = (a * 0.2).cos() * 4.0;
        self.lights[2].position.z = (a * 0.2).sin() * 4.0;
        self.lights[3].position.y = (-a * 0.35).cos() * 4.0;
        self.lights[3].position.z = (-a * 0.35).sin() * 4.0;

        // Rota el Torus
        self.model_a
            .set_transform(&(*self.model_a.transform() * Matrix::rotate_x(-0.025)));
        self.model_a
            .set_transform(&(*self.model_a.transform() * Matrix::rotate_z(0.012)));

        // Actualizamos el shader con la posición de vista de la cámara
        let camera_pos = Vector4::new(
            self.camara.position.x,
            self.camara.position.y,
            self.camara.position.z,
            1.0,
        );
        let loc = self.shader.locs_mut()
            [raylib::consts::ShaderLocationIndex::SHADER_LOC_VECTOR_VIEW as usize];
        self.shader.set_shader_value(loc, camera_pos.to_v3_raylib());
        //----------------------------------------------------------------------------------
        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(th);

        d.clear_background(raylib::prelude::Color::RAYWHITE);

        // Ahora usaremos d en modo 3D
        let mut d = d.begin_mode3D(&self.camara);

        // Dibuja los tres modelos
        d.draw_model(
            &self.model_a,
            Vector4::new(0.0, 0.0, 0.0, 1.0).to_v3_raylib(),
            1.0,
            (Color::WHITE).to_color_raylib(),
        );
        d.draw_model(
            &self.model_b,
            Vector4::new(-1.6, 0.0, 0.0, 1.0).to_v3_raylib(),
            1.0,
            (Color::WHITE).to_color_raylib(),
        );
        d.draw_model(
            &self.model_c,
            Vector4::new(1.6, 0.0, 0.0, 1.0).to_v3_raylib(),
            1.0,
            (Color::WHITE).to_color_raylib(),
        );

        // Dibujamos señalizadores de donde estan las luces
        if self.lights[0].enabled {
            d.draw_sphere_ex(
                self.lights[0].position.to_v3_raylib(),
                0.2,
                8,
                8,
                (Color::WHITE).to_color_raylib(),
            )
        }

        if self.lights[1].enabled {
            d.draw_sphere_ex(
                self.lights[1].position.to_v3_raylib(),
                0.2,
                8,
                8,
                (Color::RED).to_color_raylib(),
            )
        }

        if self.lights[2].enabled {
            d.draw_sphere_ex(
                self.lights[2].position.to_v3_raylib(),
                0.2,
                8,
                8,
                (Color::GREEN).to_color_raylib(),
            )
        }
        if self.lights[3].enabled {
            d.draw_sphere_ex(
                self.lights[3].position.to_v3_raylib(),
                0.2,
                8,
                8,
                (Color::BLUE).to_color_raylib(),
            )
        }
    }

    pub fn mouse_pressed(&mut self) {
        dbg!("mouse pressed");
    }

    pub fn key_pressed(&mut self) {
        if self.engine.param.key == CodigosTecla::W {
            //self.lights[0].enabled = !self.lights[0].enabled;
            self.lights[0].enabled = false;
        }
        if self.engine.param.key == CodigosTecla::R {
            //self.lights[1].enabled = !self.lights[1].enabled;
            self.lights[1].enabled = false;
        }
        if self.engine.param.key == CodigosTecla::G {
            //self.lights[2].enabled = !self.lights[2].enabled;
            self.lights[2].enabled = false;
        }
        if self.engine.param.key == CodigosTecla::B {
            //self.lights[3].enabled = !self.lights[3].enabled;
            self.lights[3].enabled = false;
        }

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
