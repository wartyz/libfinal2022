//https://www.youtube.com/watch?v=Y9s2jP2sf3Q
// 13:02
pub mod sketch;
pub mod posibles;
pub mod sudoku;
pub mod ayuda_sudoku;

use sketch::Sketch;

fn main() {
    println!("Hola, world!");
    let mut game = Sketch::new();
    game.setup();

    // Bucle principal ***********************************************************************
    'main_loop: loop {
        if !game.update() {
            break 'main_loop;
        }

        game.draw();
        game.key_pressed();
        game.key_released();


        //game.engine.canvas.as_mut().unwrap().clear();
        game.engine.canvas.as_mut().unwrap().present();
    }
}


