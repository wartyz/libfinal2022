use sketch::Sketch;

// https://www.youtube.com/watch?v=KWoJgHFYWxY&list=PLRqwX-V7Uu6ZiZxtDDRCi6uhfTH4FilpH&index=33

pub mod sketch;

fn main() {
    println!("Hola, world!");

    let (mut rl, th) = raylib::init()
        .msaa_4x()
        .size(sketch::ANCHO, sketch::ALTO)
        .title("Ventana en main")
        .build();

    rl.set_target_fps(60);

    //rl.set_target_fps(1);
    let mut game = Sketch::new();
    game.setup(&mut rl, &th);
    //let mut d = rl.begin_drawing(&th);
    //game.setup();

//    let (mut rl, th) = libfinal::engine::Engine::devuelve_tupla_handle();
//    let d = rl.begin_drawing(&th);


//    game.setup(d);
    // Bucle principal ***********************************************************************
    //'main_loop: loop {
    while !rl.window_should_close() {
        //let mut d = rl.begin_drawing(&th);
        //d.clear_background(Color::WHITE);

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
}

