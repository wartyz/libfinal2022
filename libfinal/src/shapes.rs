use crate::p5::vector3d_p5::Vector4;
use crate::p5::vector_p5::Vector3;
use crate::parametros::*;
use raylib::prelude::*;

//use crate::p5::color::Color;

// https://rust-sdl2.github.io/rust-sdl2/sdl2/gfx/primitives/trait.DrawRenderer.html

/******************************************************
                 PRIMITIVAS 2D
******************************************************/
// Draw a piece of a circle FUNCION PROVISIONAL
pub fn arc(
    param: &mut Parametros,
    d: &mut RaylibDrawHandle,
    x_vieja: f32,
    y_vieja: f32,
    ancho: f32,
    alto: f32,
    start: f32, // angulo inicial
    stop: f32,  // angulo final
) {
    let p = param.matriz_total * Vector3::new(x_vieja, y_vieja, 1.0); // punto w = 1

    if param.fill_bool {
        d.draw_circle_sector(
            raylib::prelude::Vector2::new(p.x, p.y),
            ancho, // OJO provisional, solo uso ancho
            start,
            stop,
            100, // OJO provisional
            param.stroke_color.to_color_raylib(),
        );
    }
    if param.stroke_bool {
        d.draw_circle_sector_lines(
            raylib::prelude::Vector2::new(p.x, p.y),
            ancho, // OJO provisional, solo uso ancho
            start,
            stop,
            100, // OJO provisional
            param.stroke_color.to_color_raylib(),
        );

        // Si hay ancho añade alternativamente fuera/dentro otro circulo
        if param.stroke_weight > 1.0 {
            let kk = param.stroke_weight as usize;
            let mut signo = 1;
            let mut nn: f32;
            for i in 1..(kk + 1) {
                nn = (((i + 1) / 2) as i32 * signo) as f32;
                let radioa = (ancho / 2.0) + nn;
                let _radiob = (alto / 2.0) + nn;
                signo = signo * -1;
                d.draw_circle_sector_lines(
                    raylib::prelude::Vector2::new(p.x, p.y),
                    radioa,
                    start,
                    stop,
                    100, // OJO provisional
                    param.stroke_color.to_color_raylib(),
                );
            }
        }
    }
}

// Dibuja una elipse ancho y alto son diametros
pub fn ellipse(
    param: &mut Parametros,
    d: &mut RaylibDrawHandle,
    x_vieja: f32,
    y_vieja: f32,
    ancho: f32,
    alto: f32,
) {
    let p = param.matriz_total * Vector3::new(x_vieja, y_vieja, 1.0); // punto w = 1

    if param.fill_bool {
        d.draw_ellipse(
            p.x as i32,
            p.y as i32,
            ancho / 2.0,
            alto / 2.0,
            param.fill_color.to_color_raylib(),
        );
    }
    if param.stroke_bool {
        d.draw_ellipse_lines(
            p.x as i32,
            p.y as i32,
            ancho / 2.0,
            alto / 2.0,
            param.stroke_color.to_color_raylib(),
        );

        // Si hay ancho añade alternativamente fuera/dentro otra elipse
        if param.stroke_weight > 1.0 {
            let kk = param.stroke_weight as usize;
            let mut signo = 1;
            let mut nn: f32;
            for i in 1..(kk + 1) {
                nn = (((i + 1) / 2) as i32 * signo) as f32;
                let radioa = (ancho / 2.0) + nn;
                let radiob = (alto / 2.0) + nn;
                signo = signo * -1;
                d.draw_ellipse_lines(
                    p.x as i32,
                    p.y as i32,
                    radioa,
                    radiob,
                    param.stroke_color.to_color_raylib(),
                );
            }
        }
    }
}

pub fn circle(
    param: &mut Parametros,
    d: &mut RaylibDrawHandle,
    x_vieja: f32,
    y_vieja: f32,
    radio: f32,
) {
    ellipse(param, d, x_vieja, y_vieja, radio, radio);
}

