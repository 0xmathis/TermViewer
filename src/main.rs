use anyhow::Result;
use serde::Serialize;
use std::fs::File;
use std::path::PathBuf;

use bmp::BMP;
use clap::{Parser, ValueEnum};

use jpeg::JPEG;

mod bmp;
mod jpeg;

#[derive(ValueEnum, Clone, Debug, Serialize)]
enum Type {
    /// JPEG
    JPEG,

    /// PNG
    PNG,

    /// BMP
    BMP,
}

/// TermViewer
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// File
    filepath: PathBuf,

    /// File type
    #[clap(default_value = "jpeg")]
    filetype: Option<Type>,
}

// TODO: Be able to draw in the terminal (with colors)
// TODO: Be able to decode image
// TODO: Be able to decode video stream
// https://yasoob.me/posts/understanding-and-writing-jpeg-decoder-in-python/#jpeg-decoding
// https://koushtav.me/jpeg/tutorial/c++/decoder/2019/03/02/lets-write-a-simple-jpeg-library-part-2/#detailed-description-of-the-markers
// https://imrannazar.com/series/lets-build-a-jpeg-decoder/huffman-tables

fn main() -> Result<()> {
    let args: Args = Args::parse();
    let filepath: PathBuf = args.filepath;
    
    match args.filetype.expect("Should not be None") {
        Type::JPEG => Type::JPEG,
        Type::PNG => todo!(),
        Type::BMP => todo!(),
    };

    assert!(filepath.exists());
    assert!(filepath.is_file());

    let mut file: File = File::open(&filepath).unwrap();
    let jpeg: JPEG = JPEG::from_file(&mut file)?;
    // println!("\n{jpeg}");

    let bmp: BMP = jpeg.to_bmp()?;

    let mut bmp_filepath = filepath.to_str().unwrap().to_owned();
    bmp_filepath.push_str(".bmp");
    bmp.write_to_file(PathBuf::from(bmp_filepath))
}
