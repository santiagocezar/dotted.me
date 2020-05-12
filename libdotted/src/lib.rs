use image::DynamicImage;

pub mod canvas;
use canvas::Canvas;

pub fn dotted(img: DynamicImage, level: u8, width: u32, invert: bool) -> String {
    let img = img.grayscale();
    let img = img.resize(width, std::u32::MAX, image::imageops::Nearest);
    let img = img.into_rgba();

    let width = img.width() as usize;
    let height = img.height() as usize;
    
    let mut c = Canvas::new(width, height);

    let mut i = 0;
    for p in img.pixels() {
        c.set(invert ^ (p[0] < level), (i % width) as usize, (i / height) as usize);
        i+=1;
    }
    

    c.draw()
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn wadotted(pixels: &JsValue, w: u32, h: u32, level: u8, x_dots: u32, invert: bool) {
    let pixel_vec: Vec<u8> = pixels.into_serde().unwrap();
    let img = image::RgbImage::from_raw(w, h, pixel_vec).unwrap();
    dotted(DynamicImage::ImageRgb8(img), level, x_dots, invert);
}
