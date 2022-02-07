//use sdl2::Sdl;

//use crate::engine::Engine;

//pub fn createcanvas(engine: &mut Engine, ancho: i32, alto: i32)
//                    -> (raylib::RaylibHandle, raylib::RaylibThread) {
// Inicializamos ventana
//    engine.param.ancho = ancho as f32;
//    engine.param.alto = alto as f32;

//    let titulo = "Ventanita";
//
//    let mut rl_builder = raylib::init();
//    let (mut rl, thread) = rl_builder
//        .size(ancho, alto)
//        .title(titulo)
//        .build();
//    //let logo = raylib::prelude::Image::load_image("static/logo.png").unwrap();
//    //rl.set_window_icon(&logo);
//    let fps = 60; // OJOOOOOOOOOOOOOO
//    rl.set_target_fps(fps);
//    (rl, thread)
//    let mut rl_builder = raylib::init();
//    let rl_builder = rl_builder.size(640, 480);
//    let rl_builder = rl_builder.title("Hola mundo");
//
//    let (mut rl, thread) = rl_builder.build();

//let mut d = rl.begin_drawing(&thread);

//engine.rl = Some(rl);
//engine.thread = Some(thread);

//let sdl_context = sdl2::init().unwrap();

//let video_subsystem = sdl_context.video().unwrap();

//    let window = video_subsystem
//        .window(titulo, ancho, alto)
//        .position_centered()
//        .opengl()
//        .build()
//        .map_err(|e| e.to_string())?;
//
//    engine.canvas = Some(
//        window
//            .into_canvas()
//            .present_vsync()
//            .accelerated()
//            .build()
//            .map_err(|e| e.to_string())?,
//    );
//    //engine.canvas.unwrap().present();
//
//    let c = sdl2::pixels::Color::RGB(
//        engine.param.color_background.r,
//        engine.param.color_background.g,
//        engine.param.color_background.b);

//    engine
//        .canvas
//        .as_mut()
//        .unwrap()
//        .set_draw_color(c);
//
//    //engine.canvas.as_mut().unwrap().clear();
//    engine.canvas.as_mut().unwrap().present();
//    println!("rendering 4");
//
//    engine.sdl_context = Some(sdl_context);

// Ok(())
//}
