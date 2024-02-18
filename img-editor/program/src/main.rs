// TODO: use turbojpeg for compression

use image::{imageops, ImageBuffer, RgbImage};
use lib::Transformation;
use p256::ecdsa::{signature::Verifier, VerifyingKey};

#[no_mangle]
extern "C" fn round(x: f32) -> f32 {
    let scale: f32 = 10_f32.powi(0);
    (x * scale).round() / scale
}

#[no_mangle]
extern "C" fn roundf(x: f32) -> f32 {
    let offset = x % 1.0f32;

    if offset >= 0.5 {
        return x + (1.0f32 - offset);
    }
    return x - offset;
}

pub fn main() {
    let transformation = sp1_zkvm::io::read::<Transformation>();
    let img_buf = sp1_zkvm::io::read::<Vec<u8>>();
    let width = sp1_zkvm::io::read::<u32>();
    let height = sp1_zkvm::io::read::<u32>();

    let signing_key = sp1_zkvm::io::read::<Vec<u8>>(); // serialized
    let signature = sp1_zkvm::io::read::<Vec<u8>>(); // signature

    // crop image
    println!("Before reading image");
    println!("width: {:?}", width);
    println!("height: {:?}", height);
    println!("img_buf.len(): {:?}", img_buf.len());
    let mut img: RgbImage = ImageBuffer::from_raw(width, height, img_buf).unwrap();
    println!("Finished reading image");

    let new_width = 100;
    let new_height = 50;

    let cropped_img = imageops::crop(&mut img, 0, 0, new_wdith, new_height).to_image();
    let img_buffer = cropped_img.as_raw();

    // Verify signature.
    let verifying_key = VerifyingKey::from(&signing_key);
    let sig_verified = verifying_key.verify(message, &signature).is_ok();

    // Write back cropped image.
    sp1_zkvm::io::write(&img_buffer);
    sp1_zkvm::io::write(&new_width);
    sp1_zkvm::io::write(&new_height);

    //
}

/*
FUTURE ROADMAP: support list of transformations

    for transformation in transformations {
        match transformation {
            Transformation::Crop(params) => image.crop(),
            Transformation::BlackAndWhite() => image.black_and_white(),
        }
    }
*/
