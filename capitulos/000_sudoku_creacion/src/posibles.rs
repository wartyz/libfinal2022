use std::fmt;
use std::io::Write;
use std::str;

#[derive(Debug, Copy, Clone)]
pub struct Posibles {
    b: [bool; 9],
}

impl Posibles {
    pub fn new(b: [bool; 9]) -> Posibles {
        Posibles { b }
    }

    pub fn activo(&mut self, v: usize) -> bool {
        self.b[v - 1]
    }
    pub fn elimina(&mut self, v: usize) {
        println!("v = {}", v);
        self.b[v - 1] = false;
    }

    // Devuelve valores true (activos)
    pub fn num_activos(&mut self) -> usize {
        self.b.iter().filter(|&n| *n == true).count()
    }

    // Devuelve la primera posición por la izquierda de valor true
    pub fn val(&mut self) -> usize {
        let pos = self.b.iter().position(|n| n == &true);
        println!("en val pos = {:#?}", pos.unwrap() + 1);
        pos.unwrap() + 1
    }

    pub fn str(&mut self) {
        let mut s = String::from(" ");
        for i in (1..=9_u8).rev() {
            if self.activo(i as usize) {
                let ichar = (i + b'0') as char;
                s.insert(0, ichar);
            } else {
                s.insert(0, ' ');
            }
        }
        print!(" | {}", s);
    }
}
