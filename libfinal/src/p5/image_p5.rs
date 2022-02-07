/*
Parámetros:
      width
      height
      pixels

Métodos:
      loadPixels()
      updatePixels()
      get()
      set()
      resize()
      copy()
      mask()
      filter()
      blend()
      save()
      reset()
      getCurrentFrame()
      setFrame()
      numFrames()
      play()
      pause()
      delay()
*/
//use image::{DynamicImage, GenericImageView, load};
//extern crate image;
//use crate::p5::color_p5::Color;
use crate::parametros::{Filtros, Parametros};
use raylib::prelude::*;
use raylib::texture::Image;

#[derive(Debug, Clone)]
pub struct P5Image {
    //pub image: GameResult<Image>,
    pub img: Image,
    pub pixels: Vec<Color>,

    pub width: i32,
    pub height: i32,
}

impl P5Image {
    //  Pasa a image_pixels[] lo cargado
    pub fn load_image_anterior(_nombre_imagen: &str) { //-> P5Image {
                                                       //        match image::open(nombre_imagen) {
                                                       //            Ok(img) => {
                                                       //                let (width, height) = img.dimensions();
                                                       //                println!("width = {:?}  height = {:?}", width, height);
                                                       //
                                                       //                let img = match img {
                                                       //                    DynamicImage::ImageRgba8(img) => img,
                                                       //                    img => img.to_rgba(),
                                                       //                };
                                                       //
                                                       //                P5Image {
                                                       //                    image_pixels: img.into_raw(),
                                                       //                    image_width: width,
                                                       //                    image_height: height,
                                                       //
                                                       //                }
                                                       //            }
                                                       //            Err(e) => panic!("No se puede cargar la imagen {}: {}", nombre_imagen, e)
                                                       //        }
    }

    // Usando raylib
    pub fn load_image_fi(nombre_imagen: &str) -> P5Image {
        let image = Image::load_image(nombre_imagen).expect("error con imagen");
        let mut image_pixels = vec![];

        let image_width = image.width();
        let image_height = image.height();

        for _x in 0..image_width {
            for _y in 0..image_height {
                image_pixels.push(Color::new(0, 0, 0, 255));
            }
        }

        P5Image {
            img: image,
            pixels: image_pixels,
            width: image_width,
            height: image_height,
        }
    }

    // Rellena image_pixels con u8 sacados de image
    // Como método en p5
    pub fn load_pixels(
        &mut self,
        param: &mut Parametros,
        _rl: &mut raylib::RaylibHandle,
        _th: &raylib::RaylibThread,
    ) {
        //let p = param.ancho * param.alto * 4.0;
        //let img_rl = rl.get_screen_data(&th);
        // tamaño de img_data = ancho * alto y cada elemento tiene 4 u8 (r,g,b,a)
        //let img_data = img_rl.get_image_data();
        let img_data = self.img.get_image_data(); // Ahora solo cogemos pixels de la imagen
                                                  // recibida
        let _kk = param.pixels.len();
        let _hh = img_data.len();

        for f in 0..img_data.len() {
            let p = Color::new(img_data[f].r, img_data[f].g, img_data[f].b, img_data[f].a);
            self.pixels[f] = p;
        }
    }

    // Pinta en ventana imagen con los bits recibidos en un Vec<Color>
    //    pub fn update_pixels(param: &mut Parametros, rl: &mut raylib::RaylibHandle,
    //                         th: &raylib::RaylibThread, v: Vec<Color>, ancho: i32, alto: i32) {

    // Como método en p5
    pub fn update_pixels(
        &mut self,
        _param: &mut Parametros,
        _rl: &mut raylib::RaylibHandle,
        _th: &raylib::RaylibThread,
        _ancho: i32,
        _alto: i32,
    ) {
        //let kk = v.len();
        //let mut d = rl.begin_drawing(&th);

        //self.image = self.image_pixels.get_data();
        //let kk = self.pixels.len();

        // Cambio raylib 3.7
        //self.img = Image::load_image_ex(&self.pixels, self.width, self.height).unwrap();

        //let kk = self.pixels.len();
        //        for x in 0..ancho {
        //            for y in 0..alto {
        //                let indice = (x + y * ancho) as usize;
        //                d.draw_pixel(x as i32, y as i32, param.pixels[indice]);
        //            }
        //        }
    }

    // Pinta en ventana datos en estructura Image
    pub fn image(
        &mut self,
        x: i32,
        y: i32,
        rl: &mut raylib::RaylibHandle,
        th: &raylib::RaylibThread,
    ) {
        let t = rl.load_texture_from_image(th, &mut self.img).unwrap();

        let mut d = rl.begin_drawing(&th);

        d.draw_texture(&t, x, y, Color::new(255, 255, 255, 255));
    }

