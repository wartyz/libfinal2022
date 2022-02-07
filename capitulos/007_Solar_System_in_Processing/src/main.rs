use sketch::Sketch;

pub mod sketch;
pub mod planet;

fn main() {
    println!("Hola, world!");
    let ancho = 1200.0;
    let alto = 800.0;


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
    'main_loop: loop {
        // Creamos el único RaylibDrawHandle
        //let mut d = rl.begin_drawing(&th);
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


