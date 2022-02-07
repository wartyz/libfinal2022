use crate::p5::color_p5::{fill3, stroke3, Color};
use crate::p5::vector3d_p5::Vector4;
use crate::parametros::Parametros;
use crate::shapes::{draw_linea_3d, draw_rectangulo_3d};
use raylib::prelude::RaylibDrawHandle;

pub fn aux_hsv_to_rgb(hue: f32, saturation: f32, value: f32) -> Color {
    //dbg!(hue, saturation, value);
    let c = raylib::core::color::Color::color_from_hsv(hue, saturation, value);

    //dbg!(c.r, c.g, c.b, c.a);
    Color::new(c.r, c.g, c.b, c.a)
}

// dt es la distancia de los cuadrados de ejes al centro de los ejes
pub fn dibuja_ejes(d: &mut RaylibDrawHandle, p: &mut Parametros) {
    // El tamaño del cuadrado es proporcional a la distancia en la que se encuentra

    let dt = p.distancia_cuadrado_eje;
    let dd = dt / 2.0;
    // Eje x ----------------- ROJO -------------------------------
    let va = Vector4::new(-800.0, 0.0, 0.0, 1.0);
    let vb = Vector4::new(800.0, 0.0, 0.0, 1.0);
    stroke3(255.0, 0.0, 0.0, p);
    draw_linea_3d(d, p, va, vb);

    // Cuadrado eje X positivo ------- NARANJA --------------------
    fill3(255.0, 150.0, 0.0, p);
    let var = Vector4::new(dt, -dd, dd, 1.0);
    let vbr = Vector4::new(dt, -dd, -dd, 1.0);
    let vcr = Vector4::new(dt, dd, -dd, 1.0);
    let vdr = Vector4::new(dt, dd, dd, 1.0);
    draw_rectangulo_3d(d, p, var, vbr, vcr, vdr);

    // Eje y ---------------------- AZUL --------------------------
    let va = Vector4::new(0.0, -400.0, 0.0, 1.0);
    let vb = Vector4::new(0.0, 400.0, 0.0, 1.0);
    stroke3(0.0, 0.0, 255.0, p);
    draw_linea_3d(d, p, va, vb);

    // Cuadrado eje y positivo -------- BLANCO --------------------
    fill3(255.0, 255.0, 255.0, p);
    let var = Vector4::new(-dd, dt, dd, 1.0);
    let vbr = Vector4::new(dd, dt, dd, 1.0);
    let vcr = Vector4::new(dd, dt, -dd, 1.0);
    let vdr = Vector4::new(-dd, dt, -dd, 1.0);
    draw_rectangulo_3d(d, p, var, vbr, vcr, vdr);

    // Eje z -------------------- ALLO -----------------------------
    let va = Vector4::new(0.0, 0.0, -400.0, 1.0);
    let vb = Vector4::new(0.0, 0.0, 400.0, 1.0);
    stroke3(255.0, 255.0, 0.0, p);
    draw_linea_3d(d, p, va, vb);

    // Cuadrado eje z positivo ----------- VERDE -------------------
    fill3(0.0, 255.0, 0.0, p);
    let var = Vector4::new(-dd, -dd, dt, 1.0);
    let vbr = Vector4::new(dd, -dd, dt, 1.0);
    let vcr = Vector4::new(dd, dd, dt, 1.0);
    let vdr = Vector4::new(-dd, dd, dt, 1.0);
    draw_rectangulo_3d(d, p, var, vbr, vcr, vdr);
}

// fn mueve_camara_circular_eje_y() {
//     self.angulo_camara += 0.005;
//     self.engine.param.camara.posx = self.distancia_camara * self.angulo_camara.cos();
//     self.engine.param.camara.posz = self.distancia_camara * self.angulo_camara.sin();
// }

pub fn camara_fija(p: &mut Parametros) {
    p.camara.posy = 200.0;
    p.camara.posx = 0.0;
    p.camara.posz = 300.0;
}
