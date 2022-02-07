/*
    createImage()
    saveCanvas()
    saveFrames()
    p5.Image

Loading & Displaying

    loadImage()
    image()
    tint()
    noTint()
    imageMode()

Pixels

    pixels
    blend()
    copy()
    filter()
    get()
    loadPixels()
    set()
    updatePixels()

*/

//use image::{DynamicImage, GenericImageView, load};
//extern crate image;
use crate::p5::vector_p5::Vector3;
use raylib::prelude::*;

use crate::matem::grados;
use crate::parametros::{ImageMode, Parametros};

pub trait ImageLF {
    fn get(&mut self, x: f32, y: f32, ancho: f32, alto: f32) -> Image;
}

impl ImageLF for Image {
    fn get(&mut self, x: f32, y: f32, ancho: f32, alto: f32) -> Image {
        self.from_image(Rectangle::new(x, y, ancho, alto))
    }
}

#[derive(Debug, Clone)]
pub struct ImageLf {
    //pub image: GameResult<Image>,
    pub image_pixels: Vec<Color>,
    pub image_width: u32,
    pub image_height: u32,
}

impl ImageLf {
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
    pub fn load_image_fi(nombre_imagen: &str) -> Image {
        Image::load_image(nombre_imagen).expect("error con imagen")
    }

    // Rellena param.pixels con Color sacados de pantalla
    pub fn load_pixels(
        param: &mut Parametros,
        rl: &mut raylib::RaylibHandle,
        th: &raylib::RaylibThread,
    ) {
        let img_rl = rl.get_screen_data(&th);
        // Cambio raylib 3.7 añadido to_vec()
        param.pixels = img_rl.get_image_data().to_vec();
    }

    //    pub fn update_pixels(param: &mut Parametros, rl: &mut raylib::RaylibHandle,
    //                         th: &raylib::RaylibThread, v: Vec<Color>, ancho: i32, alto: i32) {

    // borrado por raylib 3.7
    // pub fn preupdate_pixels(
    //     param: &mut Parametros,
    //     rl: &mut raylib::RaylibHandle,
    //     th: &raylib::RaylibThread,
    //     ancho: i32,
    //     alto: i32,
    // ) -> Texture2D {
    //     let im = Image::load_image_ex(&param.pixels, ancho, alto).unwrap();
    //     let textu = rl
    //         .load_texture_from_image(th, &im)
    //         .expect("error preupdate_pixels");
    //
    //     textu
    // }

    // Pinta en ventana imagen con los bits de param.pixel<Color>
    //    pub fn update_pixels(param: &mut Parametros, rl: &mut raylib::RaylibHandle,
    //                         th: &raylib::RaylibThread, ancho: i32, alto: i32) {

    pub fn update_pixels(
        param: &mut Parametros,
        d: &mut RaylibDrawHandle,
        //textu: Option<&Texture2D>,
        ancho: i32,
        alto: i32,
    ) {
        //pub fn update_pixels(d: &mut RaylibDrawHandle, textu: Option<&Texture2D>) {

        //let im = Image::load_image_ex(&param.pixels, ancho, alto).unwrap();

        //        let t = d.lo

        //let kk = v.len();
        //let mut d = rl.begin_drawing(&th);
        //let t = textu.as_ref();

        //d.draw_texture(&textu.unwrap(), 0, 0, Color::WHITE);

        for x in 0..ancho {
            for y in 0..alto {
                let indice = (x + y * ancho) as usize;
                d.draw_pixel(x as i32, y as i32, param.pixels[indice]);
            }
        }
    }

    // Pinta en ventana datos en estructura Image
    //    pub fn image(rl: &mut raylib::RaylibHandle, th: &raylib::RaylibThread,
    //                 im: &Image, x: f32, y: f32) {

    pub fn image(
        d: &mut RaylibDrawHandle,
        param: &mut Parametros,
        t: &Texture2D,
        x_vieja: f32,
        y_vieja: f32,
    ) {
        //let t = rl.load_texture_from_image(th, im).unwrap();

        let p = param.matriz_total * Vector3::new(x_vieja, y_vieja, 1.0); // es punto w =1

        let xx = p.x;
        let yy = p.y;

        // Calculamos el angulo de rotación de la matriz total
        let angulo = grados(param.matriz_total.data[0].acos());

        // Ancho y alto de la textura
        let ancho = t.width() as f32;
        let alto = t.height() as f32;

        // Origen y destino igual no hay cambio de tamaño
        let sourcerec = Rectangle::new(0.0, 0.0, ancho, alto);
        let destrec = Rectangle::new(xx, yy, ancho, alto);

        // El origen de rotación variará según sea el modo
        let origin = match param.image_mode {
            ImageMode::Center => Vector2::new(ancho / 2.0, alto / 2.0),
            ImageMode::Corner => Vector2::new(0.0, 0.0),
            _ => Vector2::new(0.0, 0.0),
        };

        d.draw_texture_pro(t, sourcerec, destrec, origin, angulo, Color::WHITE);
    }
}

// Cambia el parámetro de modo de agarre de la imagen
pub fn image_mode(param: &mut Parametros, modo: ImageMode) {
    param.image_mode = modo;
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
