use perlin_noise::PerlinNoise;
use rand::Rng;
use raylib::prelude::*;

use crate::parametros::{ModosAngulo, Parametros};

// Cálculo  **************************************************************

pub fn abs() {}

pub fn ceil() {}

// constrain(n, low, high) Restringe un valor entre un valor mínimo y máximo.
pub fn constrain(n: f32, low: f32, high: f32) -> f32 {
    if n < low {
        return low;
    }
    if n > high {
        return high;
    }
    n
}

// constraini32(n, low, high) Restringe un valor entre un valor mínimo y máximo.
pub fn constraini32(n: i32, low: i32, high: i32) -> i32 {
    if n < low {
        return low;
    }
    if n > high {
        return high;
    }
    n
}

pub fn dist4(x0: f32, y0: f32, x1: f32, y1: f32) -> f32 {
    let x = x0 - x1;
    let y = y0 - y1;
    ((x * x) + (y * y)).sqrt()
}

pub fn exp() {}

pub fn floor(v: f32) -> f32 {
    v.floor()
}

// Interpolación lineal https://en.wikipedia.org/wiki/Linear_interpolation
pub fn lerp(start: f32, stop: f32, amt: f32) -> f32 {
    if amt > 1.0 || amt < 0.0 {
        panic!("Error en matem::lerp");
    }
    (1.0 - amt) * start + amt * stop
}

pub fn log() {}

pub fn mag() {}

pub fn map() {}

pub fn max(a: f32, b: f32) -> f32 {
    if a > b {
        return a;
    } else if a < b {
        return b;
    }
    0.0
}

pub fn min(a: f32, b: f32) -> f32 {
    if a > b {
        return b;
    } else if a < b {
        return a;
    }
    0.0
}

pub fn norm() {}

pub fn pow(a: f32, pot: i32) -> f32 {
    a.powi(pot)
}

//Calcula el entero más cercano al parámetro n. Por ejemplo, round(133.8) retorna el valor 134.
// Usa la función Math.round().
pub fn round(n: f32) -> f32 {
    n.round()
}

pub fn sq() {}

pub fn sqrt() {}

pub fn fract() {}

// Trigronometria  **************************************************************
//pub fn atan2(v1: f32, v2: f32) -> f32 {
//    return float32(math.Atan2(float64(v1), float64(v2)));
//}

pub fn angle_mode(modo: ModosAngulo, param: &mut Parametros) {
    param.angulo_mode = modo;
}

// grados a radianes
pub fn radians(angulo: f32) -> f32 {
    (angulo * PI as f32) / 180.0
}

// radianes a grados
pub fn grados(angulo: f32) -> f32 {
    (angulo * 180.0) / PI as f32
}

// Recibe radianes, devuelve coseno
pub fn cos(valor: f32) -> f32 {
    valor.cos()
}

// Recibe grados devuelve coseno
pub fn cos_gr(valor: f32) -> f32 {
    let valor = valor.to_radians();
    valor.cos()
}

// Recibe radianes, devuelve seno
pub fn sin(valor: f32) -> f32 {
    valor.sin()
}

// Recibe grados devuelve seno
pub fn sin_gr(valor: f32) -> f32 {
    let valor = valor.to_radians();
    valor.sin()
}

// ****************************************************************

pub fn mapa(n: f32, start1: f32, stop1: f32, start2: f32, stop2: f32) -> f32 {
    let fnn = n;
    //    if fnn < start1 {
    ////        fnn = start1;
    ////    }
    ////    if fnn > stop1 {
    ////        fnn = stop2;
    ////    }

    let fstart1 = start1;
    let fstop1 = stop1;
    let fstart2 = start2;
    let fstop2 = stop2;

    let kk = ((fnn - fstart1) / (fstop1 - fstart1)) * (fstop2 - fstart2) + fstart2;

    //println!("en mapa kk = {} n = {}  start1 = {}  stop1= {}, start2 = {} stop2 = {}",
    //kk, n, start1, stop1, start2, stop2);

    kk

    // if devolver < 0 {
    // 	devolver = 0
    // }
    // if devolver > 255 {
    // 	devolver = 255
    // }
}

pub fn random_range(p0: f32, p1: f32) -> f32 {
    if p0 == p1 {
        println!("valores iguales en random_range()");
        p0
    } else {
        let mut rng = rand::thread_rng();
        let v = rng.gen_range(p0, p1);
        //println!("valor v en random_range() = {}", v);
        v
    }
}

pub fn random_u8_range(p0: u8, p1: u8) -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(p0, p1)
}

pub fn random_i32_range(p0: i32, p1: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(p0, p1)
}

pub fn random(p0: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0, p0)
}

pub fn random_u8(p0: u8) -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0, p0)
}

pub fn random_i32(p0: i32) -> i32 {
    let mut rng = rand::thread_rng();
    if p0 == 0 {
        return 0;
    }
    rng.gen_range(0, p0)
}

pub fn random_usize(p0: usize) -> usize {
    if p0 == 0 {
        return 0;
    }
    let mut rng = rand::thread_rng();
    rng.gen_range(0, p0)
}

// Noise ----------------------------------------------

pub fn noise1(v: f32) -> f32 {
    let perlin = PerlinNoise::new();
    perlin.get(v as f64) as f32
}

pub fn noise2(a: f32, b: f32) -> f32 {
    let perlin = PerlinNoise::new();
    perlin.get2d([a as f64, b as f64]) as f32
}

pub fn noise3(a: f32, b: f32, c: f32) -> f32 {
    let perlin = PerlinNoise::new();
    perlin.get3d([a as f64, b as f64, c as f64]) as f32
}

