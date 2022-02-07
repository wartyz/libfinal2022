//use std::fs::File;
//use std::io::BufReader;
//use rodio::Source;
//use rodio::Device;
//use rodio::decoder::Decoder;
//use rodio::source::SamplesConverter;


//pub struct SoundFile {
//    device: Device,
//    kk: SamplesConverter<I, D>,
//
//}
//
//impl SoundFile {
//    pub fn new(file: String) -> SoundFile {
//        let device = rodio::default_output_device().unwrap();
//        let fichero = File::open(file.clone()).unwrap();
//        let source = rodio::Decoder::new(BufReader::new(fichero)).unwrap();
//        let kk = source.convert_samples();
//
//
//        SoundFile {
//            device,
//            kk,
//        }
//    }
//
//    pub fn play(&self) {
////        let device = rodio::default_output_device().unwrap();
////        let fichero = File::open(self.file.clone()).unwrap();
////        let source = rodio::Decoder::new(BufReader::new(fichero)).unwrap();
//        rodio::play_raw(&self.device, self.source.convert_samples());
////        let mut audio = Music::from_file(&self.file);
////        match audio {
////            Ok(r) => println!("recibido---------->>>>>>>: {:?}", r),
////            Err(e) => { println!("Error en sonido: {:#?}", e); }
////        };
//
//        //let r = audio.play(12);
//        //println!("valor = {:#?}", r);
//    }
//}