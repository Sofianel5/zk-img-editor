// define Transformation types

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CropParameters {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlurParameters {
    pub sigma: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BrightenParameters {
    pub value: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContrastParameters {
    pub contrast: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Transformation {
    Crop(CropParameters),
    Grayscale(),
    Rotate90(),
    Rotate180(),
    Rotate270(),
    FlipVertical(),
    FlipHorizontal(),
    Brighten(BrightenParameters),
    Constrast(ContrastParameters),
}
