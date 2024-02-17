//! A simple program to be proven inside the zkVM.

// #![no_main]
// sp1_zkvm::entrypoint!(main);

use lib::Transformation;
use serde::{Deserialize, Serialize};

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

pub fn main() {
    // read in wanted transformations each as a JSON
    let transformations = sp1_zkvm::io::read::<Vec<Transformation>>();
    let mut img = 1; // TODO: change to desired image

    for transformation in transformations {
        match transformation {
            Transformation::Crop(params) => {}
            Transformation::BlackAndWhite() => {}
        }
    }

    sp1_zkvm::io::write(&img); // this should be the transformedf image
}
