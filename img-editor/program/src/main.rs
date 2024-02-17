
// TODO: use turbojpeg for compression

use lib::Transformation;
use image::{GenericImageView, RgbaImage, ImageBuffer,imageops, Rgb, DynamicImage, ImageFormat};
use std::io::Cursor;

#[no_mangle]
extern "C" fn round(x: f32) -> f32 {
    1.0f32
}

#[no_mangle]
extern "C" fn roundf(x: f32) -> f32 {
    1.0f32
}
// EXAMPLE
//  let params = CropParameters {
//     x1: 0,
//     y1: 1,
//     x2: 3,
//     y2: 3,
// };
// let t = Transformation::Crop(params);
// let res = serde_json::to_string(&t).unwrap();
// res = {"Crop":{"x1":0,"y1":1,"x2":3,"y2":3}}

/* Crops the image */
pub fn crop() {}

pub fn black_and_white() {}

// TODO: need to change this to also have write path
// struct Image {
//     buffer: [u8]
// }

// impl Image {
//     fn crop(&self) {
//     }
//     fn black_and_white(&self) {
//     }
// }

pub fn main() {
    let transformations = sp1_zkvm::io::read::<Transformation>(); // TODO: make this Vec<Transformation> instead
    println!("here1");
    let img_buf = sp1_zkvm::io::read::<Vec<u8>>(); 
    println!("here2");
    let width = sp1_zkvm::io::read::<u32>();
    let height = sp1_zkvm::io::read::<u32>();

    // let mut img = RgbaImage::new(width, height);
    
    // for transformation in transformations {
    //     match transformation {
    //         Transformation::Crop(params) => image.crop(),
    //         Transformation::BlackAndWhite() => image.black_and_white(),

    //     }
    // }

    println!("{:?}", img_buf);

    let img = DynamicImage::ImageRgba8(ImageBuffer::from_raw(width, height, img_buf).unwrap());
    // let img = rgba.into_luma8(); 

    // let img = ImageBuffer::from_raw(width, height, img_buf).unwrap();
    let mut buffer = Cursor::new(Vec::new());
    img.write_to(&mut buffer, ImageFormat::Jpeg).unwrap();

    sp1_zkvm::io::write(&buffer.into_inner()); // write back as raw bytes

    // let new_width = 100;
    // let new_height = 100;
    // sp1_zkvm::io::write(&new_width);
    // sp1_zkvm::io::write(&new_height);
}
