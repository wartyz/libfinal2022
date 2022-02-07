//use nalgebra as na;
use raylib::prelude::*;

use crate::p5::color_p5::Color;
use crate::p5::vector_p5::Vector3;
use crate::parametros::ModosAngulo::Radians;
use crate::transform::{identity3x3, Matrix3x3};
use crate::transform3d::{identity4x4, Matrix4x4};

// Para función angleMode() -------------------------------------

#[derive(Debug, Copy, Clone)]
pub enum ModosAngulo {
    Radians,
    Degrees,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorMode {
    RGB,
    HSB,
    HSL,
}

// Para función BeginShape() -------------------------------------
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModosBeginShape {
    Points,
    Lines,
    Triangles,
    TriangleFan,
    TriangleStrip,
    Quads,
    QuadStrip,
    Close,
    NadaShape,
}

// Para el modo de los shapes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RectMode {
    Corner,
    Corners,
    Center,
    Radius,
}

// Para teclado  ---------------------------------
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CodigosTecla {
    NadaTecla,
    BACKSPACE,
    DELETE,
    ENTER,
    RETURN,
    TAB,
    ESCAPE,
    SHIFT,
    CONTROL,
    Option,
    Alt,
    UpArrow,
    DownArrow,
    LeftArrow,
    RightArrow,
    SPACE,
    A,
    B,
    C,
    G,
    J,
    M,
    R,
    S,
    W,
    Z,
    N1,
    N2,
    N3,
    N4,
}

// Para ratón  ---------------------------------
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CodigosRaton {
    NadaRaton,
    Derecho,
    Izquierdo,
    Medio,
}

// Para filter() en P5.image_p5
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Filtros {
    NoFiltro,
    Threshold,
    Gray,
    Opaque,
    Invert,
    Posterize,
    Blur,
    Erode,
    Dilate,
}

// Para modos de Imagen  ---------------------------------
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImageMode {
    Corner,
    Corners,
    Center,
}

// Para vision 3d
#[derive(Debug, Clone)]
pub struct Camara {
    // Posición de la cámara
    pub posx: f32,
    pub posy: f32,
    pub posz: f32,
    // Punto al que mira
    pub objx: f32,
    pub objy: f32,
    pub objz: f32,
    // Vector up
    pub upx: f32,
    pub upy: f32,
    pub upz: f32,
    // angulo de vision
    pub fovy: f32,
    //distancia_cuadrado_eje: f32,
}

impl Default for Camara {
    fn default() -> Self {
        Self {
            posx: 4.0,
            posy: 4.0,
            posz: 4.0,

            objx: 0.0,
            objy: 0.0,
            objz: 0.0,

            upx: 0.0,
            upy: 1.0,
            upz: 0.0,

            fovy: 45.0,
            //distancia_cuadrado_eje: 200.0,
        }
    }
}

// impl Camara {
//     pub fn new(
//         // Posición de la cámara
//         posx: f32,
//         posy: f32,
//         posz: f32,
//         // Punto al que mira
//         objx: f32,
//         objy: f32,
//         objz: f32,
//         // Vector up
//         upx: f32,
//         upy: f32,
//         upz: f32,
//         // angulo de vision
//         fovy: f32,
//     ) -> Self {
//         Self {
//             posx,
//             posy,
//             posz,
//
//             objx,
//             objy,
//             objz,
//
//             upx,
//             upy,
//             upz,
//
//             fovy,
//         }
//     }
//
//
// }

#[derive(Debug, Clone)]
pub struct Parametros {
    pub ancho: f32,
    pub alto: f32,

    pub mouse_posicion: raylib::prelude::Vector2,

    pub mouse_boton_inicio: CodigosRaton,
    pub mouse_boton_mantiene: CodigosRaton,
    pub mouse_boton_final: CodigosRaton,

    pub mouse_rueda: f32,

    pub matriz_total: Matrix3x3,
    pub matriz_total3d: Matrix4x4,
    pub matriz_stack: Vec<Matrix3x3>,
    pub matriz_stack3d: Vec<Matrix4x4>,

    pub fill_bool: bool,
    pub fill_color: Color,
    pub color_background: Color,

    pub stroke_bool: bool,
    pub stroke_weight: f32,
    pub stroke_color: Color,
    //coord_cursor: Vector3,
    pub colormode: ColorMode,

    // Para shapes no implementa copy lo quito provisionalmente
    //pub vertex: Vec<Vector2>,
    // Para BeginShape() Vertex() y EndShape() --
    pub vectores: Vec<Vector2>,
    pub rect_mode: RectMode,

