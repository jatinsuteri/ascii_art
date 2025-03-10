use std::path::Path;
use image::{GenericImageView,ImageBuffer,Luma,DynamicImage};
use clap::ValueEnum;
use imageproc::gradients::{horizontal_sobel, vertical_sobel};
use opencv::{core::{self, Mat, MatTraitConst, MatTraitConstManual}, highgui::{self}, imgproc, videoio::{self, VideoCapture, VideoCaptureTrait, VideoCaptureTraitConst, CAP_ANY}};
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
            Self::Acerola => &[' ', '.', ';', 'c', '?','o', 'P', '#', '@', '█'],
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

fn handlecam(path: &Path,imagedetec:bool ,ascii_char:AsciiPattern,video: bool) -> opencv::Result<()>{
    let mut cam:VideoCapture;
    if video{
        cam = videoio::VideoCapture::from_file(
            path.to_str().unwrap(),
               videoio::CAP_ANY,
          )?;
    }
    else{
        cam = VideoCapture::new(1,CAP_ANY)?;
    }    
    
	if !cam.is_opened()? {
		panic!("Unable to open default camera!");
	}
    loop {
		let mut frame = Mat::default();
		cam.read(&mut frame)?;
        
        let mut gray = Mat::default();
        imgproc::cvt_color(
            &frame,
            &mut gray,
            imgproc::COLOR_BGR2GRAY,
            0,
            core::AlgorithmHint::ALGO_HINT_DEFAULT,
        )?;

        let width = gray.cols();
        let height = gray.rows();
        let new_width = 150;
        let new_height = height * new_width / width;


        let mut resized = Mat::default();
        imgproc::resize(
            &gray,
            &mut resized,
            opencv::core::Size::new(new_width, new_height),
            0.0,
            0.0,
            imgproc::INTER_LINEAR,

        )?;
        let mut output = String::new();

        let ascii_char = ascii_char.pattern().to_vec();
        if imagedetec{
            let image_buffer=image::ImageBuffer::<Luma<u8>, _>
            ::from_raw(width as u32, height as u32, resized.data_bytes().unwrap().to_vec()).unwrap();
            let img = DynamicImage::ImageLuma8(image_buffer);
            output = edgedetect(img, width as u32, height as u32, String::new(), false, ascii_char.clone());
        }
        else{
            
            for y in 0..resized.rows() {
                for x in 0..resized.cols() {
                    let pixel = *resized.at_2d::<u8>(y, x)?;
                    let idx = (pixel as usize * (ascii_char.len())) / 256;
                    let ch = ascii_char[idx.min(ascii_char.len() - 1)];
                    output.push(ch);
                }
                output.push('\n');
            }
        }

        print!("{}[2J", 27 as char);
        println!("{output}");

        let key = highgui::wait_key(10)?;
        if key > 0 && key != 255 {
            break;
        }
    }

    Ok(())
}

pub fn run(
    path: &Path , 
    color:bool, 
    ascii_char:AsciiPattern, 
    invert: bool, 
    edgedetec: bool, 
    brighten: bool,  
    video: bool,  
    camera: bool) -> String {
        
        let ascii_char = ascii_char;
        let mut res = String::new();
    if camera {
        let _ = handlecam(path,edgedetec,ascii_char.clone(),false);
        return String::new();
    }

    if video {
        let _ = handlecam(path,edgedetec,ascii_char.clone(),true);
        return String::new();
    }

    let img = image::open(path).expect("Image not found");
    let (width, height) = img.dimensions();
    let mut ascii_char = ascii_char.pattern().to_vec();
    if invert{ ascii_char.reverse(); }
    
    //basic preprocessing

    let img = img.resize(width/4, height/4, image::imageops::FilterType::Lanczos3);
    let (width, height) = img.dimensions();

    if edgedetec{ res = edgedetect(img,width,height,res,brighten,ascii_char); return res; }
    if color { res = color_image(img, width, height,res,ascii_char); return res; }
    else { res = normal(img,width,height,res,ascii_char); return res; }

}

fn edgedetect (img:DynamicImage,width: u32,height: u32, mut res:String,brighten: bool,ascii_char:Vec<char>) -> String{
    
    let img = img.grayscale().to_luma8();
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
                    false => ((pixel as f32) / 255.0).powf(2.0),
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

fn color_image(img:DynamicImage,width: u32,height: u32,mut res:String,ascii_char:Vec<char>) -> String{
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
    res
}

fn normal (img:DynamicImage,width: u32,height: u32,mut res:String,ascii_char:Vec<char>)->String{
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
    res
}

// fn convert_to_image(mat: &core::Mat) ->DynamicImage {
//     let size = mat.size().unwrap();
//     let rows = size.height as usize;
//     let cols = size.width as usize;

//     let data = mat.data_bytes().unwrap();
//     let image = RgbaImage::from_raw(cols as u32, rows as u32, data.to_vec())
//     .ok_or("Failed to create image from raw data");
//     DynamicImage::ImageRgba8(image.unwrap())
// }

// fn resize_to_terminal(img: DynamicImage) -> DynamicImage {
//     if let Some((width, height)) = term_size::dimensions() {
//         let new_width = width as u32 * 2;
//         let new_height = height as u32 - 2;
//         img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3)
//     } else {
//         img
//     }
// }

// pub fn handlevideo(path: &Path, ascii_char: AsciiPattern, edgedetec: bool, brighten: bool, color: bool) -> opencv::Result<()> {
//     let mut videofile = opencv::videoio::VideoCapture::from_file(&path.display().to_string(), CAP_ANY)?;
//     let mut frame = Mat::default();

//     while videofile.read(&mut frame).unwrap_or(false) {
//         let img = convert_to_image(&frame);
//         let img = resize_to_terminal(img);
//         let (width, height) = img.dimensions();
//         let output = if edgedetec {
//             edgedetect(img, width, height, String::new(), brighten, ascii_char.pattern().to_vec())
//         } else if color {
//             color_image(img, width, height, String::new(), ascii_char.pattern().to_vec())
//         } else {
//             normal(img, width, height, String::new(), ascii_char.pattern().to_vec())
//         };

//         print!("{}[2J", 27 as char); // Clear screen
//         println!("{}", output);
//     }
//     Ok(())
// }

// pub fn handlecam(ascii_char: AsciiPattern, edgedetec: bool, brighten: bool, color: bool) -> opencv::Result<()> {
//     let mut cam = VideoCapture::new(1, CAP_ANY)?;
//     let mut frame = Mat::default();

//     while cam.read(&mut frame)? {
//         let img = convert_to_image(&frame);
//         let img = resize_to_terminal(img);
//         let (width, height) = img.dimensions();
//         let output = if edgedetec {
//             edgedetect(img, width, height, String::new(), brighten, ascii_char.pattern().to_vec())
//         } else if color {
//             color_image(img, width, height, String::new(), ascii_char.pattern().to_vec())
//         } else {
//             normal(img, width, height, String::new(), ascii_char.pattern().to_vec())
//         };

//         print!("{}[2J", 27 as char); // Clear screen
//         println!("{}", output);
//     }
//     Ok(())
// }