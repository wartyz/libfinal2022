use libfinal::parametros::CodigosRaton;
use sketch::Sketch;

pub mod light;
pub mod sketch;

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
        //let mut d = rl.begin_drawing(&th);

        game.key_pressed();

        game.draw(&mut rl, &th);

        //        if !game.update() {
        //            break 'main_loop;
        //        }
        //        game.draw();

        //println!("en main loop");
        // if game.engine.param.mouse_boton_inicio == CodigosRaton::Izquierdo {
        //     game.mouse_pressed();
        // }

        //game.key_released();
    }
}
