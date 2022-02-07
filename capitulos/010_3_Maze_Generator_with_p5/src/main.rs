use raylib::prelude::*;

use sketch::Sketch;

pub mod sketch;
pub mod cell;

fn main() {
    println!("Hola, world!");

    let (mut rl, th) = raylib::init()
        .msaa_4x()
        .size(sketch::ANCHO, sketch::ALTO)
        .title("Ventana en devuelve_main")
        .build();


    let mut game = Sketch::new();
    game.setup(&mut rl, &th);

//    let (mut rl, th) = libfinal::engine::Engine::devuelve_tupla_handle();
//    let d = rl.begin_drawing(&th);


//    game.setup(d);
    // Bucle principal ***********************************************************************
    //'main_loop: loop {
    while !rl.window_should_close() {
        //let mut d = rl.begin_drawing(&th);
        //d.clear_background(Color::WHITE);

        //if !game.update(&mut d) {
        if !game.update(&mut rl, &th) {
            break;
        }
        let mut d = rl.begin_drawing(&th);
        game.draw(&mut d);
        //println!("en main loop");
        //game.key_pressed();
        //game.key_released();

        //game.engine.canvas.as_mut().unwrap().clear();
        //game.engine.canvas.as_mut().unwrap().present();
    }
}



