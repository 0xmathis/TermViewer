use anyhow::Result;
use clap::Parser;
use term_drawer::drawer::draw;
use std::path::PathBuf;

use image::{from_file, ImageType};
use image::bmp::BMP;

mod image;
mod term_drawer;

/// TermViewer
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// File
    filepath: PathBuf,

    /// File type
    #[clap(default_value = "jpeg")]
    filetype: Option<ImageType>,
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
    let image_type: ImageType = args.filetype.expect("Should not be None");

    assert!(filepath.exists());
    assert!(filepath.is_file());

    let bmp: BMP = from_file(&filepath, image_type)?.to_bmp()?;
    // println!("{bmp}");
    let mut bmp_filepath = filepath.to_str().unwrap().to_owned();
    bmp_filepath.push_str(".bmp");
    bmp.write_to_file(PathBuf::from(bmp_filepath))?;

    draw(bmp);

    Ok(())
}
