use image::{ImageBuffer, Rgb, RgbImage};
use hmac_sha256::Hash;

use hsl::HSL;
use nibbler::Nibbler;
use remap::Remap;

mod hsl;
mod nibbler;
mod remap;

impl From<hsl::RGB> for Rgb<u8> {
    fn from(from: hsl::RGB) -> Rgb<u8> {
        Rgb([from.red, from.green, from.blue])
    }
}

pub struct Identicon {
    size: u32,
    hash: [u8; 32],
}

impl Identicon {
    pub fn new(source: impl Into<String>) -> Self {
        let source_into : String = source.into();
        let hash = Identicon::to_hash(&source_into.into_bytes());

        Self { size: 560, hash: hash.to_owned()}
    }

    fn to_hash(source: &Vec<u8>) -> [u8; 32] {
        let mut sha256 = Hash::new();
        sha256.update(source);
        sha256.finalize()
    }

    fn pattern_color(&self) -> Rgb<u8> {
        let first_half = (self.hash[28] as u16 & 0x0f) << 8;
        let latter_half = self.hash[29] as u16;

        let bits_for_hue = (first_half | latter_half) as u32;
        let bits_for_sat = self.hash[30] as u32;
        let bits_for_lum = self.hash[31] as u32;

        let hue = Remap::new(bits_for_hue).to_hue();
        let sat = Remap::new(bits_for_sat).to_sat();
        let lum = Remap::new(bits_for_lum).to_lum();

        HSL::new(hue, sat, lum).to_rgb().into()
    }

    fn rect(image: &mut RgbImage, x0: u32, y0: u32, x1: u32, y1: u32, color: Rgb<u8>) {
        for x in x0..x1 {
            for y in y0..y1 {
                image.put_pixel(x, y, color);
            }
        }
    }

    fn pixels(&self) -> [bool; 49] {
        let mut nibbles = Nibbler::new(&self.hash).map(|x| x % 2 == 0);
        let mut pixels = [false; 49];
        for col in (0..4).rev() {
            for row in 0..7 {
                let ix = col + (row * 7);
                let mirror_col = 6 - col;
                let mirror_ix = mirror_col + (row * 7);
                let paint = nibbles.next().unwrap_or(false);
                pixels[ix] = paint;
                pixels[mirror_ix] = paint;
            }
        }

        pixels
    }

    pub fn generate(&self) -> RgbImage {
        let pixel_size = 70;
        let sprite_size = 7;
        let margin = pixel_size / 2;

        let background = Rgb([240, 240, 240]);
        let pattern_color = self.pattern_color();

        let mut image = ImageBuffer::from_pixel(self.size, self.size, background);

        for (row, pix) in self.pixels().chunks(sprite_size).enumerate() {
            for (col, painted) in pix.iter().enumerate() {
                if *painted {
                    let x = col * pixel_size;
                    let y = row * pixel_size;
                    Identicon::rect(
                        &mut image,
                        (x + margin) as u32,
                        (y + margin) as u32,
                        (x + pixel_size + margin) as u32,
                        (y + pixel_size + margin) as u32,
                        pattern_color,
                    )                    
                }
            }
        }

        image
    }
}
