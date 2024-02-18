// define Transformation types

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CropParameters {
    x1: usize,
    y1: usize,
    width: usize,
    height: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Transformation {
    Crop(CropParameters),
    BlackAndWhite(),
}
