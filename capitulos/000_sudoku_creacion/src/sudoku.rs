use crate::posibles::Posibles;
use libfinal::matem::{max, random, random_i32, random_usize};

pub struct Sudoku {
    celdas: [[[usize; 9]; 9]; 9],
    acabado: [[usize; 9]; 9],
}

impl Sudoku {
    // Crea un sudoku con todas las posibilidades
    pub fn new() -> Sudoku {
        let celdas: [[[usize; 9]; 9]; 9] = [[[0; 9]; 9]; 9];
        let acabado: [[usize; 9]; 9] = [[0; 9]; 9];
        Sudoku {
            celdas,
            acabado,
        }
    }

    pub fn getaleatorio(&mut self, fila: usize, columna: usize) -> Option<usize> {
        let v1 = self.celdas[fila][columna];
        //println!("fila = {:#?} columna = {:#?} v = {:#?}", fila, columna, v1);
        let mut v2: Vec<usize> = vec![];
        let mut cont = 0;
        for f in 0..9 {
            if v1[f] != 0 {
                v2.push(v1[f]);
                cont += 1;
            }
        }
        if cont == 0 { return None; }
        let aleato = random_usize(v2.len());
        Some(v2[aleato])
    }

    pub fn rellena_posibles(&mut self) {
        for f in 0..9 {
            for g in 0..9 {
                for h in 0..9 {
                    self.celdas[f][g][h] = h + 1;
                    //self.celdas[f][g][h] = random_usize(9);
                }
            }
        }
        for f in 0..9 {
            for g in 0..9 {
                self.acabado[f][g] = 0;
            }
        }
    }


    // borra todas las incidencias del numero n en la fila g
    pub fn borra_n_en_fila(&mut self, elegido: usize, fila: usize) {
        //println!("grupos hori");
        for columna in 0..9 {
            self.celdas[fila][columna][elegido - 1] = 0;
        }
    }

    // borra todas las incidencias del numero n en la columna g
    pub fn borra_n_en_columna(&mut self, elegido: usize, columna: usize) {
        //println!("grupos vert");
        for fila in 0..9 {
            //println!("en v    g = {}", g);
            //println!("en v    f = {}", f);
            //println!("en v    elegido = {}", elegido);
            self.celdas[fila][columna][elegido - 1] = 0;
        }
    }

    pub fn borra_n_en_cuadrado(&mut self, elegido: usize, fila: usize, columna: usize) {
        // averiguamos a que cuadrado pertenece
        let cuadrado = (fila / 3) * 3 + columna / 3;
        //println!("fila {} y columna {} pertenecen al cuadrado {}", fila, columna, cuadrado);
        // averiguamos la esquina superior izquierda del cuadrado f1   c1
        let f1 = (cuadrado / 3) * 3; // Ojo son operaciones de enteros
        let c1 = (cuadrado * 3) % 9;
        //println!("fila {} columna {} está en f1 {} y c1 {}", fila, columna, f1, c1);
//        self.celdas[f1][c1][elegido - 1] = 0;
//        self.celdas[f1][c1+1][elegido - 1] = 0;
//        self.celdas[f1][c1+2][elegido - 1] = 0;
//
//        self.celdas[f1+1][c1][elegido - 1] = 0;
//        self.celdas[f1+1][c1+1][elegido - 1] = 0;
//        self.celdas[f1+1][c1+2][elegido - 1] = 0;
//
//        self.celdas[f1+2][c1][elegido - 1] = 0;
//        self.celdas[f1+2][c1+1][elegido - 1] = 0;
//        self.celdas[f1+2][c1+2][elegido - 1] = 0;

        for j in 0..3 {
            for k in 0..3 {
                self.celdas[f1 + j][c1 + k][elegido - 1] = 0;
            }
        }
    }

    // Pone fijo un numero en fila columna
    pub fn pone_en_acabado(&mut self, elegido: usize, fila: usize, columna: usize) {
        self.acabado[fila][columna] = elegido;
    }


    pub fn imprime(&mut self) {
        let mut sep1 = 0;
        let mut sep2 = 0;
        let mut s = String::from(" ");
        for f in 0..9 {
            for g in 0..9 {
                for h in 0..9 {
                    //println!("{:?}", self.celdas[f][g][h]);
                    let ichar = ((self.celdas[f][g][h]) as u8 + b'0') as char;
                    s.insert(0, ichar);
                }
                s.insert(0, ' ');
                sep1 += 1;
                if sep1 == 3 {
                    s.insert(0, '+');
                    s.insert(0, ' ');
                    sep1 = 0;
                }
            }
            s.insert(0, '\n');
            sep2 += 1;
            if sep2 == 3 {
                s.insert_str(0, "\n \
                -----------------------------------------------------------------------------------------------");

                sep2 = 0;
            }
        }
        println!("{}", s);
    }

    pub fn imprime2(&mut self) {
        let mut sep1 = 0;
        let mut sep2 = 0;
        let mut s = String::from("");
        for f in 0..9 {
            for g in 0..9 {
                for h in 0..9 {
                    //println!("{:?}", self.celdas[f][g][h]);
                    let ichar = ((self.celdas[f][g][h]) as u8 + b'0') as char;
                    s.push(ichar);
                }
                s.push(' ');
                sep1 += 1;
                if sep1 == 3 {
                    s.push('+');
                    s.push(' ');
                    sep1 = 0;
                }
            }
            s.push('\n');
            sep2 += 1;
            if sep2 == 3 {
                s.push_str("\
                -----------------------------------------------------------------------------------------------\n");

                sep2 = 0;
            }
        }
        println!("{}", s);
    }


    pub fn imprime_acabado(&mut self) {
        println!("***************************");
        for f in 0..9 {
            if f % 3 == 0 {
                println!(" +-------+-------+-------+");
            }
            for g in 0..9 {
                if g % 3 == 0 {
                    print!(" |");
                }
                print!(" {}", self.acabado[g][f]);
            }
            print!(" |");
            print!("\n");
        }
    }

    // Cuenta numero de ceros en el resultado final
    pub fn cuentaceros(&mut self) -> usize {
        let mut z = 0;
        for f in 0..9 {
            for g in 0..9 {
                if self.acabado[g][f] == 0 {
                    z += 1;
                }
            }
        }
        z
    }

    // Devuelve true si no hay repetidos
    pub fn hayrepetidos(&mut self) -> bool {
        // comprueba filas
        let mut v1 = 0;
        let mut v2 = 0;
        for fila in 0..9 {
            for c in 0..9 {
                v1 += self.acabado[fila][c];
            }
            //println!("v1 = {}  v2 = {}", v1, v2);
            if v1 == 45 {
                v2 += 1;
                v1 = 0;
            }
        }
        if v2 == 9 {
            self.imprime_acabado();
            return false;
        }


        true
    }
}