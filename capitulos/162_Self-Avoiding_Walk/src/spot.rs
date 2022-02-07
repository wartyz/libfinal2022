use libfinal::matem::random_usize;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Step {
    dx: f32,
    dy: f32,
    tried: bool,
}

impl Step {
    pub fn new(dx: f32, dy: f32) -> Self {
        Self {
            dx,
            dy,
            tried: false,
        }
    }
}

fn all_options() -> Vec<Step> {
    vec![
        Step::new(1.0, 0.0),
        Step::new(-1.0, 0.0),
        Step::new(0.0, 1.0),
        Step::new(0.0, -1.0),
    ]
}
#[derive(Debug, PartialEq)]
pub struct Spot {
    i: usize,
    j: usize,
    pub x: f32,
    pub y: f32,
    options: Vec<Step>,
    pub visited: bool,
}

impl Clone for Spot {
    fn clone(&self) -> Self {
        let mut v = vec![];
        for i in 0..self.options.len() {
            v.push(self.options[i]);
        }
        Spot {
            i: self.i,
            j: self.j,
            x: self.x,
            y: self.y,
            options: v,
            visited: self.visited,
        }
    }
}

// impl Default for Spot {
//     fn default() -> Self {
//         Self {
//             i: 0,
//             j: 0,
//             x: 0.0,
//             y: 0.0,
//             options: vec![],
//             visited: false,
//         }
//     }
// }

impl Spot {
    pub fn new(i: usize, j: usize, spacing: f32) -> Self {
        Self {
            i,
            j,
            x: i as f32 * spacing,
            y: j as f32 * spacing,
            options: all_options(),
            visited: false,
        }
    }

    pub fn clear(&mut self) {
        self.visited = false;
        self.options = all_options();
    }

    // La pongo aqui
    fn es_valido(
        &self,
        i: usize,
        j: usize,
        rows: usize,
        cols: usize,
        grid: &Vec<Vec<Spot>>,
    ) -> bool {
        if i < 0 || i >= cols || j < 0 || j >= rows {
            return false;
        }

        !grid[i][j].visited
    }

    pub fn next_spot(&self, rows: usize, cols: usize, grid: &Vec<Vec<Spot>>) -> Option<Spot> {
        let mut valid_options = vec![];
        for option in self.options.iter() {
            let new_x = self.i as f32 + option.dx;
            let new_y = self.j as f32 + option.dy;

            if self.es_valido(new_x as usize, new_y as usize, rows, cols, grid) & !option.tried {
                valid_options.push(option);
            }
        }

        if valid_options.len() > 0 {
            let kk = random_usize(valid_options.len());
            let mut step: Step = *valid_options[kk];
            step.tried = true;
            return Some(grid[self.i + step.dx as usize][self.j + step.dy as usize].clone());
        }

        return None;
    }
}
