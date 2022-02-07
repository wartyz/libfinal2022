use raylib::prelude::*;

use crate::parametros::*;
use crate::transform::identity3x3;
use crate::transform3d::identity4x4;

pub struct Engine {
    pub param: Parametros,
    pub parambakup: Parametros,
    pub si_ventana: bool,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new(100.0, 200.0)
    }
}

impl Engine {
    pub fn new(ancho: f32, alto: f32) -> Engine {
        Engine {
            //rl,
            //th,
            param: Parametros::new(ancho, alto),
            parambakup: Parametros::new(ancho, alto),
            si_ventana: false,
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, _th: &RaylibThread) -> bool {
        self.modifica_framecount();
        self.reinicia_matrices();
        self.reinicia_vertex();
        self.reinicia_teclado();
        self.reinicia_raton();

        self.comprueba_teclado(rl);
        self.comprueba_raton(rl);
        self.param.mouse_posicion = rl.get_mouse_position();

        let k = rl.window_should_close();

        !k
    }
    pub fn modifica_framecount(&mut self) {
        self.param.framecount += 1;
    }

    pub fn reinicia_matrices(&mut self) {
        self.param.matriz_total = identity3x3();
        self.param.matriz_total3d = identity4x4();
    }

    // Aqui se reinicia en arreglo vertex: Vec<Vector2> para shapes
    pub fn reinicia_vertex(&mut self) {
        //self.param.vertex.clear();
    }

    // Aqui se reinicia en el indicador de tecla presionad
    pub fn reinicia_teclado(&mut self) {
        self.param.key = CodigosTecla::NadaTecla;
        self.param.keyr = CodigosTecla::NadaTecla;
    }

    // Aqui se reinicia en el indicador de boton de raton presionado
    pub fn reinicia_raton(&mut self) {
        self.param.mouse_boton_inicio = CodigosRaton::NadaRaton;
        self.param.mouse_boton_mantiene = CodigosRaton::NadaRaton;
        self.param.mouse_boton_final = CodigosRaton::NadaRaton;
    }

    pub fn comprueba_teclado(&mut self, rl: &RaylibHandle) {
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_RIGHT) {
            self.param.key = CodigosTecla::RightArrow;
        }
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_LEFT) {
            self.param.key = CodigosTecla::LeftArrow;
        }
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_UP) {
            self.param.key = CodigosTecla::UpArrow;
        }
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_DOWN) {
            self.param.key = CodigosTecla::DownArrow;
        }
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_A) {
            self.param.key = CodigosTecla::A;
        }
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_B) {
            self.param.key = CodigosTecla::B;
        }
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_C) {
            self.param.key = CodigosTecla::C;
        }
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_G) {
            self.param.key = CodigosTecla::G;
        }
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_R) {
            self.param.key = CodigosTecla::R;
        }
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_S) {
            self.param.key = CodigosTecla::S;
        }
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_W) {
            self.param.key = CodigosTecla::W;
        }
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_Z) {
            self.param.key = CodigosTecla::Z;
        }
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_KP_1) {
            self.param.key = CodigosTecla::N1;
        }
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_KP_2) {
            self.param.key = CodigosTecla::N2;
        }
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_KP_3) {
            self.param.key = CodigosTecla::N3;
        }
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_KP_4) {
            self.param.key = CodigosTecla::N4;
        }
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_SPACE) {
            self.param.key = CodigosTecla::SPACE;
        }
    }

    pub fn comprueba_raton(&mut self, rl: &RaylibHandle) {
        if rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_RIGHT_BUTTON) {
            self.param.mouse_boton_inicio = CodigosRaton::Derecho;
        }
        if rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON) {
            //println!("En comprueba_raton LEFT");
            self.param.mouse_boton_inicio = CodigosRaton::Izquierdo;
        }
        if rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_MIDDLE_BUTTON) {
            self.param.mouse_boton_inicio = CodigosRaton::Medio;
        }
        // -----------------------------------------------------------------------------------
        if rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_RIGHT_BUTTON) {
            self.param.mouse_boton_mantiene = CodigosRaton::Derecho;
        }
        if rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON) {
            self.param.mouse_boton_mantiene = CodigosRaton::Izquierdo;
        }
        if rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_MIDDLE_BUTTON) {
            self.param.mouse_boton_mantiene = CodigosRaton::Medio;
        }
        // -----------------------------------------------------------------------------------
        if rl.is_mouse_button_up(raylib::consts::MouseButton::MOUSE_RIGHT_BUTTON) {
            self.param.mouse_boton_final = CodigosRaton::Derecho;
        }
        if rl.is_mouse_button_up(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON) {
            self.param.mouse_boton_final = CodigosRaton::Izquierdo;
        }
        if rl.is_mouse_button_up(raylib::consts::MouseButton::MOUSE_MIDDLE_BUTTON) {
            self.param.mouse_boton_final = CodigosRaton::Medio;
        }
        // -----------------------------------------------------------------------------------

        // Rueda
        self.param.mouse_rueda = rl.get_mouse_wheel_move();
    }
}
