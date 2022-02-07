use crate::posibles::Posibles;
use libfinal::matem::max;

pub struct Sudoku {
    celdas: Vec<Posibles>,
    grupos: Vec<Vec<usize>>,
    grupos_de: Vec<Vec<usize>>,
    vecinos: Vec<Vec<usize>>,
}

impl Sudoku {
    // Recibimos un sudoku en formato texto
    pub fn new(&mut self, s: &str) {
        let mut k = 0;
        //println!("s.len={}", s.len());
        for i in 0..s.len() {
            let ch = (s.chars().nth(i).unwrap());
            //if k >= '0' && k <= '9' {
            if ch.is_ascii_digit() && ch != '0' {
                println!("ch = {}", ch);
                let a = ch.to_digit(10).unwrap() as usize;
                println!("a = {}", a);
                //let b = '0'.to_digit(10).unwrap();
                //println!("b = {}", b);
                if !self.asigna(k, a) {
                    println!("Error en el Sudoku de entrada k={}     Sudoku::new()", k);
                }
                k += 1;
                // ojo error &&  ??
            } else if ch == '0' || ch == '.' {
                k += 1;
            }
        }

        assert!(k == 81);
    }

    pub fn inicializa() -> Sudoku {
        let mut celdas = vec![];
        for f in 0..81 {
            celdas.push(Posibles::new([true, true, true, true, true, true, true, true, true]));
        }

        let mut grupos: Vec<Vec<usize>> = vec![vec![]; 27];
        let mut grupos_de: Vec<Vec<usize>> = vec![vec![]; 81];
        let mut vecinos: Vec<Vec<usize>> = vec![vec![]; 81];

        for i in 0..9 {
            for j in 0..9 {
                let k = (i * 9 + j);
                // Calcula los grupos a los que pertenece la casilla k
                let g = [i, (9 + j), (18 + (i / 3) * 3 + j / 3)];
                // Ahora metemos en los arreglos el valor k
                for x in 0..3 {
                    grupos[g[x]].push(k);
                    grupos_de[k as usize].push(g[x]);
                }
            }
        }

        // Vecinos
        for k in 0..81 {
            // grupos a los que pertenece k
            for x in 0..3 {
                let g = grupos_de[k][x];
                // Casillas de un grupo
                for i in 0..9 {
                    let k2 = grupos[g][i];
                    if k2 != k {
                        vecinos[k].push(k2);
                    }
                }
            }
        }

        Sudoku {
            celdas,
            grupos,
            grupos_de,
            vecinos,
        }
    }

    // Devuelve los numeros posibles de una casilla
    pub fn posibles(&mut self, k: usize) -> Posibles {
        self.celdas[k]
    }

    // Indica si el sudoku está resuelto
    pub fn resuelto(&mut self) -> bool {
        for k in 0..self.celdas.len() {
            if self.celdas[k].num_activos() != 1 {
                return false;
            }
        }
        true
    }

    // en la casilla k sólo deja el valor val eliminando el resto,
    // devolviendo true si ha funcionado bien
    pub fn asigna(&mut self, k: usize, val: usize) -> bool {
        println!("asigna({},{})", k, val);
        for i in 1..=9 {
            if i != val {
                if !self.elimina(k, i) {
                    println!("fallo al eliminar k = {}  e i={}", k, i);
                    return false;
                }
            }
        }
        true
    }

    // en la casilla k de celdas elimina el valor val
    // devolviendo true si ha funcionado bien
    pub fn elimina(&mut self, k: usize, val: usize) -> bool {
        println!("elimina({},{})", k, val);
        if !self.celdas[k].activo(val) {
            return true;
        }
        self.celdas[k].elimina(val);
        let n = self.celdas[k].num_activos();
        if n == 0 {
            return false;
        } else if n == 1 { //Hay un valor activo
            let v2 = self.celdas[k].val();
            // pues quitar de todos los vecinos
            for i in 0..self.vecinos[k].len() {
                //let kp = self.vecinos[k][i];
                if !self.elimina(self.vecinos[k][i], v2) {
                    return false;
                }
            }
        }
        // 3 grupos
        for x in 0..3 {
            let g = self.grupos_de[k][x];
            let mut n = 0;
            let mut k2 = 0;
            for i in 0..9 {
                let kp = self.grupos[g][i];
                if self.celdas[kp].activo(val) {
                    n += 1;
                    k2 = kp;
                }
            }
            if n == 0 {
                return false;
            } else if n == 1 { // Solo queda una casilla en los grupos
                if !self.asigna(k2, val) {
                    return false;
                }
            }
        }
        true
    }

    pub fn escribe(&mut self) {
//        let mut ancho = 2.0;
//        for k in 0..self.celdas.len() {
//            ancho = max(ancho, 1.0 + self.celdas[k].num_activos() as f32)
//        }


        for i in 0..9_usize {
            if i == 3 || i == 6 {
                for _ in 0..3 {
                    print!(" +------------+------------+---------- + ")
                }
                println!();
            }
            for j in 0..9_usize {
                let k = i * 9 + j;
                if j == 3 || j == 6 {
                    print!("| ");
                }

                self.celdas[k].str();
            }
            print!("| ");
            println!("");
        }
    }
}