/// Dibuja una linea
pub fn line(
    d: &mut RaylibDrawHandle,
    param: &mut Parametros,
    x0_vieja: f32,
    y0_vieja: f32,
    x1_vieja: f32,
    y1_vieja: f32,
) {
    //let mut d = &mut engine.rl.begin_drawing(&engine.th);
    let p0 = param.matriz_total * Vector3::new(x0_vieja, y0_vieja, 1.0); // Punto w = 1
    let p1 = param.matriz_total * Vector3::new(x1_vieja, y1_vieja, 1.0);

    d.draw_line_ex(
        p0.to_v2_raylib(),
        p1.to_v2_raylib(),
        param.stroke_weight,
        raylib::color::Color::new(
            param.stroke_color.r,
            param.stroke_color.g,
            param.stroke_color.b,
            param.stroke_color.a,
        ),
    );
}

pub fn point(d: &mut RaylibDrawHandle, param: &mut Parametros, x_vieja: f32, y_vieja: f32) {
    let p = param.matriz_total * Vector3::new(x_vieja, y_vieja, 1.0);

    d.draw_circle_v(
        Vector2::new(p.x, p.y),
        param.stroke_weight / 2.0,
        raylib::prelude::color::Color::new(
            param.stroke_color.r,
            param.stroke_color.g,
            param.stroke_color.b,
            param.stroke_color.a,
        ),
    )
}

// Dibuja un cuadrilátero
pub fn quad(
    d: &mut RaylibDrawHandle,
    param: &mut Parametros,
    x1_vieja: f32,
    y1_vieja: f32,
    x2_vieja: f32,
    y2_vieja: f32,
    x3_vieja: f32,
    y3_vieja: f32,
    x4_vieja: f32,
    y4_vieja: f32,
) {
    let p1 = param.matriz_total * Vector3::new(x1_vieja, y1_vieja, 1.0);
    let p2 = param.matriz_total * Vector3::new(x2_vieja, y2_vieja, 1.0);
    let p3 = param.matriz_total * Vector3::new(x3_vieja, y3_vieja, 1.0);
    let p4 = param.matriz_total * Vector3::new(x4_vieja, y4_vieja, 1.0);

    if param.fill_bool {
        d.draw_triangle(
            // Ojo, orden en sentido contrario a las agujas del reloj
            p3.to_v2_raylib(),
            p2.to_v2_raylib(),
            p1.to_v2_raylib(),
            param.fill_color.to_color_raylib(),
        );
        d.draw_triangle(
            // Ojo, orden en sentido contrario a las agujas del reloj
            p1.to_v2_raylib(),
            p4.to_v2_raylib(),
            p3.to_v2_raylib(),
            param.fill_color.to_color_raylib(),
        );
    }

    if param.stroke_bool {
        line(d, param, x1_vieja, y1_vieja, x2_vieja, y2_vieja);
        line(d, param, x2_vieja, y2_vieja, x3_vieja, y3_vieja);
        line(d, param, x3_vieja, y3_vieja, x4_vieja, y4_vieja);
        line(d, param, x4_vieja, y4_vieja, x1_vieja, y1_vieja);
    }
}

// Dibuja un rectángulo
pub fn rect(
    d: &mut RaylibDrawHandle,
    param: &mut Parametros,
    x_vieja: f32,
    y_vieja: f32,
    ancho: f32,
    alto: f32,
) {
    //    if param.rect_mode == RectMode::CENTER {
    //        x_vieja -= ancho / 2.0;
    //        y_vieja -= alto / 2.0;
    //    }
    //
    //    // Para la escala aplicamos matriz escala al ancho y alto
    //    //println!("en shapes:ellipse antes ancho = {:#?} alto = {:#?}", ancho, alto);
    //    let r = product_matrix_v3_columna(&param.matriz_escala, &Vector2::new(ancho, alto));
    //println!("En shapes:rect  ");
    //
    let p = param.matriz_total * Vector3::new(x_vieja, y_vieja, 0.0);

    if param.fill_bool {
        d.draw_rectangle(
            p.x as i32,
            p.y as i32,
            ancho as i32,
            alto as i32,
            param.fill_color.to_color_raylib(),
        );
    }
    if param.stroke_bool {
        d.draw_rectangle_lines_ex(
            Rectangle::new(p.x, p.y, ancho, alto),
            param.stroke_weight as i32,
            param.stroke_color.to_color_raylib(),
        );
    }
}

