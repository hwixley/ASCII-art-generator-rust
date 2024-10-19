use image::{open, RgbaImage, DynamicImage, GrayImage, ImageBuffer, Pixel, imageops};

pub struct MyPicture {
    pub path: String,
}

impl MyPicture {
    pub fn new(path: &String) -> MyPicture {
        MyPicture { path: path.to_string() }
    }

    pub fn load_img(&self) -> RgbaImage {
        println!("Loading image with path '{}'...", self.path.clone());

        if std::path::Path::new(&self.path).is_file() {
            return open(self.path.clone()).unwrap().to_rgba8()
        } else {
            println!("File path '{}' does not exist, exiting...", self.path.clone());
            std::process::exit(1);
        }
    }

    pub fn load_gray_img(&self, nwidth: u32) -> GrayImage {
        let gray: ImageBuffer<image::Luma<u8>, Vec<u8>> = DynamicImage::ImageRgba8(MyPicture::load_img(self)).into_luma8();
        let (width, height): (u32, u32) = gray.dimensions();
        let scale: f32 = (width as f32)/(nwidth as f32);
        let new_height: f32 = (height as f32)/scale;

        println!("Parsing image into grayscale with width {}...", nwidth);

        return imageops::resize(&gray, nwidth as u32, new_height as u32, imageops::FilterType::CatmullRom)
    }

    pub fn to_ascii(&self, twidth: u32) -> String {
        let charset: String = " .:-=+*#%@".to_string();
        let img: ImageBuffer<image::Luma<u8>, Vec<u8>> = MyPicture::load_gray_img(self, twidth);
        let (width, _): (u32, u32) = img.dimensions();

        println!("Parsing image into ASCII string of width {}...", twidth);
        
        let mut ascii_out: String = "".to_string();
        for pixel in img.enumerate_pixels() {
            let pixel_val: usize = pixel.2.channels()[0] as usize;
            for i in 0..charset.len() {
                if pixel_val < 255/(charset.len()-i) {
                    ascii_out.push_str(&charset.chars().nth(i).unwrap().to_string());
                    break;
                }
            }
            if pixel.0 == width-1 {
                ascii_out.push_str("\n");
            }
        }
        return ascii_out
    }
}