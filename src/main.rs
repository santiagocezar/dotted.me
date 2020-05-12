/*
converts 1 2 3 4 5 6 7 8
    into 8 7 6 4 2 5 3 1
*/


use libdotted::dotted;
use image::open;
use clap::Clap;

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
    image: std::path::PathBuf,
}

fn main() -> Result<(), std::io::Error> {
    let opt: Opt = Opt::parse();
    let img = match open(opt.image) {
        Ok(img) => img,
        Err(e) => {
            println!("Error al cargar la imagen: {}", e.to_string());
            return Err(std::io::ErrorKind::InvalidInput.into())
        }
    };
    
    let dots = dotted(img, opt.level, opt.width, opt.invert);
    println!("{}", dots);
    Ok(())
}
