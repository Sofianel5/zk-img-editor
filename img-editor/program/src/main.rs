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
    let transformations = sp1_zkvm::io::read::<Vec<Transformation>>();
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
    let img: RgbImage = ImageBuffer::from_raw(old_width, old_height, img_buf).unwrap();
    println!("Finished reading image");

    let mut new_width = old_width;
    let mut new_height = old_height;

    let mut transformed_img = img;
    for transformation in transformations {
        match transformation {
            Transformation::Crop(params) => {
                new_width = params.width;
                new_height = params.height;
                transformed_img = imageops::crop(
                    &mut transformed_img,
                    params.x,
                    params.y,
                    new_width,
                    new_height,
                )
                .to_image();
            }
            Transformation::Grayscale() => {
                transformed_img = RgbImage::from_raw(
                    new_width,
                    new_height,
                    imageops::grayscale(&transformed_img).as_raw().to_vec(),
                )
                .unwrap();
            }
            Transformation::Rotate90() => {
                transformed_img = imageops::rotate90(&transformed_img);
            }
            Transformation::Rotate180() => {
                transformed_img = imageops::rotate180(&transformed_img);
            }
            Transformation::Rotate270() => {
                transformed_img = imageops::rotate270(&transformed_img);
            }
            Transformation::FlipVertical() => {
                transformed_img = imageops::flip_vertical(&transformed_img);
            }
            Transformation::FlipHorizontal() => {
                transformed_img = imageops::flip_horizontal(&transformed_img);
            }
            Transformation::Brighten(params) => {
                transformed_img = imageops::brighten(&transformed_img, params.value)
            }
            Transformation::Constrast(params) => {
                transformed_img = imageops::contrast(&transformed_img, params.contrast);
            }
        };
    }
    let img_buffer = transformed_img.as_raw();

    // Write back cropped image.
    sp1_zkvm::io::write(&img_buffer);
    sp1_zkvm::io::write(&new_width);
    sp1_zkvm::io::write(&new_height);
}
