// Las funciones estáticas acaban en _s

use crate::matem::{cos, sin};
use rand::Rng;
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub w: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, w: f32) -> Self {
        Self { x, y, w }
    }

    pub fn set(&mut self, x: f32, y: f32, w: f32) {
        self.x = x;
        self.y = y;
        self.w = w;
    }

    // devuelve una copia de el mismo
    pub fn copy(&self) -> Vector3 {
        Vector3::new(self.x, self.y, self.w)
    }

    // Suma a este otro vector
    pub fn add(&mut self, b: &Vector3) {
        self.x += b.x;
        self.y += b.y;
        self.w += b.w;
    }

    // Limita la magnitud máxima del vector
    pub fn limit(&mut self, max: f32) {
        let magnitud_actual = (self.x * self.x + self.y * self.y).sqrt();
        if magnitud_actual > max {
            self.set_mag(max);
        }
    }

    // Normaliza este vector
    pub fn normalize(&mut self) {
        let a = self.x * self.x;
        let b = self.y * self.y;
        //let c = float64(v.Z * v.Z)

        let longitud = (a + b).sqrt();

        if longitud != 0.0 {
            self.x /= longitud;
            self.y /= longitud;
            //v.Z /= longitud
        }
    }

    // Devuelve la magnitud de este vector
    pub fn mag(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    // Devuelve la magnitud al cuadrado (por velocidad)
    fn mag_sq(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    // Establece la magnitud de este vector
    pub fn set_mag(&mut self, magnitud: f32) {
        self.normalize();
        self.x *= magnitud;
        self.y *= magnitud;
    }

    // Multiplica este vector por un escalar
    pub fn mult(&mut self, v: f32) {
        self.x *= v;
        self.y *= v;
    }

    pub fn lerp(&mut self, vector: Vector3, amt: f32) {
        self.x = (1.0 - amt) * self.x + amt * vector.x;
        self.y = (1.0 - amt) * self.y + amt * vector.y;
    }

    // Distancia entre dos vectores función estática
    pub fn dist_s(a: &Vector3, b: &Vector3) -> f32 {
        let x = a.x - b.x;
        let y = a.y - b.y;
        ((x * x) + (y * y)).sqrt()
    }

    // Distancia entre este vector y otro
    pub fn dist(&mut self, b: &Vector3) -> f32 {
        let x = self.x - b.x;
        let y = self.y - b.y;
        ((x * x) + (y * y)).sqrt()
    }

    // Resta de dos vectores función estática
    pub fn sub_s(a: &Vector3, b: &Vector3) -> Vector3 {
        Vector3::new(a.x - b.x, a.y - b.y, 0.0)
    }

    // Calcula el ángulo de rotación para este vector los vectores creados
    // usando createVector() tomarán en consideración el ángulo de ángulo actual,
    // y darán el ángulo en radianes o grados en consecuencia.
    pub fn heading(&mut self) -> f32 {
        self.y.atan2(self.x)
    }

    // Devuelve un vector aleatório función estática
    pub fn random2d_s() -> Vector3 {
        let mut rng = rand::thread_rng();
        let x: f32 = rng.gen_range(-1.0, 1.0);
        let y: f32 = rng.gen_range(-1.0, 1.0);
        //println!("En p5_vector random2d x= {:?}", x);
        let mut v = Vector3::new(x, y, 0.0);
        v.normalize();
        v
    }

    // Divide este vector por un numero
    pub fn div(&mut self, b: f32) {
        self.x /= b;
        self.y /= b;
    }

    // Resta a este otro vector
    pub fn sub(&mut self, b: &Vector3) {
        self.x -= b.x;
        self.y -= b.y;
    }

    pub fn from_angle_s(angulo: f32) -> Vector3 {
        Vector3::new(cos(angulo), sin(angulo), 0.0)
    }

    // Me la invento, devuelve el ángulo del vector en radianes
    fn get_angle(&mut self) -> f32 {
        if self.mag() == 0.0 {
            return 0.0;
        }
        let mut r = self.y.atan2(self.x);
        if r < 0.0 {
            r += 2.0 * PI;
        }
        r
    }

    // Me la invento establece longitud del vector
    fn set_length(&mut self, long: f32) {
        let ang_rad = self.get_angle();
        self.x = long * ang_rad.cos();
        self.y = long * ang_rad.sin();
    }

    // Me la invento establece el ángulo
    fn set_angle(&mut self, _angulo: f32) {
        let long = self.mag();
        let ang_viejo_rad = self.get_angle();
        self.x = long * ang_viejo_rad.cos();
        self.y = long * ang_viejo_rad.sin();
    }

    pub fn to_v2_raylib(&self) -> raylib::core::math::Vector2 {
        raylib::core::math::Vector2::new(self.x, self.y)
    }

    pub fn to_v3_raylib(&self) -> raylib::core::math::Vector3 {
        raylib::core::math::Vector3::new(self.x, self.y, self.w)
    }

    pub fn vector3lerp(vector_a: Vector3, vector_b: Vector3, amt: f32) -> Vector3 {
        let rx = (1.0 - amt) * vector_a.x + amt * vector_b.x;
        let ry = (1.0 - amt) * vector_a.y + amt * vector_b.y;
        let ret = Vector3::new(rx, ry, 0.0);
        ret
    }
}
