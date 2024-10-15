use std::fs::File;
use std::path::PathBuf;

use clap::Parser;

use jpeg::JPEG;

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

fn main() {
    let args: Args = Args::parse();
    let filepath: PathBuf = args.filepath;

    assert!(filepath.exists());
    assert!(filepath.is_file());

    let mut file: File = File::open(filepath).unwrap();
    let jpeg: JPEG = JPEG::from_file(&mut file);

    println!("\n{jpeg}");
}