// Dibuja un cuadrado, no están puestos valores opcionales
pub fn square(
    d: &mut RaylibDrawHandle,
    param: &mut Parametros,
    x_vieja: f32,
    y_vieja: f32,
    lado: f32,
) {
    rect(d, param, x_vieja, y_vieja, lado, lado);
}

// Dibuja un triangulo, no están puestos valores opcionales
pub fn triangle(
    d: &mut RaylibDrawHandle,
    param: &mut Parametros,
    x1_vieja: f32,
    y1_vieja: f32,
    x2_vieja: f32,
    y2_vieja: f32,
    x3_vieja: f32,
    y3_vieja: f32,
) {
    let p1 = param.matriz_total * Vector3::new(x1_vieja, y1_vieja, 1.0);
    let p2 = param.matriz_total * Vector3::new(x2_vieja, y2_vieja, 1.0);
    let p3 = param.matriz_total * Vector3::new(x3_vieja, y3_vieja, 1.0);

    if param.fill_bool {
        d.draw_triangle(
            // Ojo, orden en sentido contrario a las agujas del reloj
            p3.to_v2_raylib(),
            p2.to_v2_raylib(),
            p1.to_v2_raylib(),
            param.fill_color.to_color_raylib(),
        );
    }
    if param.stroke_bool {
        d.draw_triangle_lines(
            p1.to_v2_raylib(),
            p2.to_v2_raylib(),
            p3.to_v2_raylib(),
            param.stroke_color.to_color_raylib(),
        );
    }
}

/******************************************************
                 VERTEX
******************************************************/
pub fn begin_contour() {
    todo!("no impl");
}

pub fn begin_shape(_shape: ModosBeginShape) {}

pub fn bezier_vertex() {
    todo!("no impl");
}

pub fn curve_vertex() {
    todo!("no impl");
}

pub fn end_contour() {
    todo!("no impl");
}

pub fn end_shape(d: &mut RaylibDrawHandle, param: &mut Parametros, modo_close: ModosBeginShape) {
    if param.stroke_bool {
        if modo_close == ModosBeginShape::Close {
            param.vectores.push(param.vectores[0]);
        }

        d.draw_line_strip(&param.vectores, param.stroke_color.to_color_raylib());
    }

    if param.fill_bool {}

    param.vectores = vec![];
}

pub fn quadratic_vertex() {
    todo!("no impl");
}

pub fn vertex(x_vieja: f32, y_vieja: f32, param: &mut Parametros) {
    let p0 = param.matriz_total * Vector3::new(x_vieja, y_vieja, 1.0); // w = 1 es punto

    param.vectores.push(Vector2::new(p0.x, p0.y));
}

/******************************************************
                 ATRIBUTOS
******************************************************/

pub fn ellipse_mode() {
    todo!("no impl");
}

pub fn no_smooth() {
    todo!("no impl");
}

pub fn rect_mode(mode: RectMode, param: &mut Parametros) {
    param.rect_mode = mode;
}

pub fn smooth() {
    todo!("no impl");
}

pub fn stroke_cap() {
    todo!("no impl");
}

pub fn stroke_join() {
    todo!("no impl");
}

pub fn stroke_weight(sw: f32, param: &mut Parametros) {
    param.stroke_weight = sw;
}

/******************************************************
                 PRIMITIVAS 3D
******************************************************/
pub fn point3d(d: &mut RaylibDrawHandle, param: &mut Parametros, x: f32, y: f32, z: f32) {
    let camera = Camera3D::perspective(
        raylib::prelude::Vector3::new(param.camara.posx, param.camara.posy, param.camara.posz), // posición
        raylib::prelude::Vector3::new(param.camara.objx, param.camara.objy, param.camara.objz), // objetivo
        // raylib::prelude::Vector3::new(pcamara.x, pcamara.y, pcamara.z), // posición
        // raylib::prelude::Vector3::new(ocamara.x, ocamara.y, ocamara.z), // objetivo
        raylib::prelude::Vector3::new(param.camara.upx, param.camara.upy, param.camara.upz), // up
        param.camara.fovy,
    );
    let p1 = param.matriz_total3d * Vector4::new(x, y, z, 1.0);

    // Modo 3D
    let mut d3 = d.begin_mode3D(camera);
    // //dbg!(p1);
    d3.draw_sphere(
        p1.to_v3_raylib(),
        param.stroke_weight / 2.0,
        param.stroke_color.to_color_raylib(),
    );
}

