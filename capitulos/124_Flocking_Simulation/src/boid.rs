// 18:08 no se mueve

use raylib::core::drawing::RaylibDrawHandle;
use raylib::prelude::*;

//use libfinal::dom::Slider;
use libfinal::matem::{dist4, random, random_range};
use libfinal::p5::color_p5::stroke1;
//use raylib::core::math::Vector2;
use libfinal::p5::vector_p5::Vector3;
use libfinal::parametros::Parametros;
use libfinal::shapes::*;

use crate::sketch::{ALTO, ANCHO};

#[derive(Debug, Clone, Copy)]
pub struct Boid {
    position: Vector3,
    velocity: Vector3,
    acceleration: Vector3,
    maxforce: f32,
    maxspeed: f32,
}

// Para poder evaluar igualdades entre objetos Bold
impl PartialEq for Boid {
    fn eq(&self, otro: &Self) -> bool {
        self.position.x == otro.position.x && self.position.y == otro.position.y
    }
}

impl Boid {
    pub fn new(ancho: f32, alto: f32) -> Boid {
        let mut velocity = Vector3::random2d_s();
        velocity.set_mag(random_range(2.0, 4.0));
        Boid {
            position: Vector3::new(random(ancho), random(alto), 0.0),
            velocity,
            acceleration: Vector3::new(0.0, 0.0, 0.0),
            maxforce: 10.0,
            maxspeed: 4.0,
        }
    }

    pub fn edges(mut self) -> Self {
        if self.position.x > ANCHO as f32 {
            self.position.x = 0.0;
        } else if self.position.x < 0.0 {
            self.position.x = ANCHO as f32;
        }
        if self.position.y > ALTO as f32 {
            self.position.y = 0.0;
        } else if self.position.y < 0.0 {
            self.position.y = ALTO as f32;
        }
        self
    }

    pub fn align(mut self, boids: &Vec<Boid>) -> Vector3 {
        let perception: f32 = 100.0;
        let mut steering = Vector3::new(0.0, 0.0, 0.0);
        let mut total: f32 = 0.0;

        for other in boids.iter() {
            let d = dist4(
                self.position.x,
                self.position.y,
                other.position.x,
                other.position.y,
            );
            if (*other != self) && (d < perception) {
                steering.add(&other.velocity);
                total += 1.0;
            }
        }
        if total > 0.0 {
            steering.div(total);
            steering.set_mag(self.maxspeed);
            steering.sub(&self.velocity);
            steering.limit(self.maxforce);
        }
        steering
    }

    pub fn cohesion(mut self, boids: &Vec<Boid>) -> Vector3 {
        let perception: f32 = 100.0;
        let mut steering = Vector3::new(0.0, 0.0, 0.0);
        let mut total: f32 = 0.0;

        for other in boids.iter() {
            let d = dist4(
                self.position.x,
                self.position.y,
                other.position.x,
                other.position.y,
            );
            if (*other != self) && (d < perception) {
                steering.add(&other.position);
                total += 1.0;
            }
        }
        if total > 0.0 {
            steering.div(total);
            steering.sub(&self.position);
            steering.set_mag(self.maxspeed);
            steering.sub(&self.velocity);
            steering.limit(self.maxforce);
        }
        steering
    }

    pub fn separation(mut self, boids: &Vec<Boid>) -> Vector3 {
        let perception: f32 = 50.0;
        let mut steering = Vector3::new(0.0, 0.0, 0.0);
        let mut total: f32 = 0.0;

        for other in boids.iter() {
            let d = dist4(
                self.position.x,
                self.position.y,
                other.position.x,
                other.position.y,
            );
            if (*other != self) && (d < perception) {
                let mut diff = Vector3::sub_s(&self.position, &other.position);
                diff.div(d);
                steering.add(&diff);
                total += 1.0;
            }
        }
        if total > 0.0 {
            steering.div(total);
            //steering.sub(&self.position);
            steering.set_mag(self.maxspeed);
            steering.sub(&self.velocity);
            steering.limit(self.maxforce);
        }
        steering
    }

    //    pub fn flock(mut self, boids: &Vec<Boid>) {
    //        let alignment = self.align(boids);
    //        self.acceleration = alignment;
    //    }

    pub fn flock(
        mut self,
        boids: &Vec<Boid>,
        align_slider: f32,
        cohesion_slider: f32,
        separation_slider: f32,
    ) -> Self {
        //self.acceleration.set(0.0, 0.0);
        let mut alignment = self.align(boids);
        let mut cohesion = self.cohesion(boids);
        let mut separation = self.separation(boids);

        alignment.mult(align_slider);
        cohesion.mult(cohesion_slider);
        separation.mult(separation_slider);

        self.acceleration.add(&separation);
        self.acceleration.add(&alignment);
        self.acceleration.add(&cohesion);
        self
    }

    pub fn update(mut self) -> Self {
        self.position.add(&self.velocity);
        self.velocity.add(&self.acceleration);
        self.velocity.limit(self.maxspeed);
        self.acceleration.mult(0.0);

        self
    }

    pub fn show(mut self, d: &mut RaylibDrawHandle, param: &mut Parametros) -> Self {
        stroke_weight(8.0, param);
        stroke1(255.0, param);
        point(d, param, self.position.x, self.position.y);
        self
    }
}
