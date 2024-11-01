use anyhow::Result;
use clap::ValueEnum;
use serde::Serialize;
use std::fs::File;
use std::path::PathBuf;


use bmp::BMP;
use jpeg::JPEG;

pub mod bmp;
pub mod jpeg;

#[derive(ValueEnum, Clone, Debug, Serialize)]
pub enum ImageType {
    JPEG,
    PNG,
    BMP,
}

pub trait Image {
    fn from_file(file: File) -> Result<Self> where Self: Sized;
    fn to_bmp(&mut self) -> Result<BMP>;
}

pub fn from_file(filepath: &PathBuf, image_type: ImageType) -> Result<Box<dyn Image>> {
    let file: File = File::open(filepath)?;

    match image_type {
        ImageType::BMP => Ok(Box::new(BMP::from_file(file)?)),
        ImageType::JPEG => Ok(Box::new(JPEG::from_file(file)?)),
        ImageType::PNG => todo!(),
    }
}