pub fn plane() {}

pub fn box_shape(
    d: &mut RaylibDrawHandle,
    param: &mut Parametros,
    position: Vector4,
    ancho: f32,
    _alto: f32,
    _profundidad: f32,
) {
    // let mut camera = Camera3D::perspective(
    //     raylib::prelude::Vector3::new(200.0, 300.0, 500.0), // posición
    //     raylib::prelude::Vector3::new(0.0, 0.0, 0.0),       // objetivo
    //     raylib::prelude::Vector3::new(0.0, 1.0, 0.0),       // up
    //     45.0,
    // );

    // let pcamara = param.matriz_total3d
    //     * Vector4::new(param.camara.posx, param.camara.posy, param.camara.posz, 0.0);
    // let ocamara = param.matriz_total3d
    //     * Vector4::new(param.camara.objx, param.camara.objy, param.camara.objz, 0.0);

    let camera = Camera3D::perspective(
        raylib::prelude::Vector3::new(param.camara.posx, param.camara.posy, param.camara.posz), // posición
        raylib::prelude::Vector3::new(param.camara.objx, param.camara.objy, param.camara.objz), // objetivo
        // raylib::prelude::Vector3::new(pcamara.x, pcamara.y, pcamara.z), // posición
        // raylib::prelude::Vector3::new(ocamara.x, ocamara.y, ocamara.z), // objetivo
        raylib::prelude::Vector3::new(param.camara.upx, param.camara.upy, param.camara.upz), // up
        param.camara.fovy,
    );
    let p1 = param.matriz_total3d * Vector4::new(position.x, position.y, position.z, 1.0);

    // Experimento, aplico (0,0) de la matriz total3d porque es el parametro de tamaño y es igual
    // en los tres ejes al ser un cubo
    // let medida = ancho * param.matriz_total3d.data[0];
    // dbg!(medida);
    let medida = ancho;

    // Modo 3D
    let mut d3 = d.begin_mode3D(camera);
    // //dbg!(p1);
    d3.draw_cube(
        p1.to_v3_raylib(),
        medida,
        medida,
        medida,
        param.fill_color.to_color_raylib(),
    );

    // //fill(Color::new(190, 33, 55, 255), &mut self.engine.param); //MAROON
    d3.draw_cube_wires(
        p1.to_v3_raylib(),
        medida,
        medida,
        medida,
        param.stroke_color.to_color_raylib(),
    );

    //grid(&mut d3, 10.0, 1.0);
}

pub fn box_hilos_shape(
    d: &mut RaylibDrawHandle,
    param: &mut Parametros,
    position: Vector4,
    ancho: f32,
    alto: f32,
    profundidad: f32,
) {
    let camera = Camera3D::perspective(
        raylib::prelude::Vector3::new(param.camara.posx, param.camara.posy, param.camara.posz), // posición
        raylib::prelude::Vector3::new(param.camara.objx, param.camara.objy, param.camara.objz), // objetivo
        raylib::prelude::Vector3::new(param.camara.upx, param.camara.upy, param.camara.upz), // up
        param.camara.fovy,
    );

    {
        // Modo 3D
        let mut d3 = d.begin_mode3D(camera);

        //fill(Color::new(255, 0, 0, 255), param); //RED

        // d3.draw_cube(
        //     position.to_v3_raylib(),
        //     ancho,
        //     alto,
        //     profundidad,
        //     param.fill_color.to_color_raylib(),
        // );

        //fill(Color::new(190, 33, 55, 255), &mut self.engine.param); //MAROON
        d3.draw_cube_wires(
            position.to_v3_raylib(),
            ancho,
            alto,
            profundidad,
            param.fill_color.to_color_raylib(),
        );

        //grid(&mut d3, 10.0, 1.0);
    } // Fin modo 3D
}

