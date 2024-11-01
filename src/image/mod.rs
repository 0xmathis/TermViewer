use anyhow::Result;
use clap::ValueEnum;
use mcu::MCU;
use serde::Serialize;
use std::fs::File;
use std::path::PathBuf;

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
    JPEG,
    PNG,
    BMP,
}

pub trait Image {
    fn from_file(file: File) -> Result<Self> where Self: Sized;
    fn to_bmp(&mut self) -> Result<BMP>;
    fn width(&self) -> u16;
    fn height(&self) -> u16;
    fn mcus(&self) -> &Vec<MCU>;
}

pub fn from_file(filepath: &PathBuf, image_type: ImageType) -> Result<Box<dyn Image>> {
    let file: File = File::open(filepath)?;

    match image_type {
        ImageType::BMP => Ok(Box::new(BMP::from_file(file)?)),
        ImageType::JPEG => Ok(Box::new(JPEG::from_file(file)?)),
        ImageType::PNG => todo!(),
    }
}

