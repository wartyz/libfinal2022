use sketch::Sketch;

mod particle;
pub mod sketch;
mod spring;

fn main() {
    println!("Hola, world!");

    let (mut rl, th) = raylib::init()
        .msaa_4x()
        .size(sketch::ANCHO, sketch::ALTO)
        .title("Ventana en main")
        .build();

    rl.set_target_fps(60);

    let mut game = Sketch::new(&mut rl, &th);
    game.setup(&mut rl, &th);
    //game.setup();

    // Bucle principal ***********************************************************************
    //while !rl.window_should_close() {
    'main_loop: loop {
        if !game.update(&mut rl, &th) {
            break 'main_loop;
        }
        // Creamos el único RaylibDrawHandle
        let mut d = rl.begin_drawing(&th);
        game.draw(&mut d);

        //        if !game.update() {
        //            break 'main_loop;
        //        }
        //        game.draw();

        //println!("en main loop");

        //game.key_pressed();
        //game.key_released();
    }
}
