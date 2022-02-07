use libfinal::p5::color_p5::Color;
use libfinal::p5::vector3d_p5::Vector4;
use raylib::prelude::{RaylibShader, WeakShader};

static mut LIGHTS_COUNT: i32 = 0;
const MAX_LIGHTS: u32 = 4;

#[repr(i32)] // para que un enum adquiera el valor de un entero
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LightType {
    LightDirectional = 0,
    LightPoint = 1,
}

impl Default for LightType {
    fn default() -> Self {
        Self::LightDirectional
    }
}

#[derive(Debug, Default, Clone)]
pub struct Light {
    pub enabled: bool,
    pub light_type: LightType,
    pub position: Vector4,
    pub target: Vector4,
    pub color: Color,
    pub enabled_loc: i32,
    pub type_loc: i32,
    pub pos_loc: i32,
    pub target_loc: i32,
    pub color_loc: i32,
}

impl Light {
    // En codigo original es create_light()
    pub fn new(
        light_type: LightType,
        pos: Vector4,
        targ: Vector4,
        color: Color,
        shader: &mut raylib::prelude::WeakShader,
    ) -> Self {
        // unsafe {
        //     dbg!(LIGHTS_COUNT);
        // }

        let mut light = Light::default();

        if (unsafe { LIGHTS_COUNT as u32 } < MAX_LIGHTS) {
            light.enabled = true;
            light.light_type = light_type;
            light.position = pos.clone(); // ¿clone()?
            light.target = targ.clone(); // ¿clone()?
            light.color = color.clone(); // ¿clone()?

            let lights_count = unsafe { LIGHTS_COUNT };

            // Nombres de las variables del FS en su arreglo lights[] con instancias Light
            let enabled_name = format!("lights[{}].enabled", lights_count);
            let type_name = format!("lights[{}].type", lights_count);
            let pos_name = format!("lights[{}].position", lights_count);
            let target_name = format!("lights[{}].target", lights_count);
            let color_name = format!("lights[{}].color", lights_count);

            // Guardamos la localización de las variables del shader
            light.enabled_loc = shader.get_shader_location(&enabled_name);
            light.type_loc = shader.get_shader_location(&type_name);
            light.pos_loc = shader.get_shader_location(&pos_name);
            light.target_loc = shader.get_shader_location(&target_name);
            light.color_loc = shader.get_shader_location(&color_name);

            // Cargamos las variables en el shader
            update_light_values(shader, light.clone());

            unsafe {
                LIGHTS_COUNT += 1;
            }
        }
        light
    }
}

// Introduce datos de luz en el shader
pub fn update_light_values(shader: &mut WeakShader, light: Light) {
    // Enviamos al shader los valores de state y type de las luces habilitadas

    shader.set_shader_value(light.enabled_loc, light.enabled as i32);

    shader.set_shader_value(light.type_loc, light.light_type as i32);

    // Enviamos al shader la posición de la luz
    shader.set_shader_value(light.pos_loc, light.position.to_v3_raylib());

    // Enviamos al shader el color de la luz convirtiendo el color en Vector4
    let kk = light.color.to_color_raylib(); // Color de libfinal -> Color de raylib
    let color: raylib::prelude::Vector4 = kk.into(); // Color de raylib -> Vector4 de raylib
    shader.set_shader_value(light.color_loc, color);
}
