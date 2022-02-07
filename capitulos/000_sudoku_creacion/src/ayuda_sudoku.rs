use libfinal::matem::random_usize;

pub struct Azar {
    num: Vec<usize>
}

impl Azar {
    pub fn new() -> Azar {
        let mut v = vec![];
        for f in 0..9 {
            v.push(f + 1);
        }
        Azar {
            num: v
        }
    }

    pub fn get_azar(&mut self) -> Option<usize> {
        if self.num.len() > 0 {
            let n = random_usize(self.num.len());
            //println!("self.num.len() antes = {}", self.num.len());
            let ret = Some(self.num.remove(n));
            //println!("self.num.len() despues = {}", self.num.len());
            ret
        } else {
            None
        }
    }


    pub fn reinicia_azar(&mut self) {
        for f in 0..9 {
            self.num.push(f + 1);
        }
    }
}