pub fn sphere() {}

pub fn cylinder() {}

pub fn cone() {}

pub fn ellipsoid() {}

pub fn torus() {}

// Funciones solo de raylib
pub fn grid(d: &mut RaylibMode3D<RaylibDrawHandle>, x: f32, spacing: f32) {
    d.draw_grid(x as i32, spacing);
}

pub fn draw_linea_3d(d: &mut RaylibDrawHandle, param: &mut Parametros, va: Vector4, vb: Vector4) {
    let camera = Camera3D::perspective(
        raylib::prelude::Vector3::new(param.camara.posx, param.camara.posy, param.camara.posz), // posición
        raylib::prelude::Vector3::new(param.camara.objx, param.camara.objy, param.camara.objz), // objetivo
        raylib::prelude::Vector3::new(param.camara.upx, param.camara.upy, param.camara.upz), // up
        param.camara.fovy,
    );

    let p1 = param.matriz_total3d * Vector4::new(va.x, va.y, va.z, 1.0);
    let p2 = param.matriz_total3d * Vector4::new(vb.x, vb.y, vb.z, 1.0);

    let mut d3 = d.begin_mode3D(camera);
    d3.draw_line_3D(
        p1.to_v3_raylib(),
        p2.to_v3_raylib(),
        param.stroke_color.to_color_raylib(),
    );
}

// Sentido contrario a las agujas de reloj
pub fn draw_triangulo_3d(
    d: &mut RaylibDrawHandle,
    param: &mut Parametros,
    va: Vector4,
    vb: Vector4,
    vc: Vector4,
    //col: Color,
) {
    let camera = Camera3D::perspective(
        raylib::prelude::Vector3::new(param.camara.posx, param.camara.posy, param.camara.posz), // posición
        raylib::prelude::Vector3::new(param.camara.objx, param.camara.objy, param.camara.objz), // objetivo
        raylib::prelude::Vector3::new(param.camara.upx, param.camara.upy, param.camara.upz), // up
        param.camara.fovy,
    );

    let p1 = param.matriz_total3d * Vector4::new(va.x, va.y, va.z, 1.0);
    let p2 = param.matriz_total3d * Vector4::new(vb.x, vb.y, vb.z, 1.0);
    let p3 = param.matriz_total3d * Vector4::new(vc.x, vc.y, vc.z, 1.0);

    // Modo 3D

    if param.fill_bool {
        let mut d3 = d.begin_mode3D(camera);
        d3.draw_triangle3D(
            // lado 1
            p1.to_v3_raylib(),
            p2.to_v3_raylib(),
            p3.to_v3_raylib(),
            param.fill_color.to_color_raylib(),
        );
        d3.draw_triangle3D(
            // lado 2
            p3.to_v3_raylib(),
            p2.to_v3_raylib(),
            p1.to_v3_raylib(),
            param.fill_color.to_color_raylib(),
        );
    } else {
        draw_linea_3d(d, param, va, vb);
        draw_linea_3d(d, param, vb, vc);
        draw_linea_3d(d, param, vc, va);
    }
}

// (0.0) en el centro del cuadrado
pub fn cuadrado_2d_en_3d(
    d: &mut RaylibDrawHandle,
    param: &mut Parametros,
    x: f32,
    y: f32,
    lado: f32,
) {
    let ladom = lado / 2.0;
    let va = Vector4::new(x - ladom, y - ladom, 0.0, 1.0);
    let vb = Vector4::new(x + ladom, y - ladom, 0.0, 1.0);
    let vc = Vector4::new(x + ladom, y + ladom, 0.0, 1.0);
    let vd = Vector4::new(x - ladom, y + ladom, 0.0, 1.0);

    draw_rectangulo_3d(d, param, va, vb, vc, vd);
}