    // Para tecladp
    pub key: CodigosTecla,
    pub keyr: CodigosTecla,

    // Para la font de texto
    pub tamafont: i32,

    // Para sliders
    pub cantidad_de_sliders: i32,

    // Para loadpixels()
    pub pixels: Vec<raylib::color::Color>,

    // Para angulos
    pub angulo_mode: ModosAngulo,

    // para Image coordenada de agarre
    pub image_mode: ImageMode,

    // para poder utilizar background() en draw() y no usarlo en setup()
    pub background_1_vez: bool,

    // para visión 3d
    pub camara: Camara,
    // En ejes distancia del cuadrado al centro
    pub distancia_cuadrado_eje: f32,

    // cuenta frames dede el inicio
    pub framecount: i32,
}

impl Default for Parametros {
    fn default() -> Self {
        Self::new(10.0, 20.0)
    }
}

impl Parametros {
    pub fn new(ancho: f32, alto: f32) -> Parametros {
        Parametros {
            ancho: ancho,
            alto: alto,

            mouse_posicion: Vector2::new(0.0, 0.0),

            mouse_boton_inicio: CodigosRaton::NadaRaton,
            mouse_boton_mantiene: CodigosRaton::NadaRaton,
            mouse_boton_final: CodigosRaton::NadaRaton,

            mouse_rueda: 0.0,
            matriz_total: identity3x3(),
            matriz_total3d: identity4x4(),
            matriz_stack: vec![],
            matriz_stack3d: vec![],

            fill_bool: true,
            fill_color: Color::new(255, 255, 255, 255),
            color_background: Color::new(255, 255, 255, 255),
            stroke_bool: true,
            stroke_weight: 1.0,
            stroke_color: Color::new(0, 0, 0, 255),
            //coord_cursor: Vector3::new(0.0, 0.0, 0.0),
            colormode: ColorMode::RGB,
            //vertex: vec![],
            vectores: vec![],
            rect_mode: RectMode::Corner,
            key: CodigosTecla::NadaTecla,
            keyr: CodigosTecla::NadaTecla,
            tamafont: 16,
            cantidad_de_sliders: 0,

            pixels: vec![],

            angulo_mode: Radians,
            image_mode: ImageMode::Corner,

            background_1_vez: false,

            camara: Camara::default(),
            distancia_cuadrado_eje: 0.0,
            framecount: 0,
        }
    }

    pub fn set_camara(
        &mut self, // Posición de la cámara
        posx: f32,
        posy: f32,
        posz: f32,
        // Punto al que mira
        objx: f32,
        objy: f32,
        objz: f32,
        // Vector up
        upx: f32,
        upy: f32,
        upz: f32,
        // angulo de vision
        fovy: f32,
    ) {
        self.camara.posx = posx;
        self.camara.posy = posy;
        self.camara.posz = posz;

        self.camara.objx = objx;
        self.camara.objy = objy;
        self.camara.objz = objz;

        self.camara.upx = upx;
        self.camara.upy = upy;
        self.camara.upz = upz;

        self.camara.fovy = fovy;
    }

    // Recibe una camara y devuelve una Camera3d de raylib ortografica
    pub fn convierte_a_3d_raylib_ortho(&self) -> raylib::prelude::Camera3D {
        Camera3D::orthographic(
            raylib::prelude::Vector3::new(self.camara.posx, self.camara.posy, self.camara.posz), // posición
            raylib::prelude::Vector3::new(self.camara.objx, self.camara.objy, self.camara.objz), // objetivo
            raylib::prelude::Vector3::new(self.camara.upx, self.camara.upy, self.camara.upz), // up
            self.camara.fovy,
        )
    }

    // Recibe una camara y devuelve una Camera3d de raylib perspectiva

    pub fn convierte_a_3d_raylib_persp(&self) -> raylib::prelude::Camera3D {
        Camera3D::perspective(
            raylib::prelude::Vector3::new(self.camara.posx, self.camara.posy, self.camara.posz), // posición
            raylib::prelude::Vector3::new(self.camara.objx, self.camara.objy, self.camara.objz), // objetivo
            raylib::prelude::Vector3::new(self.camara.upx, self.camara.upy, self.camara.upz), // up
            self.camara.fovy,
        )
    }

    pub fn set_distancia_cuadrado_ejes_3d(&mut self, d: f32) {
        self.distancia_cuadrado_eje = d;
    }
}
