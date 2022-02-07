//let cantidadDeSliders = 0;

// let ancho_boton = 4;

//const ANCHO_BOTON: i32 = 4;

pub struct Slider {
    numero_slider: i32,
    posicion_x: f32,
    posicion_y: f32,
    valor: f32,
    min: f32,
    max: f32,
    paso: f32,
}


impl Slider {
    //pub fn new(min: f32, max: f32, valor: f32, paso: f32) -> Slider {

//        let np = anch - ANCHO_BOTON as f32;
//
//
//        let mut arreglo_de_pasos: Vec<f32> = vec![];
//        let paso_por_pixel = (max - min) / (np - 1.0);
//        for f in 0..np as i32 {
//            arreglo_de_pasos.push(paso_por_pixel * f as f32);
//        }
//
//        let s = Slider {
//            //numero_slider: cantidad_de_sliders,
//            numero_slider: 0,
//            posicion_x: x,
//            posicion_y: y,
//            // slider_value: ,
//            min: min,
//            max: max,
//            altura: alt,
//            anchura: anch,
//            pos_x_boton: 0.0, // Empezamos probando desde 0
//            dentro_x: false,
//            dentro_y: false,
//            numero_de_pasos: np,
//            arreglo_de_pasos: arreglo_de_pasos,
//            paso_elegido: 0.0,
//        };
//        //cantidad_de_sliders += 1;
//        s
    //   }

    //pub fn value(&mut self, engine: &mut Engine) -> f32 {
//        stroke3(255.0, 0.0, 0.0, &mut engine.param);
//
//        //Rect(sl.posicionX, sl.posicionY, width/2, sl.altura)
//        //gfx.RectangleColor(renderer, int32(sl.posicionX), int32(sl.posicionY), int32(width/2), int32(sl.altura),
//        //    sdl.Color{255, 0, 0, 255})
//
//        if self.slider_value < self.min {
//            self.sliderValue = self.min
//        } else if self.sliderValue >= self.max {
//            self.sliderValue = self.max
//        }
//
//        //if sl.anchoBoton < 1 {
//        //    sl.anchoBoton = 1
//        //}
//
//        //Rect(sl.posicionX, sl.posicionY, width/2, sl.altura)
//        // Rectangulo contenedor :no le deben afectar los cambios de ejes en rect() por lo que debemos ejecutar sdl.rect() aqui
//
//        rect(engine.canvas, &mut engine.param, self.posicion_x, self.posicion_y,
//             self.posicion_x + self.anchura, self.posicion_y + self.altura);
//
//        // Botón deslizante  Ojo cambia color
//
//        engine.param.fill_bool = true;
//        engine.param.fill_color = Color::new(255, 0, 0, 255);
//
//        // Si hay colisión del ratón con el rectangulo
//        if (engine.param.mousex as f32) >= self.posicion_x
//            && (engine.param.mousex as f32) < (self.posicion_x + self.anchura) {
//            self.dentro_x = true;
//        }
//
//        if (engine.param.mousey as f32) > self.posicion_y
//            && (engine.param.mousey as f32) < (self.posicion_y + self.altura) {
//            self.dentro_y = true;
//        }
//        //println("numero de pasos = ", int(sl.numeroDePasos))
//        if self.dentro_x && self.dentro_y {
//            if engine.param.estado_raton == sdl.BUTTON_LEFT {
//                //sl.sliderValue = float32(floor((float32(mouseX) - sl.posicionX) / sl.anchoBoton))
//                //sl.numeroDePasos = float32(floor((float32(mouseX) - sl.posicionX) / anchoBoton))
//                if engine.Param.Mousex < self.posicionX + self.anchura - anchoBoton {
//                    self.pasoElegido = (float32(engine.Param.Mousex) - self.posicionX);
//                    //println("paso elegido = ", int(sl.pasoElegido))
//                }
//
//                self.pos_x_boton = self.posicion_x + self.paso_elegido;
//            }
//        }
//
//        self.dentroX = false;
//        self.dentroY = false;
//
////        // No le deben afectar los cambios de ejes en rect() por lo que debemos ejecutar sdl.rect() aqui
////        let recta = sdl.Rect
////        {
////            int32(self.pos_x_boton), int32(self.posicion_y), int32(ancho_boton), int32(self.altura)
////        }
//        let err = engine.Renderer.FillRect(&recta);
//        if err != nil {
//            fmt.Println("Error al crear rectangulo de slider")
//        }
//        //println("posxBoton = ", sl.posXBoton)
//        //println("int(posxBoton) = ", int(sl.posXBoton))
//        //println("posicionX = ", sl.posicionX)
//        //
//        //println("paso elegido = ", sl.pasoElegido)
//        self.slider_value = self.arreglo_de_pasos[int(self.paso_elegido)];
//        return self.sliderValue;
    // }
}