// Sentido contrario a las agujas de reloj
pub fn draw_rectangulo_3d(
    d: &mut RaylibDrawHandle,
    param: &mut Parametros,
    va: Vector4,
    vb: Vector4,
    vc: Vector4,
    vd: Vector4,
) {
    // draw_triangulo_3d(d, param, va, vb, vc);
    // draw_triangulo_3d(d, param, vc, vd, va);
    // draw_linea_3d(d, param, va, vb);
    // draw_linea_3d(d, param, vb, vc);
    // draw_linea_3d(d, param, vc, vd);
    // draw_linea_3d(d, param, vd, va);

    // let camera = Camera3D::perspective(
    //     raylib::prelude::Vector3::new(param.camara.posx, param.camara.posy, param.camara.posz), // posición
    //     raylib::prelude::Vector3::new(param.camara.objx, param.camara.objy, param.camara.objz), // objetivo
    //     raylib::prelude::Vector3::new(param.camara.upx, param.camara.upy, param.camara.upz), // up
    //     param.camara.fovy,
    // );

    let camera = param.convierte_a_3d_raylib_persp();

    let p1 = param.matriz_total3d * Vector4::new(va.x, va.y, va.z, 1.0);
    let p2 = param.matriz_total3d * Vector4::new(vb.x, vb.y, vb.z, 1.0);
    let p3 = param.matriz_total3d * Vector4::new(vc.x, vc.y, vc.z, 1.0);
    let p4 = param.matriz_total3d * Vector4::new(vd.x, vd.y, vd.z, 1.0);
    // Modo 3D

    if param.fill_bool {
        let mut d3 = d.begin_mode3D(camera);
        d3.draw_triangle3D(
            // lado 1
            p1.to_v3_raylib(),
            p2.to_v3_raylib(),
            p3.to_v3_raylib(),
            param.fill_color.to_color_raylib(),
        );
        d3.draw_triangle3D(
            // lado 2
            p3.to_v3_raylib(),
            p2.to_v3_raylib(),
            p1.to_v3_raylib(),
            param.fill_color.to_color_raylib(),
        );
        // Segundo triangulo ------------------------------------------------
        d3.draw_triangle3D(
            // lado 1
            p3.to_v3_raylib(),
            p4.to_v3_raylib(),
            p1.to_v3_raylib(),
            param.fill_color.to_color_raylib(),
        );
        d3.draw_triangle3D(
            // lado 2
            p1.to_v3_raylib(),
            p4.to_v3_raylib(),
            p3.to_v3_raylib(),
            param.fill_color.to_color_raylib(),
        );
    }

    if param.stroke_bool {
        draw_linea_3d(d, param, va, vb);
        draw_linea_3d(d, param, vb, vc);
        draw_linea_3d(d, param, vc, vd);
        draw_linea_3d(d, param, vd, va);
    }
}

// Esta funcion es un cubo con puntos creado desde triangulos para que le afecten a todos los puntos las matrices
// la posicion es el centro del cubo
pub fn cubo3d(d: &mut RaylibDrawHandle, param: &mut Parametros, lado: f32) {
    let slado = lado / 2.0;
    let puntos: Vec<Vector4> = vec![
        Vector4::new(-slado, -slado, slado, 1.0),  // 0
        Vector4::new(slado, -slado, slado, 1.0),   // 1
        Vector4::new(slado, slado, slado, 1.0),    // 2
        Vector4::new(-slado, slado, slado, 1.0),   // 3
        Vector4::new(-slado, -slado, -slado, 1.0), // 4
        Vector4::new(slado, -slado, -slado, 1.0),  // 5
        Vector4::new(slado, slado, -slado, 1.0),   // 6
        Vector4::new(-slado, slado, -slado, 1.0),  // 7
    ];

    // la matriz total3d, se aplica al renderizar los triangulos

    draw_rectangulo_3d(d, param, puntos[0], puntos[1], puntos[2], puntos[3]);
    draw_rectangulo_3d(d, param, puntos[1], puntos[5], puntos[6], puntos[2]);
    draw_rectangulo_3d(d, param, puntos[5], puntos[4], puntos[7], puntos[6]);
    draw_rectangulo_3d(d, param, puntos[0], puntos[3], puntos[7], puntos[4]);
    draw_rectangulo_3d(d, param, puntos[2], puntos[6], puntos[7], puntos[3]);
    draw_rectangulo_3d(d, param, puntos[0], puntos[4], puntos[5], puntos[1]);
}
/******************************************************
                       CURVAS
******************************************************/

