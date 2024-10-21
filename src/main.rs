use std::fs::File;
use std::io;
use std::path::PathBuf;

use bmp::BMP;
use clap::Parser;

use jpeg::huffman::huffman_decoder;
use jpeg::mcu::MCU;
use jpeg::JPEG;

mod bmp;
mod jpeg;

/// TermViewer
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// File
    filepath: PathBuf,
}

// TODO: Be able to draw in the terminal (with colors)
// TODO: Be able to decode image
// TODO: Be able to decode video stream
// https://yasoob.me/posts/understanding-and-writing-jpeg-decoder-in-python/#jpeg-decoding
// https://koushtav.me/jpeg/tutorial/c++/decoder/2019/03/02/lets-write-a-simple-jpeg-library-part-2/#detailed-description-of-the-markers
// https://imrannazar.com/series/lets-build-a-jpeg-decoder/huffman-tables

fn main() -> io::Result<()> {
    let args: Args = Args::parse();
    let filepath: PathBuf = args.filepath;

    assert!(filepath.exists());
    assert!(filepath.is_file());

    let mut file: File = File::open(&filepath).unwrap();
    let mut jpeg: JPEG = JPEG::from_file(&mut file);
    // println!("\n{jpeg}");

    let mcus: Vec<MCU> = huffman_decoder(&mut jpeg).expect("Should not panic");
    let mut bmp_filepath = filepath.to_str().unwrap().to_owned();
    bmp_filepath.push_str(".bmp");
    BMP::write_to_file(&jpeg.header, mcus, PathBuf::from(bmp_filepath))
}
