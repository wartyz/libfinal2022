pub struct Animal {
    pub patas: i32,
    pub ojos: i32,
    pub narices: i32,
}

impl Animal {
    pub fn new() -> Animal {
        Animal {
            patas: 0,
            ojos: 0,
            narices: 0,
        }
    }

    pub fn creaPatas(&mut self, num: i32) -> &mut i32 {
        self.patas = num;
        &mut self.patas
    }

    pub fn creaOjos(&mut self, num: i32) -> &mut i32 {
        self.ojos = num;
        &mut self.ojos
    }

    pub fn creaNarices(&mut self, num: i32) -> &mut i32 {
        self.narices = num;
        &mut self.narices
    }

    pub fn modifica(mut self) -> Self {
        self.patas = 123;
        self.narices = 45;
        self.ojos = 56;
        self
    }
}

fn main() {
    println!("Hello, world!");
    let mut animal = Animal::new();
    let p = animal.creaPatas(5).clone();
    let o: &mut i32 = &mut animal.creaOjos(3).clone();
    let n: &mut i32 = animal.creaNarices(12);

    recibe3(p, o, n);
    println!("p = {}", p);
    println!("o = {}", o);

    println!("animal.patas = {}", animal.patas);
    println!("animal.ojos = {}", animal.ojos);

    animal = animal.modifica();
    println!("2 animal.patas = {}", animal.patas);
    println!("2 animal.narices = {}", animal.narices);
    println!("2 animal.ojos = {}", animal.ojos);

    animal = modificaSolo(animal);

    println!("3 animal.patas = {}", animal.patas);
    println!("3 animal.narices = {}", animal.narices);
    println!("3 animal.ojos = {}", animal.ojos);
}

pub fn recibe3(mut a: i32, b: &mut i32, c: &mut i32) {
    a = 8;
    *b = 7;
    *c = 56;
}

pub fn modificaSolo(mut a: Animal) -> Animal {
    a.patas = 333;
    a.narices = 444;
    a.ojos = 555;
    a
}