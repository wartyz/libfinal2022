use raylib::prelude::*;

use libfinal::engine::Engine;
use libfinal::matem::random_range;
use libfinal::p5::color_p5::{fill, fill1, fill3, lerp_color, stroke, stroke1, stroke3, Color};
use libfinal::p5::vector_p5::Vector3;
use libfinal::parametros::{ModosBeginShape, Parametros};
use libfinal::shapes::{begin_shape, end_shape, line, stroke_weight, vertex};
use libfinal::structure::{pop, push};
use libfinal::transform::{rotate_z, translate};

pub struct Vehicle {
    acceleration: Vector3,
    velocity: Vector3,
    position: Vector3,
    r: f32,
    maxspeed: f32,
    maxforce: f32,

    health: f32,

    dna: [f32; 2],
}

impl Vehicle {
    pub fn new(x: f32, y: f32) -> Vehicle {
        let dna = [random_range(-5.0, 5.0), random_range(-5.0, 5.0)];
        //let dna = [1.0, -1.0];
        Vehicle {
            acceleration: Vector3::new(0.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, -2.0, 0.0),
            position: Vector3::new(x, y, 0.0),
            r: 4.0,
            maxspeed: 5.0,
            maxforce: 0.5,

            health: 1.0,

            dna,
        }
    }

    pub fn update(&mut self) {
        self.health -= 0.01;
        // Update vlocity
        self.velocity.add(&self.acceleration);
        // Limit speed
        self.velocity.limit(self.maxspeed);
        self.position.add(&self.velocity);
        // Reset accelerationelertion to 0 each cycle
        self.acceleration.mult(0.0);
    }

    pub fn applyforce(&mut self, force: Vector3) {
        // We could add mass here if we want A = F
        self.acceleration.add(&force)
    }

    pub fn behaviors(&mut self, good: &mut Vec<Vector3>, bad: &mut Vec<Vector3>) {
        let mut steer_g = self.eat(good, 0.1);
        let mut steer_b = self.eat(bad, -0.5);

        steer_g.mult(self.dna[0]);
        steer_b.mult(self.dna[1]);

        self.applyforce(steer_g);
        self.applyforce(steer_b);
    }

    pub fn eat(&mut self, list: &mut Vec<Vector3>, nutrition: f32) -> Vector3 {
        let mut record = std::f32::MAX;
        let mut closest = None; // -1 en video
        for i in 0..list.len() {
            let d = self.position.dist(&list[i]);
            if d < record {
                record = d;
                closest = Some(i);
            }
        }

        // This is the moment of eating
        if record < 5.0 {
            list.remove(closest.unwrap());
            self.health += nutrition;
        } else if closest.is_some() {
            return self.seek(list[closest.unwrap()]);
        }

        Vector3::new(0.0, 0.0, 0.0)
    }

    pub fn seek(&mut self, target: Vector3) -> Vector3 {
        let mut desired = Vector3::sub_s(&target, &self.position);

        // Scale to maximum speed
        desired.set_mag(self.maxspeed);

        // Steering = Desired minus velocity
        let mut steer = Vector3::sub_s(&desired, &self.velocity);
        steer.limit(self.maxforce); // Limit to max

        steer
        //self.applyforce(steer);
    }

    pub fn dead(&mut self) -> bool {
        self.health < 0.0
    }

    pub fn display(&mut self, engine: &mut Engine, d: &mut RaylibDrawHandle) {
        // Draw a triangle rotated in the direction
        let angle = self.velocity.heading() + PI as f32 / 2.0;
        //

        push(engine);

        translate(self.position.x, self.position.y, &mut engine.param);
        //println!("position.x = {}", self.position.x);
        //rotate_z_por_un_punto(angle, self.position.x, self.position.y, &mut engine.param);
        rotate_z(angle, &mut engine.param);

        stroke3(0.0, 255.0, 0.0, &mut engine.param);

        line(d, &mut engine.param, 0.0, 0.0, 0.0, -self.dna[0] * 20.0);
        //        line(engine.canvas.as_mut().unwrap(),
        //             &mut engine.param, 0.0, 0.0, 0.0, -self.dna[0] * 20.0);

        stroke3(255.0, 0.0, 0.0, &mut engine.param);

        line(d, &mut engine.param, 0.0, 0.0, 0.0, -self.dna[1] * 20.0);
        //        line(engine.canvas.as_mut().unwrap(),
        //             &mut engine.param, 0.0, 0.0, 0.0, -self.dna[1] * 20.0);

        let gr = Color::new(0, 255, 0, 255);
        let rd = Color::new(255, 0, 255, 255);
        let col = lerp_color(rd, gr, self.health);

        //fill1(127.0, &mut engine.param);
        fill(col, &mut engine.param);
        stroke(col, &mut engine.param);
        stroke_weight(1.0, &mut engine.param);
        begin_shape(ModosBeginShape::NadaShape);
        vertex(0.0, -self.r * 2.0, &mut engine.param);
        vertex(-self.r, self.r * 2.0, &mut engine.param);
        vertex(self.r, self.r * 2.0, &mut engine.param);
        //        end_shape(&mut engine.canvas.as_mut().unwrap(), &mut engine.param);
        end_shape(d, &mut engine.param, ModosBeginShape::Close);

        pop(engine);
    }
}
