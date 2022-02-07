use crate::parametros::{CodigosRaton, Parametros};

// Solo el inicio de la presion de raton de los tres botones
pub fn mouse_is_pressed_inicio(param: &Parametros) -> bool {
    !(param.mouse_boton_inicio == CodigosRaton::NadaRaton)
}

// indica si cualquiera de los tres botones está presionado
pub fn mouse_is_pressed(param: &Parametros) -> bool {
    !(param.mouse_boton_inicio == CodigosRaton::NadaRaton
        && param.mouse_boton_mantiene == CodigosRaton::NadaRaton)
}
