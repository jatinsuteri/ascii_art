use std::path::Path;
use image::{GenericImageView,ImageBuffer,Luma};
use clap::ValueEnum;
use imageproc::gradients::{horizontal_sobel, vertical_sobel};
use std::f32::consts::PI;


#[derive(ValueEnum, Clone, Debug)]
pub enum AsciiPattern{
    Acerola,
    Me,
    Custom,
}

impl AsciiPattern{
    pub fn pattern(&self) -> &[char]{
        match self{
            Self::Acerola => &[' ', '.', ';', 'c', '?','o', 'P', 'O', '@', '█'],
            Self::Me => &[' ', '.', '-', '+', '*', '%', '#', '&', '@', '█'],
            Self::Custom => &[' ', '.', '-', '+', '*', '%', '#', '?', '&', '@', '█'],
        }
    }
}

pub fn max_magnitude_func(
    gx: &ImageBuffer<Luma<i16>, Vec<i16>>, 
    gy: &ImageBuffer<Luma<i16>, Vec<i16>>, 
    height: u32, 
    width: u32) -> f32{
    let mut max_magnitude = 0.0;

    for y in 0..height {
        for x in 0..width {
            let gx_val = gx.get_pixel(x, y)[0] as f32;
            let gy_val = gy.get_pixel(x, y)[0] as f32;

            let magnitude = (gx_val.powi(2) + gy_val.powi(2)).sqrt();

            if magnitude > max_magnitude {
                max_magnitude = magnitude;
            }
        }
    }
    max_magnitude

}

pub fn run(path: &Path , color:bool, ascii_char:AsciiPattern, invert: bool, edgedetec: bool, brighten: bool) -> String {
    let img = image::open(path).expect("Image not found");

    let mut ascii_char = ascii_char.pattern().to_vec();
    if invert{
        ascii_char.reverse();
    }
    let (width, height) = img.dimensions();
    let mut res = String::new();

    //basic preprocessing

    let img = img.resize(width/2, height/2, image::imageops::FilterType::Lanczos3);
    let (width, height) = img.dimensions();

    if edgedetec{
        let img = img.grayscale().to_luma8();
        let _ = img.save("output.png");  
        let gx = horizontal_sobel(&img);
        let gy = vertical_sobel(&img);
        let max_magnitude = max_magnitude_func(&gx,&gy,height,width);
        let threshold = max_magnitude * 0.3;

        for y in 0..height{
            for x in 0..width{
                let gx_val = gx.get_pixel(x, y)[0] as f32;
                let gy_val = gy.get_pixel(x, y)[0] as f32;

                let normalized_angle = ((gy_val.atan2(gx_val) / PI) * 0.5) + 0.5;

                let edge_char = match normalized_angle {
                    a if a < 0.0625 => '|',   
                    a if a < 0.1875 => '/',    
                    a if a < 0.3125 => '-',    
                    a if a < 0.4375 => '\\',   
                    a if a < 0.5625 => '|',    
                    a if a < 0.6875 => '/',    
                    a if a < 0.8125 => '-',    
                    a if a < 0.9375 => '\\',   
                    _ => '|',                 
                };
                
                let magnitude = (gx_val.powf(2.0) + gy_val.powf(2.0)).sqrt();
                
                if magnitude > threshold {
                    res.push(edge_char);

                } else {
                    let pixel = img.get_pixel(x, y)[0];
                    let lum = match brighten{
                        true => (pixel as f32) / 255.0,
                        false => ((pixel as f32) / 255.0).powf(1.7),
                    };
                    let ascii_index = (lum.clamp(0.0, 1.0) * (ascii_char.len() - 1) as f32).round() as usize;
                    
                    res.push(ascii_char[ascii_index]);
                    // res.push(' ');
                }

            }
            res.push('\n')
        }
        return res
    }
    
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