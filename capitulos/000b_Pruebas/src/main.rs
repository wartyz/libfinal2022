use sketch::Sketch;

pub mod sketch;

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
