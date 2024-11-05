use anyhow::Result;
use clap::ValueEnum;
use serde::Serialize;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use bmp::BMP;
use jpeg::JPEG;
use mcu::MCU;

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
    fn from_stream(reader: BufReader<File>) -> Result<Self> where Self: Sized;
    fn to_bmp(&mut self) -> Result<BMP>;
    fn mcus(&self) -> &Vec<MCU>;
}

pub fn from_file(filepath: &PathBuf, image_type: ImageType) -> Result<Box<dyn Image>> {
    let file: File = File::open(filepath)?;
    let reader: BufReader<File> = BufReader::new(file);

    match image_type {
        ImageType::BMP => Ok(Box::new(BMP::from_stream(reader)?)),
        ImageType::JPEG => Ok(Box::new(JPEG::from_stream(reader)?)),
        ImageType::PNG => todo!(),
    }
}

