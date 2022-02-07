use sketch::Sketch;

pub mod sketch;
pub mod cell;

fn main() {
    println!("Hola, world!");

    let (mut rl, th) = raylib::init()
        .msaa_4x()
        .size(sketch::ANCHO, sketch::ALTO)
        .title("Ventana en main")
        .build();

    rl.set_target_fps(60);

    let mut game = Sketch::new();
    game.setup(&mut rl, &th);

//    let (mut rl, th) = libfinal::engine::Engine::devuelve_tupla_handle();
//    let d = rl.begin_drawing(&th);


//    game.setup(d);
    // Bucle principal ***********************************************************************
    //'main_loop: loop {
    'main_loop: loop {
        if !game.update(&mut rl, &th) {
            break 'main_loop;
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
