// TODO: use turbojpeg for compression

use image::{imageops, ImageBuffer, RgbImage};
use lib::Transformation;
use p256::ecdsa::{signature::Verifier, Signature, VerifyingKey};

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
    let old_width = sp1_zkvm::io::read::<u32>();
    let old_height = sp1_zkvm::io::read::<u32>();

    let signing_key = sp1_zkvm::io::read::<Vec<u8>>();
    let signature = sp1_zkvm::io::read::<Vec<u8>>();

    // Verify signature.
    let verifying_key = VerifyingKey::from_sec1_bytes(&signing_key).unwrap();
    assert!(verifying_key
        .verify(&img_buf, &Signature::try_from(&signature[..]).unwrap())
        .is_ok());

    // crop image.
    println!("Before reading image");
    println!("width: {:?}", old_width);
    println!("height: {:?}", old_height);
    println!("img_buf.len(): {:?}", img_buf.len());
    let mut img: RgbImage = ImageBuffer::from_raw(old_width, old_height, img_buf).unwrap();
    println!("Finished reading image");

    let mut new_width = old_width;
    let mut new_height = old_height;

    let cropped_img;

    match transformation {
        Transformation::Crop(params) => {
            new_width = params.width;
            new_height = params.height;
            cropped_img =
                imageops::crop(&mut img, params.x, params.y, new_width, new_height).to_image();
        }
    };
    let img_buffer = cropped_img.as_raw();

    // Write back cropped image.
    sp1_zkvm::io::write(&img_buffer);
    sp1_zkvm::io::write(&new_width);
    sp1_zkvm::io::write(&new_height);
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
