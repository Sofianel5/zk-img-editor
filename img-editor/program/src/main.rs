// TODO: use turbojpeg for compression

use image::{imageops, EncodableLayout, ImageBuffer, RgbImage};
use lib::Transformation;

#[no_mangle]
extern "C" fn round(x: f32) -> f32 {
    let offset = x % 1.0f32;

    if offset >= 0.5 {
        return x + (1.0f32 - offset);
    }
    return x - offset;
}

#[no_mangle]
extern "C" fn roundf(x: f32) -> f32 {
    let offset = x % 1.0f32;

    if offset >= 0.5 {
        return x + (1.0f32 - offset);
    }
    return x - offset;
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
    let img_buf = sp1_zkvm::io::read::<Vec<u8>>();
    let width = sp1_zkvm::io::read::<u32>();
    let height = sp1_zkvm::io::read::<u32>();

    // let mut img = RgbaImage::new(width, height);

    // for transformation in transformations {
    //     match transformation {
    //         Transformation::Crop(params) => image.crop(),
    //         Transformation::BlackAndWhite() => image.black_and_white(),

    //     }
    // }

    // println!("{:?}", img_buf);

    let mut img: RgbImage = ImageBuffer::from_raw(width, height, img_buf).unwrap();

    let cropped_img = imageops::crop(&mut img, 0, 0, 100, 100).to_image();
    let img_buffer = cropped_img.as_bytes();

    sp1_zkvm::io::write(&img_buffer); // write back as raw bytes

    // let new_width = 100;
    // let new_height = 100;
    // sp1_zkvm::io::write(&new_width);
    // sp1_zkvm::io::write(&new_height);
}
