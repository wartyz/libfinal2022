// https://www.youtube.com/watch?v=XaOVH8ZSRNA&list=PLRqwX-V7Uu6ZiZxtDDRCi6uhfTH4FilpH&index=96
// Video 2   acabado

use sketch::Sketch;

pub mod sketch;
pub mod particle;

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


    // Bucle principal ***********************************************************************
    'main_loop: loop {
        if !game.update(&mut rl, &th) {
            break 'main_loop;
        }
        // Creamos el único RaylibDrawHandle
        let mut d = rl.begin_drawing(&th);
        game.draw(&mut d);

        if game.engine.param.mouse_boton == libfinal::parametros::CodigosRaton::Izquierdo {
            game.mouse_pressed();
        }

        //game.key_pressed();
        //game.key_released();
    }
}
