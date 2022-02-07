// Las funciones estáticas acaban en _s

use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
impl Default for Vector4 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn set(&mut self, x: f32, y: f32, z: f32, w: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
        self.w = w;
    }

    // devuelve una copia de el mismo
    pub fn copy(&self) -> Vector4 {
        Vector4::new(self.x, self.y, self.z, self.w)
    }

    // Suma a este otro vector
    pub fn add(&mut self, b: &Vector4) {
        self.x += b.x;
        self.y += b.y;
        self.z += b.z;
        self.w += b.w;
    }

    // Limita la magnitud máxima del vector NO ESTOY SEGURO
    pub fn limit(&mut self, max: f32) {
        let magnitud_actual = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if magnitud_actual > max {
            self.set_mag(max);
        }
    }

    // Normaliza este vector
    pub fn normalize(&mut self) {
        let longitud = self.mag();

        if longitud != 0.0 {
            self.x /= longitud;
            self.y /= longitud;
            self.z /= longitud
        }
    }

    // Devuelve la magnitud de este vector
    pub fn mag(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    // Devuelve la magnitud al cuadrado (por velocidad)
    fn mag_sq(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    // Establece la magnitud de este vector
    pub fn set_mag(&mut self, magnitud: f32) {
        self.normalize();
        self.x *= magnitud;
        self.y *= magnitud;
        self.z *= magnitud;
    }

    // Multiplica este vector por un escalar
    pub fn mult(&mut self, v: f32) {
        self.x *= v;
        self.y *= v;
        self.z *= v;
    }

    // NO ESTOY SEGURO
    pub fn lerp(&mut self, vector: Vector4, amt: f32) {
        self.x = (1.0 - amt) * self.x + amt * vector.x;
        self.y = (1.0 - amt) * self.y + amt * vector.y;
        self.z = (1.0 - amt) * self.z + amt * vector.z;
    }

    // Distancia entre dos vectores función estática d = ((x2 - x1)2 + (y2 - y1)2 + (z2 - z1)2)1/2
    pub fn dist_s(a: &Vector4, b: &Vector4) -> f32 {
        let x = a.x - b.x;
        let y = a.y - b.y;
        let z = a.z - b.z;
        ((x * x) + (y * y) + (z * z)).sqrt()
    }

    // Distancia entre este vector y otro
    pub fn dist(&mut self, b: &Vector4) -> f32 {
        let x = self.x - b.x;
        let y = self.y - b.y;
        let z = self.z - b.z;
        ((x * x) + (y * y) + (z * z)).sqrt()
    }

    // Resta de dos vectores función estática
    pub fn sub_s(a: &Vector4, b: &Vector4) -> Vector4 {
        Vector4::new(a.x - b.x, a.y - b.y, a.z - b.z, 0.0)
    }

    // Calcula el ángulo de rotación para este vector los vectores creados
    // usando createVector() tomarán en consideración el ángulo de ángulo actual,
    // y darán el ángulo en radianes o grados en consecuencia.
    pub fn heading(&mut self) -> f32 {
        self.y.atan2(self.x)
    }

    // Devuelve un vector aleatório función estática
    pub fn random2d_s() -> Vector4 {
        let mut rng = rand::thread_rng();
        let x: f32 = rng.gen_range(-1.0, 1.0);
        let y: f32 = rng.gen_range(-1.0, 1.0);
        let z: f32 = rng.gen_range(-1.0, 1.0);

        //println!("En p5_vector random2d x= {:?}", x);
        let mut v = Vector4::new(x, y, z, 0.0);
        v.normalize();
        v
    }

    // Divide este vector por un numero
    pub fn div(&mut self, b: f32) {
        self.x /= b;
        self.y /= b;
        self.z /= b;
    }

    // Resta a este otro vector
    pub fn sub(&mut self, b: &Vector4) {
        self.x -= b.x;
        self.y -= b.y;
        self.z -= b.z;
    }

    // pub fn from_angle_s(angulo: f32) -> Vector4 {
    //     Vector4::new(cos(angulo), sin(angulo), 0.0)
    // }

    // // Me la invento, devuelve el ángulo del vector en radianes
    // fn get_angle(&mut self) -> f32 {
    //     if self.mag() == 0.0 {
    //         return 0.0;
    //     }
    //     let mut r = self.y.atan2(self.x);
    //     if r < 0.0 {
    //         r += 2.0 * PI;
    //     }
    //     r
    // }

    // // Me la invento establece longitud del vector
    // fn set_length(&mut self, long: f32) {
    //     let ang_rad = self.get_angle();
    //     self.x = long * ang_rad.cos();
    //     self.y = long * ang_rad.sin();
    // }

    // // Me la invento establece el ángulo
    // fn set_angle(&mut self, _angulo: f32) {
    //     let long = self.mag();
    //     let ang_viejo_rad = self.get_angle();
    //     self.x = long * ang_viejo_rad.cos();
    //     self.y = long * ang_viejo_rad.sin();
    // }

    pub fn to_v3_raylib(&self) -> raylib::core::math::Vector3 {
        raylib::core::math::Vector3::new(self.x, self.y, self.z)
    }
    pub fn to_v4_raylib(&self) -> raylib::core::math::Vector4 {
        raylib::core::math::Vector4::new(self.x, self.y, self.z, self.w)
    }
    // NO ESTOY SEGURO
    pub fn vector3lerp(vector_a: Vector4, vector_b: Vector4, amt: f32) -> Vector4 {
        let rx = (1.0 - amt) * vector_a.x + amt * vector_b.x;
        let ry = (1.0 - amt) * vector_a.y + amt * vector_b.y;
        let rz = (1.0 - amt) * vector_a.z + amt * vector_b.z;
        Vector4::new(rx, ry, rz, 0.0)
    }
}
