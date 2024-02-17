//! A simple script to generate and verify the proof of a given program.

use lib::Transformation;
use sp1_core::{SP1Prover, SP1Stdin, SP1Verifier};
use sp1_core::utils;
use image::{GenericImageView, DynamicImage, ImageFormat, RgbaImage};
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
    let img = image::open("src/dog.jpg").unwrap(); // Dynamic Image
    let (width, height) = img.dimensions(); // each u32

    let mut buffer = Cursor::new(Vec::new());
    img.write_to(&mut buffer, ImageFormat::Jpeg).unwrap();

    // Write data.
    stdin.write(&decoded_transformations);  // TODO: make this "vec![decoded_transformations]" 
    stdin.write(&buffer.into_inner()); 
    stdin.write(&width); 
    stdin.write(&height);

    let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");

    // Read transformed image.
    let transformed_img_buf = proof.stdout.read::<Vec<u8>>();
    // let new_width = proof.stdout.read::<u32>();
    // let new_height = proof.stdout.read::<u32>();
    
    let new_img = RgbaImage::from_raw(width, height, transformed_img_buf).unwrap();
    print!("got here3");
    let fout = &mut File::create("./dog_cropped.png").unwrap(); // write cropped image
    new_img.write_to(fout, ImageFormat::Jpeg).unwrap();

    // Verify proof.
    SP1Verifier::verify(ELF, &proof).expect("verification failed");

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("succesfully generated and verified proof for the program!")
}
