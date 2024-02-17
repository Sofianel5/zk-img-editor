//! A simple script to generate and verify the proof of a given program.

use image::{DynamicImage, GenericImageView, ImageFormat, RgbaImage};
use lib::Transformation;
use sp1_core::utils;
use sp1_core::{SP1Prover, SP1Stdin, SP1Verifier};
use std::fs::File;
use std::io::Cursor;

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    utils::setup_logger();
    // Generate proof.
    let mut stdin = SP1Stdin::new();

    // TODO: make this an array of transformations
    let data = include_str!("./transformations.json"); // this is a macro and finds path
    let decoded_transformations: Transformation = serde_json::from_str(data).unwrap();

    // how we get image: https://github.com/image-rs/image/blob/master/examples/opening.rs
    let img = image::open("src/500.png").unwrap().into_rgb8(); // Dynamic Image
    let (width, height) = img.dimensions(); // each u32
                                            // let channel_count = match img.color() {
                                            //     image::ColorType::L8 => 1,
                                            //     image::ColorType::La8 => 2,
                                            //     image::ColorType::Rgb8 => 3,
                                            //     image::ColorType::Rgba8 => 4,
                                            //     _ => panic!("Unsupported color type"),
                                            // };
                                            // let buffer_size = (width as usize) * (height as usize) * (channel_count as usize);
                                            // println!("Intended buffer size: {}", buffer_size);
                                            // let mut buffer = Cursor::new(Vec::with_capacity(buffer_size));
    let buffer = img.into_raw();
    // img.write_to(&mut buffer, ImageFormat::Jpeg).unwrap();

    // Write data.
    stdin.write(&decoded_transformations); // TODO: make this "vec![decoded_transformations]"
    stdin.write(&buffer);
    stdin.write(&width);
    stdin.write(&height);

    let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");

    // Read transformed image.
    let transformed_img_buf = proof.stdout.read::<Vec<u8>>();
    print!("got here2");
    // let new_width = proof.stdout.read::<u32>();
    // let new_height = proof.stdout.read::<u32>();

    let new_img = RgbaImage::from_raw(width, height, transformed_img_buf).unwrap();
    print!("got here3");
    let fout = &mut File::create("./dog_cropped.png").unwrap();
    new_img.write_to(fout, ImageFormat::Jpeg).unwrap();

    // Verify proof.
    SP1Verifier::verify(ELF, &proof).expect("verification failed");

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("succesfully generated and verified proof for the program!")
}
