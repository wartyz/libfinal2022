use raylib::prelude::*;

use libfinal::matem::*;
use libfinal::p5::color_p5::*;
use libfinal::parametros::{ColorMode, Parametros};
use libfinal::shapes::*;

pub const N: i32 = 128;
pub const SCALE: i32 = 4;

pub struct Fluid {
    size: i32,
    dt: f32,
    diff: f32,
    visc: f32,

    s: Vec<f32>,
    density: Vec<f32>,
    vx: Vec<f32>,
    vy: Vec<f32>,
    vx0: Vec<f32>,
    vy0: Vec<f32>,
}

impl Fluid {
    pub fn new(dt: f32, diffusion: f32, viscosity: f32) -> Fluid {
        Fluid {
            size: N,
            dt,
            diff: diffusion,
            visc: viscosity,

            s: vec![0.0; N as usize * N as usize],
            density: vec![0.0; N as usize * N as usize],

            vx: vec![0.0; N as usize * N as usize],
            vy: vec![0.0; N as usize * N as usize],

            vx0: vec![0.0; N as usize * N as usize],
            vy0: vec![0.0; N as usize * N as usize],
        }
    }

    pub fn step(&mut self, iter: i32) {
        let n = self.size;
        let visc = self.visc;
        let diff = self.diff;
        let dt = self.dt;
        //let vx = self.vx;
        //let vy = self.vy;
        //let vx0 = self.vx0;
//        let vy0 = self.vy0;
//        let s = self.s;
        //let density = self.density;

        diffuse(1.0, &mut self.vx0, &self.vx, visc, dt, iter);
        diffuse(2.0, &mut self.vy0, &self.vy, visc, dt, iter);

        project(&mut self.vx0, &mut self.vy0, &mut self.vx, &mut self.vy, iter);

        advect(1.0, &mut self.vx, &self.vx0, &self.vx0, &self.vy0, dt);
        advect(2.0, &mut self.vy, &self.vy0, &self.vx0, &self.vy0, dt);

        project(&mut self.vx, &mut self.vy, &mut self.vx0, &mut self.vy0, iter);

        diffuse(0.0, &mut self.s, &self.density, diff, dt, iter);
        advect(0.0, &mut self.density, &self.s, &self.vx, &self.vy, dt);
    }

    pub fn add_density(&mut self, x: i32, y: i32, amount: f32) {
        let index = ix(x, y);
        self.density[index] += amount;
    }

    pub fn add_velocity(&mut self, x: i32, y: i32, amountx: f32, amounty: f32) {
        let index = ix(x, y);
        self.vx[index] += amountx;
        self.vy[index] += amounty;
    }
    pub fn renderd(&mut self, d: &mut RaylibDrawHandle, param: &mut Parametros) {
        //let mut d = rl.begin_drawing(&th);
        //colormode(HSB, 255);
        colormode(ColorMode::HSB, param);

        for i in 0..N {
            for j in 0..N {
                let x = i * SCALE;
                let y = j * SCALE;
                let dd = self.density[ix(i, j)];
                fill3((dd + 50.0) % 255.0, 200.0, dd, param);
                no_stroke(param);
                square(d, param, x as f32, y as f32, SCALE as f32);
            }
        }
    }
}

pub fn ix(x: i32, y: i32) -> usize {
    let x = constraini32(x, 0, N - 1);
    let y = constraini32(y, 0, N - 1);
    (x + (y * N)) as usize
}

pub fn diffuse(b: f32, x: &mut Vec<f32>, x0: &Vec<f32>, diff: f32, dt: f32, iter: i32) {
    let a = dt * diff * (N - 2) as f32 * (N - 2) as f32;
    lin_solve(b, x, x0, a, 1.0 + 4.0 * a, iter);
}

pub fn lin_solve(b: f32, x: &mut Vec<f32>, x0: &Vec<f32>, a: f32, c: f32, iter: i32) {
    let crecip = 1.0 / c;

    for k in 0..iter {
        for j in 1..(N - 1) {
            for i in 1..(N - 1) {
                x[ix(i, j)] =
                    (x0[ix(i, j)]
                        + a * (x[ix(i + 1, j)]
                        + x[ix(i - 1, j)]
                        + x[ix(i, j + 1)]
                        + x[ix(i, j - 1)]
                    )) * crecip;
            }
        }

        set_bnd(b as i32, x);
    }
}