    //    pub fn image(&mut self, d: &mut RaylibDrawHandle, t: Texture2D, x: i32, y: i32) {
    //
    //        //let t = rl.load_texture_from_image(th, im).unwrap();
    //
    //
    //        // Ojo creamos otro RayLibDrawHandle aquí -----------------
    //        //let mut d = rl.begin_drawing(&th);
    //
    //
    //        d.draw_texture(&t, x as i32, y as i32, Color::new(255, 255, 255, 255));
    //    }

    pub fn filter(&mut self, filt: Filtros) {
        match filt {
            Filtros::NoFiltro => {}
            Filtros::Gray => self.img.color_grayscale(),
            _ => {}
        }

        //self.image.color_tint(color);
    }
}

//impl P5Image {
//    // Carga fichero y rellena image, image_pixels, image_width, image_height
//    //pub fn new(path: String) -> P5Image {
////        let image = Image::new(ctx, path);
////        let image_clone = image.clone().unwrap();
////        let w = image_clone.width() as usize;
////        let h = image_clone.height() as usize;
////        println!("En p5::P5Image::image::new w = {:?}, h = {:?}", w, h);
////
////        let i1 = image.clone();
////        let image_pixels = i1.unwrap().to_rgba8(ctx).unwrap();
////
////
//        //P5Image {
//            //image: image,
//            //image_pixels: image_pixels, // de p5_image
//            //image_width: w, // de p5_image
//            //image_height: h, // de p5_image
//        //}
//    }
//
//    // Pasa a image_pixels[] lo cargado
//    pub fn load_image(file: &str, engine: &mut Engine) {
//
//
//
//
//
//
//
//        //let texture_creator = engine.canvas.unwrap().texture_creator();
//        //let texture = texture_creator.load_texture(file);
//
////        let i1 = self.image.clone();
////        self.image_pixels = i1.unwrap().to_rgba8(ctx).unwrap();
//    }
//
//    // Presenta en pantalla el objeto image en x,y
//    pub fn image(&mut self, x: f32, y: f32) {
//        //let image = self.image.clone();
//
//        //let color = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };
//        //let dest_point = Point2::new(x, y);
//        //let rec = ggez::graphics::Rect::new(200.0, 300.0, 200.0, 100.0);
//        //ggez::graphics::set_screen_coordinates(ctx, rec);
//        //ggez::graphics::draw(ctx, &image.unwrap(), DrawParam::default());
//        //ggez::graphics::draw(ctx, &image.unwrap(), (dest_point, ));
//    }
//
//
//    // Devuelve un objeto P5Image con bits a 0 y image_pixels vacio
//    pub fn create_image(width: usize,
//                        height: usize,
//                        /*color_mode: ColorMode*/) -> P5Image {
//        let c = vec![0u8; width * 4 * height];
//        //let image = Image::from_rgba8(ctx, width as u16, height as u16, &c);
//        P5Image {
//            //image: image,
//            //image_pixels: vec![], // de p5_image
//            //image_width: width, // de p5_image
//            //image_height: height, // de p5_image
//        }
//    }
//
//    // Rellena image_pixels con u8 sacados de image
//    pub fn load_pixels(&mut self) {
//        //let i1 = self.image.clone();
//        //self.image_pixels = i1.unwrap().to_rgba8(ctx).unwrap();
//        //println!("En p5::image::P5Image::load_pixels() tamaño self.image_pixels = {:?}", self
//        //   .image_pixels.len());
////        let num_pixels = self.image_width * self.image_height;
////
////        for p in 0..num_pixels {
////            let (r, g, b, a) = (image_pixels[0 + 4 * p], image_pixels[1 + 4 * p],
////                                image_pixels[2 + 4 * p], image_pixels[3 + 4 * p]);
////            let col = Color::new(r as f32 / 255.0,
////                                 g as f32 / 255.0,
////                                 b as f32 / 255.0,
////                                 a as f32 / 255.0);
////            self.image_pixels.push(col);
////        }
//    }
//
//    // Rellena image con los bits de image_pixels
//    pub fn update_pixels(&mut self) {
//        //println!("En update_pixels  image_width={:?} ", self.image_width);
////        self.image = ggez::graphics::Image::from_rgba8(ctx,
////                                                       self.image_width as u16,
////                                                       self.image_height as u16,
////                                                       &self.image_pixels);
//    }
//
//    //pub fn get(&mut self) -> P5Image {
//        //P5Image {
//            //image: self.image.clone(),
//            //image_pixels: self.image_pixels.clone(),
//            //image_width: self.image_width.clone(),
//            //image_height: self.image_height.clone(),
//        //}
//    //}
//}
