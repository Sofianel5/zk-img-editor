use hex_literal::hex;
use image::{DynamicImage, GenericImageView, ImageFormat, RgbImage};
use lib::Transformation;
use sp1_core::utils;
use sp1_core::{SP1Prover, SP1Stdin, SP1Verifier};
use std::fs::File;

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    utils::setup_logger();
    // Generate proof.
    let mut stdin = SP1Stdin::new();

    // transformation
    let data = include_str!("./transformations.json");
    let decoded_transformations: Vec<Transformation> = serde_json::from_str(data).unwrap();

    // image
    let img = image::open("./src/256.png").unwrap();
    let (width, height) = img.dimensions();
    let img_buffer = img.into_bytes();

    // signature (TODO: replace dummy values)
    // reference: https://docs.rs/p256/latest/p256/ecdsa/index.html
    let signing_key = hex!("ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf");
    let ecdsa_signature: Vec<u8> = hex!("46557EFE96D22D07E104D9D7FAB558FB02F6B13116056E6D7C300D7BB132059907D538EAC68EC7864AA2AC2E23EA7082A04002B0ACDAC2FF8CCAD7E80E64DD00").to_vec();

    // Write data.
    stdin.write(&decoded_transformations);
    stdin.write(&img_buffer);
    stdin.write(&width);
    stdin.write(&height);

    stdin.write(&signing_key);
    stdin.write(&ecdsa_signature);

    let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");

    // Read transformed image.
    let transformed_img_buf = proof.stdout.read::<Vec<u8>>();
    let new_width = proof.stdout.read::<u32>();
    let new_height = proof.stdout.read::<u32>();

    println!(
        "transformed_img_bug.len() = {:?}",
        transformed_img_buf.len()
    );
    println!("new_width = {:?}", new_width);
    println!("new_height = {:?}", new_height);

    // Save transformed image.
    let new_img = RgbImage::from_raw(new_width, new_height, transformed_img_buf).unwrap();
    let fout = &mut File::create("./src/edited_img.jpg").unwrap();
    new_img.write_to(fout, ImageFormat::Jpeg).unwrap();

    // Verify proof.
    SP1Verifier::verify(ELF, &proof).expect("verification failed");

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("succesfully generated and verified proof for the program!");
    
}