// https://stackoverflow.com/questions/29711668/perlin-noise-generation
//#define maxPrimeIndex 10
const PRIMES: [[i32; 3]; 10] = [
    [995615039, 600173719, 701464987],
    [831731269, 162318869, 136250887],
    [174329291, 946737083, 245679977],
    [362489573, 795918041, 350777237],
    [457025711, 880830799, 909678923],
    [787070341, 177340217, 593320781],
    [405493717, 291031019, 391950901],
    [458904767, 676625681, 424452397],
    [531736441, 939683957, 810651871],
    [997169939, 842027887, 423882827],
];

pub fn noise_1(_na: f32, nb: f32, nc: f32) -> f32 {
    let ii = random_i32(10) as usize; // Lo pongo yo

    let mut nn = (nb + nc * 57.0) as i32;
    nn = (nn << 13) ^ nn;
    let aa = PRIMES[ii][0];
    let bb = PRIMES[ii][1];
    let cc = PRIMES[ii][2];
    let tt = (nn * (nn * nn * aa + bb) + cc) & 0x7fffffff;
    1.0 - tt as f32 / 1073741824.0
}

// 2D simplex noise Elegir este o el anterior
// https://gist.github.com/jackmott/7a85b4ff6120cc7885a22d3e162ce115

const PERM: [u8; 256] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180,
];

pub fn fast_floor(x: f32) -> i32 {
    if ((x as i32) as f32) <= x {
        return x as i32;
    }
    x as i32 - 1
}

pub fn noise_2(x: f32, y: f32) -> f32 {
    const F2: f32 = 0.366_025_4; // F2 = 0.5*(sqrt(3.0)-1.0)
    const G2: f32 = 0.211_324_8; // G2 = (3.0-Math.sqrt(3.0))/6.0

    // Noise contributions from the three corners
    let n0: f32;
    let n1: f32;
    let n2: f32;

    // Skew the input space to determine which simplex cell we're in
    let ss = (x + y) * F2; // Hairy factor for 2D
    let xs = x + ss;
    let ys = y + ss;
    let ii = fast_floor(xs);
    let jj = fast_floor(ys);

    let tt = (ii + jj) as f32 * G2;
    let xx0 = ii as f32 - tt; // Unskew the cell origin back to (x,y) space
    let yy0 = jj as f32 - tt;
    let x0 = x - xx0; // The x,y distances from the cell origin
    let y0 = y - yy0;

    // For the 2D case, the simplex shape is an equilateral triangle.
    // Determine which simplex we are in.
    // Offsets for second (middle) corner of simplex in (i,j) coords
    let i1: u8;
    let j1: u8;
    if x0 > y0 {
        i1 = 1;
        j1 = 0;
    } else {
        // lower triangle, XY order: (0,0)->(1,0)->(1,1)
        i1 = 0;
        j1 = 1;
    } // upper triangle, YX order: (0,0)->(0,1)->(1,1)

    // A step of (1,0) in (i,j) means a step of (1-c,-c) in (x,y), and
    // a step of (0,1) in (i,j) means a step of (-c,1-c) in (x,y), where
    // c = (3-sqrt(3))/6

    let x1 = x0 - i1 as f32 + G2; // Offsets for middle corner in (x,y) unskewed coords
    let y1 = y0 - j1 as f32 + G2;
    let x2 = x0 - 1.0 + 2.0 * G2; // Offsets for last corner in (x,y) unskewed coords
    let y2 = y0 - 1.0 + 2.0 * G2;

    // Wrap the integer indices at 256, to avoid indexing perm[] out of bounds
    let ii = ii as u8;
    let jj = jj as u8;

    // Calculate the contribution from the three corners
    let mut t0 = 0.5 - x0 * x0 - y0 * y0;
    if t0 < 0.0 {
        n0 = 0.0;
    } else {
        t0 *= t0;
        n0 = t0 * t0 * grad2(PERM[ii as usize + PERM[jj as usize] as usize], x0, y0);
    }

    let mut t1 = 0.5 - x1 * x1 - y1 * y1;
    if t1 < 0.0 {
        n1 = 0.0;
    } else {
        t1 *= t1;
        n1 = t1
            * t1
            * grad2(
                PERM[ii as usize + i1 as usize + PERM[jj as usize + j1 as usize] as usize],
                x1,
                y1,
            );
    }

    let mut t2 = 0.5 - x2 * x2 - y2 * y2;
    if t2 < 0.0 {
        n2 = 0.0;
    } else {
        t2 *= t2;
        n2 = t2
            * t2
            * grad2(
                PERM[ii as usize + 1 + PERM[jj as usize + 1] as usize],
                x2,
                y2,
            );
    }

    // Add contributions from each corner to get the final noise value.
    n0 + n1 + n2
}

pub fn grad2(hash: u8, x: f32, y: f32) -> f32 {
    let hh = hash & 7; // Convert low 3 bits of hash code
    let mut uu = y;
    let mut vv = 2.0 * x;
    if hh < 4 {
        uu = x;
        vv = 2.0 * y;
    } // into 8 simple gradient directions,
      // and compute the dot product with (x,y).

    if hh & 1 != 0 {
        uu = -uu;
    }
    if hh & 2 != 0 {
        vv = -vv;
    }
    uu + vv
}

pub fn noise_detail() {
    //fmt.Println("Función noiseDetail() sin hacer")
}

pub fn noise_seed() {
    //fmt.Println("Función noiseSeed() sin hacer")
}
