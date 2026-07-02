use anyhow::Result;
use clap::ValueEnum;
use serde::Serialize;
use std::io::Read;

use bmp::BMP;
use jpeg::JPEG;

mod bit_reader;
mod huffman;
mod jpeg;
mod mcu_component;
mod quantization_table;
pub mod bmp;
pub mod mcu;

#[derive(ValueEnum, Clone, Debug, Serialize)]
pub enum ImageType {
    BMP,
    JPEG,
}

pub trait Image<T>
where
    T: Read,
    Self: Sized,
{
    fn from_stream(stream: T) -> Result<Self>;
    fn to_bmp(self) -> BMP<T>;
}

pub fn from_file<T: Read>(stream: T, image_type: ImageType) -> Result<BMP<T>> {
    match image_type {
        ImageType::BMP => BMP::from_stream(stream),
        ImageType::JPEG => Ok(JPEG::from_stream(stream)?.to_bmp()),
    }
}
