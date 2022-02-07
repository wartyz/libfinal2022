use std::ops::Mul;

//use raylib::core::math::{Vector2, Vector3};

// eSTE CREO QUE NO:
// Ojo para el cálculo la matriz es 0  3  6    y el vector  x
//                                  1  4  7                 y
//                                  2  5  8                 1
use crate::p5::vector_p5::Vector3;
use crate::parametros::Parametros;

// USO VECTOR COLUMNA
// Ojo para el cálculo la matriz es 0  1  2    y el vector  x
//                                  3  4  5                 y
//                                  6  7  8                 1

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix2x2 {
    pub data: [f32; 4],
}

impl Matrix2x2 {
    #[rustfmt::skip]
    pub fn new(
        m00: f32, m01: f32,
        m02: f32, m03: f32,

    ) -> Self {
        Matrix2x2 {
            data: [m00, m01,
                   m02, m03,]
        }
    }

    pub fn determinant(&self) -> f32 {
        self.data[0] * self.data[3] - self.data[1] * self.data[2]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix3x3 {
    pub data: [f32; 9],
}

impl Matrix3x3 {
    #[rustfmt::skip]
    pub fn new(
        m00: f32, m01: f32, m02: f32,
        m03: f32, m04: f32, m05: f32,
        m06: f32, m07: f32, m08: f32,
    ) -> Self {
        Matrix3x3 { data: [m00, m01, m02,
                           m03, m04, m05,
                           m06, m07, m08,]
        }
    }

    pub fn submatrix(&self, row: usize, column: usize) -> Matrix2x2 {
        let mut matrix2x2 = identity2x2();
        let mut source_row: usize = 0;
        let mut source_column: usize = 0;
        let mut target_row: usize = 0;
        let mut target_column: usize = 0;

        while target_row < 2 {
            if source_row == row {
                // Skip row to be removed
                source_row += 1;
            }
            while target_column < 2 {
                if source_column == column {
                    // Skip column to be removed
                    source_column += 1;
                }
                matrix2x2.data[target_column + target_row * 2] =
                    self.data[source_column + source_row * 3];

                source_column += 1;
                target_column += 1;
            }
            source_row += 1;
            source_column = 0;
            target_row += 1;
            target_column = 0;
        }
        matrix2x2
    }

    pub fn minor(&self, row: usize, column: usize) -> f32 {
        self.submatrix(row, column).determinant()
    }

    pub fn cofactor(&self, row: usize, column: usize) -> f32 {
        let minor = self.minor(row, column);
        if (row + column) % 2 == 0 {
            // Even value
            minor
        } else {
            -minor
        }
    }
    pub fn determinant(&self) -> f32 {
        let mut determinant = 0.0;
        for column in 0..3 {
            determinant += self.cofactor(0, column) * self.data[column];
        }

        determinant
    }
}

#[rustfmt::skip]
pub fn identity2x2() -> Matrix2x2 {
    Matrix2x2::new(1.0, 0.0, 
                   0.0, 1.0,)
}

#[rustfmt::skip]
pub fn identity3x3() -> Matrix3x3 {
    Matrix3x3::new(1.0, 0.0, 0.0,
                   0.0, 1.0, 0.0,
                   0.0, 0.0, 1.0)
}

// Escala igual en los dos ejes
#[rustfmt::skip]
pub fn scale1(v: f32, param: &mut Parametros) {
    let m = Matrix3x3 { data: [  v, 0.0, 0.0,
                                         0.0,   v, 0.0,
                                         0.0, 0.0, 1.0],
    };
    
    param.matriz_total = m * param.matriz_total;
}

// Translación respecto a los ejes globales
#[rustfmt::skip]
pub fn translate(x: f32, y: f32, param: &mut Parametros) {
    let m= Matrix3x3 { data: [1.0, 0.0,   x,
                                        0.0, 1.0,   y,
                                        0.0, 0.0, 1.0]
    };

    param.matriz_total = param.matriz_total * m;
}

// carga la matriz de rotacion del eje x con una coordenada
#[rustfmt::skip]
pub fn rotate_x(angulo: f32, param: &mut Parametros) {
    let m= Matrix3x3 { data: [1.0,          0.0,           0.0,
                                        0.0, angulo.cos(), -angulo.sin(),
                                        0.0, angulo.sin(),  angulo.cos(),]
    };

    param.matriz_total = param.matriz_total * m;
}

// carga la matriz de rotacion del eje y con una coordenada
#[rustfmt::skip]
pub fn rotate_y(angulo: f32, param: &mut Parametros) {
    let m= Matrix3x3 { data: [angulo.cos(), 0.0, angulo.sin(),
                                                 0.0, 1.0,          0.0,
                                       -angulo.sin(), 0.0, angulo.cos(),]
    };

    param.matriz_total = param.matriz_total * m;
}

// Recibe un ángulo en radianes y carga la matriz de rotacion del eje z
#[rustfmt::skip]
pub fn rotate_z(angulo: f32, param: &mut Parametros) {
    let m= Matrix3x3 { data: [angulo.cos(), -angulo.sin(), 0.0,
                                        angulo.sin(),  angulo.cos(), 0.0,
                                        0.0,                    0.0, 1.0]
    };
    
    param.matriz_total = param.matriz_total * m;
}

// Producto de matrices
impl Mul<Matrix3x3> for Matrix3x3 {
    type Output = Matrix3x3;

    fn mul(self, b: Matrix3x3) -> Self::Output {
        let mut m = identity3x3();

        let a00 = self.data[0];
        let a01 = self.data[1];
        let a02 = self.data[2];
        let a10 = self.data[3];
        let a11 = self.data[4];
        let a12 = self.data[5];
        let a20 = self.data[6];
        let a21 = self.data[7];
        let a22 = self.data[8];

        let b00 = b.data[0];
        let b01 = b.data[1];
        let b02 = b.data[2];
        let b10 = b.data[3];
        let b11 = b.data[4];
        let b12 = b.data[5];
        let b20 = b.data[6];
        let b21 = b.data[7];
        let b22 = b.data[8];

        m.data[0] = a00 * b00 + a01 * b10 + a02 * b20;
        m.data[1] = a00 * b01 + a01 * b11 + a02 * b21;
        m.data[2] = a00 * b02 + a01 * b12 + a02 * b22;
        m.data[3] = a10 * b00 + a11 * b10 + a12 * b20;
        m.data[4] = a10 * b01 + a11 * b11 + a12 * b21;
        m.data[5] = a10 * b02 + a11 * b12 + a12 * b22;
        m.data[6] = a20 * b00 + a21 * b10 + a22 * b20;
        m.data[7] = a20 * b01 + a21 * b11 + a22 * b21;
        m.data[8] = a20 * b02 + a21 * b12 + a22 * b22;
        m
    }
}

impl Mul<Vector3> for Matrix3x3 {
    type Output = Vector3;

    fn mul(self, v: Vector3) -> Self::Output {
        let x = self.data[0] * v.x + self.data[1] * v.y + self.data[2] * v.w;
        let y = self.data[3] * v.x + self.data[4] * v.y + self.data[5] * v.w;
        let w = self.data[6] * v.x + self.data[7] * v.y + self.data[8] * v.w;

        Vector3::new(x, y, w)
    }
}

// Pone la matriz de transformación en la pila de matriz.
pub fn push_matrix(param: &mut Parametros) {
    param.matriz_stack.push(param.matriz_total);
}

pub fn pop_matrix(param: &mut Parametros) {
    param.matriz_total = param.matriz_stack.pop().unwrap();
}

// pub fn push_matrix3d(param: &mut Parametros) {
//     param.matriz_stack3d.push(param.matriz_total3d);
// }
//
// pub fn pop_matrix3d(param: &mut Parametros) {
//     param.matriz_total3d = param.matriz_stack3d.pop().unwrap();
// }

pub fn imprime_matriz_3x3(m: Matrix3x3) {
    println!("┏━━━━━━━━━━━━━━━━━━━━━━━┓");

    println!(
        "┃{0: >6.2}  {1: >6.2}  {2: >6.2} ┃", // ┃ \u{2503}
        m.data[0], m.data[1], m.data[2]
    );
    println!(
        "┃{0: >6.2}  {1: >6.2}  {2: >6.2} ┃", // ┃ \u{2503}
        m.data[3], m.data[4], m.data[5]
    );
    println!(
        "┃{0: >6.2}  {1: >6.2}  {2: >6.2} ┃", // ┃ \u{2503}
        m.data[6], m.data[7], m.data[8]
    );

    println!("┗━━━━━━━━━━━━━━━━━━━━━━━┛");
}

pub fn imprime_vector3(v: Vector3) {
    println!("┏━━━━━━━┓");
    println!("\u{2503}{0: >6.2} \u{2503}", v.x); // ┃      ┃
    println!("\u{2503}{0: >6.2} \u{2503}", v.y); // ┃      ┃
    println!("\u{2503}{0: >6.2} \u{2503}", v.w); // ┃      ┃
    println!("┗━━━━━━━┛");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::p5::vector3d_p5::Vector4;
    use crate::transform3d::{identity4x4, imprime_matriz_4x4, rotate_x3d, Matrix4x4};
    use std::convert::identity;
    use std::f32::consts::PI;

    #[test]
    fn pba() {
        assert_eq!(5, 5);
    }

    #[test]
    #[rustfmt::skip]
    fn construyendo_e_inspeccionando_una_matriz_3x3() {
        let m = Matrix3x3::new(1.0,  2.0,  3.0,
                                         5.5,  6.5,  7.5,
                                         9.0, 10.0, 11.0);

        assert_eq!(m.data[0], 1.0);
        assert_eq!(m.data[1], 2.0);
        assert_eq!(m.data[2], 3.0);
        assert_eq!(m.data[3], 5.5);
        assert_eq!(m.data[4], 6.5);
        assert_eq!(m.data[5], 7.5);
        assert_eq!(m.data[6], 9.0);
        assert_eq!(m.data[7], 10.0);
        assert_eq!(m.data[8], 11.0);
    }

    #[test]
    #[rustfmt::skip]
    fn imprimiendo_una_matriz_3x3() {
        let m = Matrix3x3::new(1.0,  2.0,  3.0,
                                         5.5,  6.5,  7.5,
                                         9.0, 10.0, 11.0);
        imprime_matriz_3x3(m);
    }

    #[test]
    fn imprimiendo_un_vector3() {
        let v = Vector3::new(5.97432, 6.484, 9.55);
        imprime_vector3(v);
    }

    #[test]
    #[rustfmt::skip]
    fn producto_matrix3_por_matrix3_igual_a_matrix3_con_asterisco() {
        let m0 = Matrix3x3::new(1.0,  2.0,  3.0,
                                5.5,  6.5,  7.5,
                                9.0, 10.0, 11.0);

        let m1 = Matrix3x3::new(0.67,  5.0,  7.0,
                                5.88,  6.5,  7.5,
                                4.0,  6.0,  2.0);
       
        println!("    m0     ");
        imprime_matriz_3x3(m0);
        
        println!("    m1     ");
        imprime_matriz_3x3(m1);
        
        println!("   m0 * m1   =");
        imprime_matriz_3x3(m0 * m1);
        
        println!("   m1 * m0   =");
        imprime_matriz_3x3(m1 * m0);
    }
    //    m0 * m1   =
    // ┏━━━━━━━━━━━━━━━━━━━━━━━┓
    // ┃ 24.43   36.00   28.00 ┃
    // ┃ 71.90  114.75  102.25 ┃
    // ┃108.83  176.00  160.00 ┃
    // ┗━━━━━━━━━━━━━━━━━━━━━━━┛
    //    m1 * m0   =
    // ┏━━━━━━━━━━━━━━━━━━━━━━━┓
    // ┃ 91.17  103.84  116.51 ┃
    // ┃109.13  129.01  148.89 ┃
    // ┃ 55.00   67.00   79.00 ┃
    // ┗━━━━━━━━━━━━━━━━━━━━━━━┛

    #[test]
    #[rustfmt::skip]
    fn producto_matrix3_por_vector3_igual_a_vector3_con_asterisco() {
        let m0 = Matrix3x3::new(1.0,  2.0,  3.0,
                                          5.5,  6.5,  7.5,
                                          9.0, 10.0, 11.0);
        let v1= Vector3::new(4.0, 7.0, 3.8);
        
        println!("   m0 * v1   =");
        imprime_vector3(m0 * v1);

    }

    #[test]
    fn calcula_el_determinante_de_una_matriz_2x2() {
        let m = Matrix2x2::new(1.0, 5.0, -3.0, 2.0);
        let expected_result = 17.0;

        let actual_result = m.determinant();

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn una_submatriz_de_2x2_en_una_matriz_de_3x3() {
        let m = Matrix3x3::new(1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, 3.0);
        let expected_result = Matrix2x2::new(-3.0, 2.0, 0.0, 6.0);

        // Borrar fila 0 y columna 2
        let actual_result = m.submatrix(0, 2);

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn a_submatrix_of_a_4x4_matrix_is_a_3x3_matrix() {
        let m = Matrix4x4::new(
            -6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0,
        );
        let expected_result = Matrix3x3::new(-6.0, 1.0, 6.0, -8.0, 8.0, 6.0, -7.0, -1.0, 1.0);

        // Borrar fila 0 y columna 2
        let actual_result = m.submatrix3d(2, 1);

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn calculate_the_minor_of_a_3x3_matrix() {
        let m = Matrix3x3::new(3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0);
        let sub = m.submatrix(1, 0);
        let determinant = sub.determinant();
        let minor = m.minor(1, 0);

        assert_eq!(25.0, determinant);
        assert_eq!(25.0, minor);
    }

    #[test]
    fn calculate_a_cofactor_of_a_3x3_matrix() {
        let m = Matrix3x3::new(3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0);

        let minor1 = m.minor(0, 0);
        let minor2 = m.minor(1, 0);

        let cofactor1 = m.cofactor(0, 0);
        let cofactor2 = m.cofactor(1, 0);

        assert_eq!(-12.0, minor1);
        assert_eq!(-12.0, cofactor1);
        assert_eq!(25.0, minor2);
        assert_eq!(-25.0, cofactor2);
    }

    #[test]
    fn calculate_the_determinant_of_a_3x3_matrix() {
        let m = Matrix3x3::new(1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0);

        let cofactor00 = m.cofactor(0, 0);
        let cofactor01 = m.cofactor(0, 1);
        let cofactor02 = m.cofactor(0, 2);

        let determinant = m.determinant();

        assert_eq!(56.0, cofactor00);
        assert_eq!(12.0, cofactor01);
        assert_eq!(-46.0, cofactor02);

        assert_eq!(-196.0, determinant);
    }

    #[test]
    fn calculating_the_determinant_of_a_4x4_matrix() {
        let m = Matrix4x4::new(
            -2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0, -9.0,
        );

        let cofactor00 = m.cofactor3d(0, 0);
        let cofactor01 = m.cofactor3d(0, 1);
        let cofactor02 = m.cofactor3d(0, 2);
        let cofactor03 = m.cofactor3d(0, 3);

        let determinant = m.determinant3d();

        assert_eq!(690.0, cofactor00);
        assert_eq!(447.0, cofactor01);
        assert_eq!(210.0, cofactor02);
        assert_eq!(51.0, cofactor03);

        assert_eq!(-4071.0, determinant);
    }

    #[test]
    fn testing_an_invertible_matrix_for_invertibility() {
        let m = Matrix4x4::new(
            6.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 6.0, 4.0, -9.0, 3.0, -7.0, 9.0, 1.0, 7.0, -6.0,
        );

        let determinant = m.determinant3d();

        assert_eq!(-2120.0, determinant);
        assert!(m.is_invertible3d());
    }

    #[test]
    fn testing_an_noninvertible_matrix_for_invertibility() {
        let m = Matrix4x4::new(
            -4.0, 2.0, -2.0, -3.0, 9.0, 6.0, 2.0, 6.0, 0.0, -5.0, 1.0, -5.0, 0.0, 0.0, 0.0, 0.0,
        );

        let determinant = m.determinant3d();

        assert_eq!(0.0, determinant);
        assert!(!m.is_invertible3d());
    }

    #[test]
    fn calculating_the_inverse_of_a_4x4_matrix() {
        let m = Matrix4x4::new(
            -5.0, 2.0, 6.0, -8.0, 1.0, -5.0, 1.0, 8.0, 7.0, 7.0, -6.0, -7.0, 1.0, -3.0, 7.0, 4.0,
        );

        let determinant = m.determinant3d();
        let cofactor23 = m.cofactor3d(2, 3);
        let cofactor32 = m.cofactor3d(3, 2);

        let expected_result = Matrix4x4::new(
            0.21805, 0.45113, 0.24060, -0.04511, -0.80827, -1.45677, -0.44361, 0.52068, -0.07895,
            -0.22368, -0.05263, 0.19737, -0.52256, -0.81391, -0.30075, 0.30639,
        );

        let actual_result = m.inverse3d();

        assert_eq!(532.0, determinant);
        assert_eq!(-160.0, cofactor23);
        assert_eq!(-160.0 / 532.0, actual_result.data[14]);

        assert_eq!(105.0, cofactor32);
        assert_eq!(105.0 / 532.0, actual_result.data[11]);

        //assert_eq!(actual_result, expected_result);
        println!("actual_result");
        imprime_matriz_4x4(actual_result);
        println!("expected_result");
        imprime_matriz_4x4(expected_result);
    }

    #[test]
    fn multiplying_a_product_by_its_inverse() {
        let m1 = Matrix4x4::new(
            3.0, -9.0, 7.0, 3.0, 3.0, -8.0, 2.0, -9.0, -4.0, 4.0, 4.0, 1.0, -6.0, 5.0, -1.0, 1.0,
        );

        let m2 = Matrix4x4::new(
            8.0, 2.0, 2.0, 2.0, 3.0, -1.0, 7.0, 0.0, 7.0, 0.0, 5.0, 4.0, 6.0, -2.0, 0.0, 5.0,
        );

        let m3 = m1 * m2;

        let actual_result = m3 * m2.inverse3d();

        //assert_eq!(actual_result, m1);
        println!("actual_result");
        imprime_matriz_4x4(actual_result);
        println!("m1");
        imprime_matriz_4x4(m1);
    }

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let mut transform = identity4x4();
        transform.translate3d(5.0, -3.0, 2.0);
        let p = Vector4::new(-3.0, 4.0, 5.0, 1.0);
        let expected_result = Vector4::new(2.0, 1.0, 7.0, 1.0);

        let actual_result = transform * p;
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        let mut transform = identity4x4();
        transform.translate3d(5.0, -3.0, 2.0);
        let inverse_transform = transform.inverse3d();
        let p = Vector4::new(-3.0, 4.0, 5.0, 1.0);
        let expected_result = Vector4::new(-8.0, 7.0, 3.0, 1.0);

        let actual_result = inverse_transform * p;
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let mut transform = identity4x4();
        transform.translate3d(5.0, -3.0, 2.0);
        let v = Vector4::new(-3.0, 4.0, 5.0, 0.0);
        let expected_result = v;

        let actual_result = transform * v;
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_point() {
        let mut transform = identity4x4();
        transform.scale3_3d(2.0, 3.0, 4.0);
        let p = Vector4::new(-4.0, 6.0, 8.0, 1.0);
        let expected_result = Vector4::new(-8.0, 18.0, 32.0, 1.0);

        let actual_result = transform * p;
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_vector() {
        let mut transform = identity4x4();
        transform.scale3_3d(2.0, 3.0, 4.0);
        let v = Vector4::new(-4.0, 6.0, 8.0, 0.0);
        let expected_result = Vector4::new(-8.0, 18.0, 32.0, 0.0);

        let actual_result = transform * v;
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let mut transform = identity4x4();
        transform.scale3_3d(2.0, 3.0, 4.0);

        let inverse_transform = transform.inverse3d();
        let v = Vector4::new(-4.0, 6.0, 8.0, 0.0);
        let expected_result = Vector4::new(-2.0, 2.0, 2.0, 0.0);

        let actual_result = inverse_transform * v;
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let mut transform = identity4x4();
        transform.scale3_3d(-1.0, 1.0, 1.0);
        let p = Vector4::new(2.0, 3.0, 4.0, 1.0);
        let expected_result = Vector4::new(-2.0, 3.0, 4.0, 1.0);

        let actual_result = transform * p;
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn rotating_a_point_around_the_x_axis() {
        let mut half_quarter = identity4x4();
        half_quarter.rotate_x3d(PI / 4.0);

        let mut full_quarter = identity4x4();
        full_quarter.rotate_x3d(PI / 2.0);

        let p = Vector4::new(0.0, 1.0, 0.0, 1.0);

        assert_eq!(
            half_quarter * p,
            Vector4::new(
                0.0,
                (2.0 as f32).sqrt() / 2.0,
                (2.0 as f32).sqrt() / 2.0,
                1.0
            )
        );

        dbg!(full_quarter * p);
        //assert_eq!(full_quarter * p, Vector4::new(0.0, 0.0, 1.0, 1.0));
    }
}
