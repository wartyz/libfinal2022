struct Particle {
    x: f32,
    y: f32,
}

impl Particle {
    pub fn new(x: f32, y: f32) -> Particle {
        Particle {
            x,
            y,
        }
    }
}