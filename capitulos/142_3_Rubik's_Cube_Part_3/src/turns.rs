use crate::cubie::Cubie;
use libfinal::parametros::Parametros;
use libfinal::transform3d::identity4x4;
use std::f32::consts::PI;

// En las funciones turn, se entra al iniciarse el desorden aleatorio de qbits
pub fn turn_z(index: i32, dir: i32, cube: &mut Vec<Cubie>, param: &mut Parametros) {
    for i in 0..cube.len() {
        if cube[i].z == index {
            let mut matrix = identity4x4();
            matrix.rotate_z3d(dir as f32 * PI / 2.0);
            matrix.translate3d(cube[i].x as f32, cube[i].y as f32, 0.0);
            //imprime_matriz_4x4(matrix);
            //self.cube[i].matrix = self.cube[i].matrix * matrix; // ----> lo pongo yo
            let zz = cube[i].z;
            cube[i].update(
                matrix.data[3].round() as i32,
                matrix.data[7].round() as i32,
                zz,
                param,
            );
            cube[i].turn_facesz(dir);
        }
    }
}

pub fn turn_y(index: i32, dir: i32, cube: &mut Vec<Cubie>, param: &mut Parametros) {
    for i in 0..cube.len() {
        //let mut qb = self.cube[i].clone();

        //println!("self.cube[i].z");
        if cube[i].y == index {
            let mut matrix = identity4x4();
            matrix.rotate_z3d(dir as f32 * PI / 2.0);
            matrix.translate3d(cube[i].x as f32, cube[i].z as f32, 0.0);
            //imprime_matriz_4x4(matrix);
            //self.cube[i].matrix = self.cube[i].matrix * matrix; // ----> lo pongo yo
            let yy = cube[i].y;
            cube[i].update(
                matrix.data[3].round() as i32,
                yy,
                matrix.data[7].round() as i32,
                param,
            );
            cube[i].turn_facesy(dir);
        }
    }
}

pub fn turn_x(index: i32, dir: i32, cube: &mut Vec<Cubie>, param: &mut Parametros) {
    for i in 0..cube.len() {
        if cube[i].x == index {
            let mut matrix = identity4x4();
            matrix.rotate_z3d(dir as f32 * PI / 2.0);
            matrix.translate3d(cube[i].y as f32, cube[i].z as f32, 0.0);
            //imprime_matriz_4x4(matrix);
            //self.cube[i].matrix = self.cube[i].matrix * matrix; // ----> lo pongo yo
            let xx = cube[i].x;
            cube[i].update(
                xx,
                matrix.data[3].round() as i32,
                matrix.data[7].round() as i32,
                param,
            );
            cube[i].turn_facesx(dir);
        }
    }
}
