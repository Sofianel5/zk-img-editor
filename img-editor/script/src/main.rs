//! A simple script to generate and verify the proof of a given program.

use lib::Transformation;
use sp1_core::{SP1Prover, SP1Stdin, SP1Verifier};
use image::{GenericImageView, RgbaImage, ImageFormat};
use std::fs::File;

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Generate proof.
    let mut stdin = SP1Stdin::new();

    // TODO: make this an array of transformations 
    let data = include_str!("./transformations.json");
    print!("{}", data);

    let decoded_transformations: Transformation = serde_json::from_str(data).unwrap(); 
    print!("{}", serde_json::to_string(&decoded_transformations).unwrap());

    // how we get image: https://github.com/image-rs/image/blob/master/examples/opening.rs
    let img = image::open("./dog.jpg").unwrap(); // not working??????????
    let (width, height) = img.dimensions(); // each u32
    print!("{}", width);

    // Write data.
    stdin.write(&img.as_bytes()); 
    stdin.write(&decoded_transformations);  // TODO: make this "vec![decoded_transformations]" 
    stdin.write(&width); 
    stdin.write(&height);

    let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");

    // Read transformed image.
    let transformed_img_buf = proof.stdout.read::<Vec<u8>>();
    // let new_width = proof.stdout.read::<u32>();
    // let new_height = proof.stdout.read::<u32>();
    
    let new_img = RgbaImage::from_raw(width, height, transformed_img_buf).unwrap();
    let fout = &mut File::create("./cat_cropped.png").unwrap();
    new_img.write_to(fout, ImageFormat::Jpeg).unwrap();

    // Verify proof.
    SP1Verifier::verify(ELF, &proof).expect("verification failed");

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("succesfully generated and verified proof for the program!")
}