pub fn project(velocx: &mut Vec<f32>, velocy: &mut Vec<f32>, p: &mut Vec<f32>,
               div: &mut Vec<f32>, iter: i32) {
    for j in 1..(N - 1) {
        for i in 1..(N - 1) {
            div[ix(i, j)] = -0.5 * (
                velocx[ix(i + 1, j)]
                    - velocx[ix(i - 1, j)]
                    + velocy[ix(i, j + 1)]
                    - velocy[ix(i, j - 1)]
            ) / N as f32;
            p[ix(i, j)] = 0.0;
        }
    }

    set_bnd(0, div);
    set_bnd(0, p);
    lin_solve(0.0, p, &div, 1.0, 4.0, iter);

    for j in 1..(N - 1) {
        for i in 1..(N - 1) {
            velocx[ix(i, j)] -= 0.5 * (p[ix(i + 1, j)] - p[ix(i - 1, j)]) * N as f32;
            velocy[ix(i, j)] -= 0.5 * (p[ix(i, j + 1)] - p[ix(i, j - 1)]) * N as f32;
        }
    }
    set_bnd(1, velocx);
    set_bnd(2, velocy);
}

pub fn advect(b: f32, d: &mut Vec<f32>, d0: &Vec<f32>, velocx: &Vec<f32>, velocy: &Vec<f32>, dt:
f32) {
    let mut i0 = 0.0;
    let mut i1 = 0.0;
    let mut j0 = 0.0;
    let mut j1 = 0.0;

    let dtx = dt * (N as f32 - 2.0);
    let dty = dt * (N as f32 - 2.0);

    let mut s0 = 0.0;
    let mut s1 = 0.0;
    let mut t0 = 0.0;
    let mut t1 = 0.0;
    let mut tmp1 = 0.0;
    let mut tmp2 = 0.0;
    let mut x = 0.0;
    let mut y = 0.0;

    let nfloat = N as f32;
    let mut ifloat = 0.0;
    let mut jfloat = 0.0;

    let i = 0;
    let j = 0;

    for j in 1..(N - 1) {
        jfloat = j as f32;
        for i in 1..(N - 1) {
            ifloat = i as f32;
            tmp1 = dtx * velocx[ix(i, j)];
            tmp2 = dty * velocy[ix(i, j)];
            x = ifloat - tmp1;
            y = jfloat - tmp2;

            if x < 0.5 {
                x = 0.5;
            }
            if x > nfloat + 0.5 {
                x = nfloat + 0.5;
            }
            i0 = floor(x);
            i1 = i0 + 1.0;
            if y < 0.5 {
                y = 0.5;
            }
            if y > nfloat + 0.5 {
                y = nfloat + 0.5;
            }
            j0 = floor(y);
            j1 = j0 + 1.0;

            s1 = x - i0;
            s0 = 1.0 - s1;
            t1 = y - j0;
            t0 = 1.0 - t1;

            let i0i = i0 as i32;
            let i1i = i1 as i32;
            let j0i = j0 as i32;
            let j1i = j1 as i32;

// DOUBLE CHECK THIS!!!
            d[ix(i, j)] =
                s0 * (t0 * d0[ix(i0i, j0i)] + t1 * d0[ix(i0i, j1i)]) +
                    s1 * (t0 * d0[ix(i1i, j0i)] + t1 * d0[ix(i1i, j1i)]);
        }
    }

    set_bnd(b as i32, d);
}

pub fn set_bnd(b: i32, x: &mut Vec<f32>) {
    for i in 1..(N - 1) {
        //x[ix(i, 0)] = b == 2? - x[ix(i, 1)]: x[ix(i, 1)];
        if b == 2 {
            x[ix(i, 0)] = -x[ix(i, 1)];
        } else {
            x[ix(i, 0)] = x[ix(i, 1)];
        }


        //x[ix(i, N - 1)] = b == 2? - x[ix(i, N - 2)]: x[ix(i, N - 2)];
        if b == 2 {
            x[ix(i, N - 1)] = -x[ix(i, N - 2)];
        } else {
            x[ix(i, N - 1)] = x[ix(i, N - 2)];
        }
    }
    for j in 1..(N - 1) {
        //x[ix(0, j)] = b == 1? - x[ix(1, j)]: x[ix(1, j)];
        if b == 1 {
            x[ix(0, j)] = -x[ix(1, j)];
        } else {
            x[ix(0, j)] = x[ix(1, j)];
        }

        //x[ix(N - 1, j)] = b == 1? - x[ix(N - 2, j)]: x[ix(N - 2, j)];
        if b == 1 {
            x[ix(N - 1, j)] = -x[ix(N - 2, j)];
        } else {
            x[ix(N - 1, j)] = x[ix(N - 2, j)];
        }
    }

    x[ix(0, 0)] = 0.5 * (x[ix(1, 0)] + x[ix(0, 1)]);
    x[ix(0, N - 1)] = 0.5 * (x[ix(1, N - 1)] + x[ix(0, N - 2)]);
    x[ix(N - 1, 0)] = 0.5 * (x[ix(N - 2, 0)] + x[ix(N - 1, 1)]);
    x[ix(N - 1, N - 1)] = 0.5 * (x[ix(N - 2, N - 1)] + x[ix(N - 1, N - 2)]);
}


