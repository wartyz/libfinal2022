use crate::engine::Engine;
use crate::transform::{pop_matrix, push_matrix};
use crate::transform3d::{pop_matrix3d, push_matrix3d};

// Al hacer push y pop se controlan las siguientes funciones
// Ojo está incompleto
// fill(), stroke(), tint(), strokeWeight(), strokeCap(), strokeJoin(),
// imageMode(), rectMode(), ellipseMode(), colorMode(), textAlign(),
// textFont(), textMode(), textSize(), textLeading().

pub fn push(engine: &mut Engine) {
    push_matrix(&mut engine.param);

    engine.parambakup.fill_color = engine.param.fill_color;
    engine.parambakup.fill_bool = engine.param.fill_bool;
    engine.parambakup.stroke_bool = engine.param.stroke_bool;
    engine.parambakup.stroke_color = engine.param.stroke_color;
    engine.parambakup.stroke_weight = engine.param.stroke_weight;
    engine.parambakup.colormode = engine.param.colormode.clone();
    engine.parambakup.rect_mode = engine.param.rect_mode.clone();
}

pub fn pop(engine: &mut Engine) {
    pop_matrix(&mut engine.param);

    engine.param.fill_color = engine.parambakup.fill_color;
    engine.param.fill_bool = engine.parambakup.fill_bool;
    engine.param.stroke_bool = engine.parambakup.stroke_bool;
    engine.param.stroke_color = engine.parambakup.stroke_color;
    engine.param.stroke_weight = engine.parambakup.stroke_weight;
    engine.param.colormode = engine.parambakup.colormode.clone();
    engine.param.rect_mode = engine.parambakup.rect_mode.clone();
}

//pub fn no_loop(ctx: &mut Context) {
////    ggez::event::quit(ctx)
////}

pub fn push3d(engine: &mut Engine) {
    push_matrix3d(&mut engine.param);

    engine.parambakup.fill_color = engine.param.fill_color;
    engine.parambakup.fill_bool = engine.param.fill_bool;
    engine.parambakup.stroke_bool = engine.param.stroke_bool;
    engine.parambakup.stroke_color = engine.param.stroke_color;
    engine.parambakup.stroke_weight = engine.param.stroke_weight;
    engine.parambakup.colormode = engine.param.colormode.clone();
    engine.parambakup.rect_mode = engine.param.rect_mode.clone();
}

pub fn pop3d(engine: &mut Engine) {
    pop_matrix3d(&mut engine.param);

    engine.param.fill_color = engine.parambakup.fill_color;
    engine.param.fill_bool = engine.parambakup.fill_bool;
    engine.param.stroke_bool = engine.parambakup.stroke_bool;
    engine.param.stroke_color = engine.parambakup.stroke_color;
    engine.param.stroke_weight = engine.parambakup.stroke_weight;
    engine.param.colormode = engine.parambakup.colormode.clone();
    engine.param.rect_mode = engine.parambakup.rect_mode.clone();
}
