use std::path::Path;
use image::GenericImageView;
use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub enum AsciiPattern{
    Acerola,
    Me,
    Custom,
}

impl AsciiPattern{
    pub fn pattern(&self) -> &[char]{
        match self{
            Self::Acerola => &[' ', '.', ';', 'c', 'o', 'P', 'O', '?', '@', '█'],
            Self::Me => &[' ', '.', '-', '+', '*', '%', '#', '&', '@', '█'],
            Self::Custom => &[' ', '.', '-', '+', '*', '%', '#', '?', '&', '@', '█'],
        }
    }
}

pub fn run(path: &Path , color:bool, ascii_char:AsciiPattern ) -> String {
    let img = image::open(path).expect("Image not found");

    let ascii_char = ascii_char.pattern();
    let (width, height) = img.dimensions();
    let mut res = String::new();

    //basic preprocessing

    let img = img.resize(width/4, height/4, image::imageops::FilterType::Lanczos3);
    let (width, height) = img.dimensions();
    
    if color {
        for y in 0..height {
            for x in 0..width {
                let pixel = img.get_pixel(x, y);
                let r = pixel[0] as f32 / 255.0;
                let g = pixel[1] as f32 / 255.0;
                let b = pixel[2] as f32 / 255.0;
                let lum = (0.2126 * r + 0.7152 * g + 0.0722 * b).clamp(0.0, 1.0);
                let ascii_index = (lum * (ascii_char.len() - 1) as f32).round() as usize;
                res.push_str(&format!(
                    "<span style=\"color: rgb({}, {}, {})\">{}</span>",
                    (r * 255.0) as u8,
                    (g * 255.0) as u8,
                    (b * 255.0) as u8,
                    ascii_char[ascii_index]
                ));
            }
            res.push_str("<br>");
        }
    }
    else {
        let img = img.grayscale();
        for y in 0..height {
            for x in 0..width {
                let pixel = img.get_pixel(x, y)[0];
                let lum = ((pixel as f32) / 255.0).powf(1.7);
                let ascii_index = (lum.clamp(0.0, 1.0) * (ascii_char.len() - 1) as f32).round() as usize;

                res.push(ascii_char[ascii_index]);
            }
            res.push('\n');
        }
    }
    res

}