pub fn bezier(
    // Curva Bezier cubica con dos anclajes y dos puntos de control
    d: &mut RaylibDrawHandle,
    param: &mut Parametros,
    anclaje1_x: f32,
    anclaje1_y: f32,
    control1_x: f32,
    control1_y: f32,
    control2_x: f32,
    control2_y: f32,
    anclaje2_x: f32,
    anclaje2_y: f32,
) {
    let delta = 0.02;
    let paso = (10.0 / delta) as usize;

    begin_shape(ModosBeginShape::Lines);
    for t in 0..=paso {
        let t = t as f32 * delta;

        let p1 = Vector3::new(anclaje1_x, anclaje1_y, 1.0);
        let p2 = Vector3::new(control1_x, control1_y, 1.0);
        let p3 = Vector3::new(control2_x, control2_y, 1.0);
        let p4 = Vector3::new(anclaje2_x, anclaje2_y, 1.0);
        let v = cubic(d, param, p1, p2, p3, p4, t);

        vertex(v.x, v.y, param);
    }
    end_shape(d, param, ModosBeginShape::Lines);

    // d.draw_line_bezier_cubic(x as i32, y as i32)
}

// Funcion auxiliar para bezier()
fn cubic(
    _d: &mut RaylibDrawHandle,
    _param: &mut Parametros,
    p1: Vector3,
    p2: Vector3,
    p3: Vector3,
    p4: Vector3,
    t: f32,
) -> Vector3 {
    let v1 = quadratic(p1, p2, p3, t);
    let v2 = quadratic(p2, p3, p4, t);

    let x = lerp(v1.x, v2.x, t);
    let y = lerp(v1.y, v2.y, t);

    Vector3::new(x, y, 1.0)
}

// Funcion auxiliar para bezier()
fn quadratic(p0: Vector3, p1: Vector3, p2: Vector3, t: f32) -> Vector3 {
    let x1 = lerp(p0.x, p1.x, t);
    let y1 = lerp(p0.y, p1.y, t);
    let x2 = lerp(p1.x, p2.x, t);
    let y2 = lerp(p1.y, p2.y, t);
    // hacemos una curva bezier cuadratica
    let x = lerp(x1, x2, t);
    let y = lerp(y1, y2, t);
    Vector3::new(x, y, 1.0)
}

