/*
converts 1 2 3 4 5 6 7 8
    into 8 7 6 4 2 5 3 1
*/

use clap::Clap;
use image::open;

mod canvas;
use canvas::Canvas;
#[derive(Debug, Clap)]

#[clap(name = "dotted.me")]
#[clap(version = "0.1")]
#[clap(author = "Santiago C. <santiagocezar2013@gmail.com>")]
#[clap(about = "Convertir imagen a carácteres Braille")]

struct Opt {

    /// Cantidad de puntos horizontales (1 caracter = 2 puntos)
    #[clap(short, long, default_value = "32")]
    width: u32,

    /// Nivel de luminosidad para el umbral (0-255)
    #[clap(short, long, default_value = "128")]
    level: u8,
    
    /// Invertir para usar en temas oscuros
    #[clap(short, long)]
    invert: bool,
    
    /// Archivo de imagen
    #[clap()]
    image: String,
}

fn main() -> Result<(), std::io::Error> {
    let opt: Opt = Opt::parse();
    let _img = match open(opt.image) {
        Ok(img) => img,
        Err(e) => {
            println!("Error al cargar la imagen: {}", e.to_string());
            return Err(std::io::ErrorKind::InvalidInput.into());
        }
    };
    let _img = _img.grayscale();
    let _img = _img.resize(opt.width, std::u32::MAX, image::imageops::Nearest);
    let img = _img.into_rgba();

    let width = img.width() as usize;
    let height = img.height() as usize;
    
    let mut c = Canvas::new(width, height);

    let mut i = 0;
    for p in img.pixels() {
        c.set(opt.invert ^ (p[0] < opt.level), (i % width) as usize, (i / height) as usize);
        i+=1;
    }

    println!("{}", c.draw());
    Ok(())
}
