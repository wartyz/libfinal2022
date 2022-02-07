use raylib::core::drawing::RaylibDrawHandle;
use raylib::core::math::Vector2;
use raylib::prelude::{Image, RaylibHandle, RaylibThread, Texture2D, PI};

use libfinal::engine::Engine;
use libfinal::image::{image_mode, ImageLf};
use libfinal::matem::{constrain, pow, random, random_range};
use libfinal::p5::color_p5;
use libfinal::p5::color_p5::stroke1;
use libfinal::p5::vector_p5::*;
use libfinal::parametros::{ImageMode, Parametros};
use libfinal::shapes::{point, stroke_weight};
use libfinal::structure::{pop, push};
use libfinal::transform::{pop_matrix, push_matrix, rotate_z, translate};

use crate::sketch::{ALTO, ANCHO};

pub struct Snowflake {
    pos: Vector3,
    vel: Vector3,
    acc: Vector3,
    angle: f32,
    dir: f32,
    r: f32,
    mass: f32,
    //img: Image,
    textu: Texture2D,
    //terminal_v: f32,
    xoff: f32,
}

impl Snowflake {
    pub fn new(img: Image, rl: &mut RaylibHandle, th: &RaylibThread) -> Snowflake {
        let x = random(ANCHO as f32);
        let y = random_range(-100.0, -10.0);

        let r = get_random_size();
        let textu = rl.load_texture_from_image(th, &img).expect("error ");

        let mut dir = 1.0;
        if random(1.0) > 0.5 {
            dir = -1.0;
        }
        Snowflake {
            pos: Vector3::new(x, y, 0.0),
            vel: Vector3::new(0.0, 0.0, 0.0),
            acc: Vector3::new(0.0, 0.0, 0.0),
            angle: random(PI as f32 * 2.0),
            dir,
            r,
            mass: 0.0,
            //img,
            textu: textu,

            xoff: 0.0,
        }
    }

    pub fn new2(
        x: f32,
        y: f32,
        img: &Image,
        rl: &mut RaylibHandle,
        th: &RaylibThread,
    ) -> Snowflake {
        let textu = rl.load_texture_from_image(th, img).expect("error ");

        let r = get_random_size();

        let mut dir = 1.0;
        if random(1.0) > 0.5 {
            dir = -1.0;
        }
        Snowflake {
            pos: Vector3::new(x, y, 0.0),
            vel: Vector3::new(0.0, 0.0, 0.0),
            acc: Vector3::new(0.0, 0.0, 0.0),
            angle: random(PI as f32 * 2.0),
            r,
            dir,
            mass: 0.0,
            //img,
            textu: textu,
            xoff: 0.0,
        }
    }

    pub fn apply_force(&mut self, force: Vector3) {
        // Parallax effect hack
        let mut f = force.copy();
        f.mult(self.r);

        //        let mut f = force.copy();
        //        f.div(self.mass);
        self.acc.add(&force);
    }

    pub fn randomize(&mut self) {
        let x = random(ANCHO as f32);
        let y = random_range(-100.0, -10.0);

        self.pos = Vector3::new(x, y, 0.0);
        self.vel = Vector3::new(0.0, 0.0, 0.0);
        self.acc = Vector3::new(0.0, 0.0, 0.0);
        self.r = get_random_size();
        self.mass = 0.0;
        //terminal_v: r,
    }

    pub fn update(&mut self, gravity: Vector3) {
        self.xoff = (self.angle).sin() * self.r;

        self.vel.add(&self.acc);
        self.vel.limit(self.r * 0.2);

        if self.vel.mag() < 1.0 {
            self.vel.normalize();
        }

        self.pos.add(&self.vel);
        self.acc.mult(0.0);

        if self.pos.y > ALTO as f32 + self.r {
            self.randomize();
        }

        self.angle += self.dir * self.vel.mag() / 200.0;
    }

    pub fn render(&mut self, d: &mut RaylibDrawHandle, engine: &mut Engine) {
        //        stroke1(255.0, param);
        //        stroke_weight(self.r, param);
        //        point(d, param, self.pos.x, self.pos.y);

        push(engine);
        translate(self.pos.x + self.xoff, self.pos.y, &mut engine.param);
        rotate_z(self.angle, &mut engine.param);
        image_mode(&mut engine.param, ImageMode::Center);

        ImageLf::image(d, &mut engine.param, &self.textu, 0.0, 0.0);
        pop(engine);
    }

    //    pub fn off_screen(&mut self) -> bool {
    //        let alto = crate::sketch::ALTO as f32;
    //        self.pos.y > alto + self.r
    //    }
}

// En video usa randomgaussian()
fn get_random_size() -> f32 {
    let r = pow(random_range(0.0, 1.0), 3);
    return constrain(r * 32.0, 2.0, 32.0);

    //    while true {
    //        let r1 = random(1.0);
    //        let r2 = random(1.0);
    //
    //        if r2 > r1 {
    //            let r = r1 * 5.5;
    //            return constrain((r * r), 2.0, 36.0);
    //        }
    //    }
    //    return 0.0;
}