pub fn bezier_detail() {}
pub fn bezier_point() {}
pub fn bezier_tangent() {}
pub fn curve() {}
pub fn curve_detail() {}
pub fn curve_tightness() {}
pub fn curve_point() {}
pub fn curve_tangent() {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::Engine;
    use crate::p5;
    use crate::p5::color_p5::{background, fill, fill1, fill3, no_fill, stroke1, Color};
    use crate::transform::translate;
    use std::time;

    #[test]
    fn pba() {
        assert_eq!(5, 5);
    }

    /*#[test]
    fn dibuja_arc() {
        // Ancho y alto de la pantalla
        const ANCHO: i32 = 600;
        const ALTO: i32 = 600;

        println!("dibuja arc");

        let (mut rl, th) = raylib::init()
            .msaa_4x()
            .size(ANCHO, ALTO)
            .title("Ventana en pbasssss")
            .build();

        // Bucle principal ***********************************************************************
        for _ in 0..20000 {
            let mut engine = Engine::new(ANCHO as f32, ALTO as f32);

            //'main_loop: loop {
            let mut d = rl.begin_drawing(&th);
            background(&mut engine, &mut d, 55);

            // Ejes de coordenadas y centrado ------------------------------------------------------
            stroke1(255.0, &mut engine.param);
            translate(ANCHO as f32 / 2.0, ALTO as f32 / 2.0, &mut engine.param);
            line(&mut d, &mut engine.param, 0.0, 300.0, 0.0, -300.0);
            line(&mut d, &mut engine.param, -300.0, 0.0, 300.0, 0.0);
            // -------------------------------------------------------------------------------------

            no_fill(&mut engine.param);
            stroke_weight(4.0, &mut engine.param);
            arc(&mut engine.param, &mut d, 0.0, 0.0, 100.0, 200.0, 0, 90)
        }
    }*/

    /*#[test]
    fn dibuja_elipse() {
        // Ancho y alto de la pantalla
        const ANCHO: i32 = 600;
        const ALTO: i32 = 600;

        println!("dibuja elipse");

        let (mut rl, th) = raylib::init()
            .msaa_4x()
            .size(ANCHO, ALTO)
            .title("Ventana en pbasssss")
            .build();

        // Bucle principal ***********************************************************************
        for _ in 0..20000 {
            let mut engine = Engine::new(ANCHO as f32, ALTO as f32);

            //'main_loop: loop {
            let mut d = rl.begin_drawing(&th);
            background(&mut engine, &mut d, 55);

            // Ejes de coordenadas y centrado ------------------------------------------------------
            stroke1(255.0, &mut engine.param);
            translate(ANCHO as f32 / 2.0, ALTO as f32 / 2.0, &mut engine.param);
            line(&mut d, &mut engine.param, 0.0, 300.0, 0.0, -300.0);
            line(&mut d, &mut engine.param, -300.0, 0.0, 300.0, 0.0);
            // -------------------------------------------------------------------------------------

            no_fill(&mut engine.param);
            stroke_weight(4.0, &mut engine.param);

            ellipse(&mut engine.param, &mut d, 0.0, 0.0, 400.0, 200.0);
        }
    }*/

    #[test]
    fn dibuja_triangulo() {
        // Ancho y alto de la pantalla
        const ANCHO: i32 = 600;
        const ALTO: i32 = 600;

        println!("dibuja triangulo");

        let (mut rl, th) = raylib::init()
            .msaa_4x()
            .size(ANCHO, ALTO)
            .title("Ventana en pbasssss")
            .build();

        // Bucle principal ***********************************************************************
        for _ in 0..20000 {
            let mut engine = Engine::new(ANCHO as f32, ALTO as f32);

            //'main_loop: loop {
            let mut d = rl.begin_drawing(&th);
            background(&mut engine, &mut d, 55);

            // Ejes de coordenadas y centrado ------------------------------------------------------
            stroke1(255.0, &mut engine.param);
            translate(ANCHO as f32 / 2.0, ALTO as f32 / 2.0, &mut engine.param);
            line(&mut d, &mut engine.param, 0.0, 300.0, 0.0, -300.0);
            line(&mut d, &mut engine.param, -300.0, 0.0, 300.0, 0.0);
            // -------------------------------------------------------------------------------------

            //no_fill(&mut engine.param);
            //stroke_weight(4.0, &mut engine.param);
            fill3(255.0, 0.0, 0.0, &mut engine.param);
            triangle(
                &mut d,
                &mut engine.param,
                -200.0,
                200.0,
                0.0,
                -200.0,
                200.0,
                200.0,
            );
        }
    }

    #[test]
    fn dibuja_esquare_2d_en_3d() {
        //}
        // pub fn square_2d_en_3d(
        //     d: &mut RaylibDrawHandle,
        //     param: &mut Parametros,
        //     x: f32,
        //     y: f32,
        //     lado: f32,

        // Ancho y alto de la pantalla
        const ANCHO: i32 = 600;
        const ALTO: i32 = 600;

        println!("dibuja triangulo");

        let (mut rl, th) = raylib::init()
            .msaa_4x()
            .size(ANCHO, ALTO)
            .title("Ventana en pbasssss")
            .build();

        // Bucle principal ***********************************************************************
        for _ in 0..20000 {
            let mut engine = Engine::new(ANCHO as f32, ALTO as f32);

            //'main_loop: loop {
            let mut d = &mut rl.begin_drawing(&th);
            background(&mut engine, &mut d, 55);

            // Ejes de coordenadas y centrado ------------------------------------------------------
            stroke1(255.0, &mut engine.param);
            translate(ANCHO as f32 / 2.0, ALTO as f32 / 2.0, &mut engine.param);

            // -------------------------------------------------------------------------------------

            //no_fill(&mut engine.param);
            //stroke_weight(4.0, &mut engine.param);
            fill3(255.0, 0.0, 0.0, &mut engine.param);
            cuadrado_2d_en_3d(d, &mut engine.param, 0.0, 0.0, 1.0);
        }
    }
}
