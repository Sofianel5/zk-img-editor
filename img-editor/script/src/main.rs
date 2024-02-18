//! A simple script to generate and verify the proof of a given program.

use std::fs::File;

use hex;
use image::{DynamicImage, GenericImageView, RgbImage};
use lib::Transformation;
use sp1_core::utils::BabyBearBlake3;
use sp1_core::SP1ProofWithIO;
use sp1_core::{SP1Prover, SP1Stdin};
use rocket::form::Form;
use rocket::fs::{FileServer, TempFile};
use sha2::{Sha256, Digest};


#[macro_use] extern crate rocket;

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn generate_proof(tranformations: Vec<Transformation>, img: DynamicImage, public_key: Vec<u8>, ecdsa_signature: Vec<u8>) -> SP1ProofWithIO<BabyBearBlake3> {
    let mut stdin = SP1Stdin::new();
    let (width, height) = img.dimensions();
    let img_buffer = img.into_bytes();

    stdin.write(&tranformations);
    stdin.write(&img_buffer);
    stdin.write(&width);
    stdin.write(&height);
    stdin.write(&public_key);
    stdin.write(&ecdsa_signature);

    return SP1Prover::prove(ELF, stdin).expect("proving failed");
}

fn package_proof(mut proof: SP1ProofWithIO<BabyBearBlake3>, index: String) {
    let transformed_img_buf = proof.stdout.read::<Vec<u8>>();
    let new_width = proof.stdout.read::<u32>();
    let new_height = proof.stdout.read::<u32>();

    let new_img = RgbImage::from_raw(new_width, new_height, transformed_img_buf).unwrap();

    proof
        .save(format!("./proofs/{index}/proof.json").as_str())
        .expect("saving proof failed");

    let fout = &mut File::create(format!("./proofs/{index}/edited_img.jpg").as_str()).unwrap();

    new_img.write_to(fout, image::ImageFormat::Jpeg).unwrap();
}

#[derive(FromForm)]
struct ProofRequest<'r> {
    transformations: String,
    img_buffer: TempFile<'r>,
    public_key: Vec<u8>,
    ecdsa_signature: Vec<u8>,
}

#[post("/", data = "<proof_request>")]
fn prove(proof_request: Form<ProofRequest<'_>>) -> String {
    let transformations: Vec<Transformation> = serde_json::from_str(&proof_request.transformations).unwrap();
    let img = image::open(proof_request.img_buffer.path().unwrap()).unwrap();
    let public_key = &proof_request.public_key;
    let ecdsa_signature = &proof_request.ecdsa_signature;

    let proof = generate_proof(transformations, img, public_key.to_owned(), ecdsa_signature.to_owned());

    let mut hasher = Sha256::new();

    hasher.update(std::fs::read(proof_request.img_buffer.path().unwrap()).unwrap());
    let result = hasher.finalize().to_vec();

    package_proof(proof, hex::encode(result.clone()));

    return hex::encode(result);
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .configure(rocket::Config::figment().merge(("port", 9999)))
        .mount("/", routes![prove])
        .mount("/proofs", FileServer::from("proofs/"))
        .launch()
        .await?;

    Ok(())
}
