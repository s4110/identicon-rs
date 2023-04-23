use std::io::{Result, self, BufRead};
use std::process::exit;

use image::codecs::png::PngEncoder;
use image::{ColorType, ImageEncoder};

use identicon_rs::Identicon;

fn main() {
    match  generate() {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    }
}

fn generate() -> Result<()> {
    let input = io::stdin();
    let mut reader = input.lock();
  
    let mut buffer = String::new();

    reader.read_line(&mut buffer).expect("input error");

    let identicon = Identicon::new(&buffer);
    let image = identicon.generate();
    let (width, height) = image.dimensions();
    let output = &mut io::stdout();
    let encoder = PngEncoder::new(output);
    encoder
        .write_image(image.as_ref(), width, height, ColorType::Rgb8)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}
