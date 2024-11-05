use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use image::bmp::BMP;
use image::{from_file, ImageType};
use term_drawer::drawer::{draw, ScalingLevel};

mod image;
mod term_drawer;

/// TermViewer
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// File
    filepath: PathBuf,

    /// Type of the file to process
    image_type: ImageType,

    /// Type of the file to process
    #[clap(short, long, default_value="level2")]
    scaling_level: ScalingLevel,

    /// Save intermediate BMP file
    #[clap(long)]
    save_bmp: bool,

    /// Enable debug
    #[clap(long)]
    debug: bool,

    /// Disable rendering
    #[clap(long)]
    no_render: bool,
}

// TODO: Be able to decode video stream
// https://yasoob.me/posts/understanding-and-writing-jpeg-decoder-in-python/#jpeg-decoding
// https://koushtav.me/jpeg/tutorial/c++/decoder/2019/03/02/lets-write-a-simple-jpeg-library-part-2/#detailed-description-of-the-markers
// https://imrannazar.com/series/lets-build-a-jpeg-decoder/huffman-tables

fn main() -> Result<()> {
    let args: Args = Args::parse();
    let filepath: PathBuf = args.filepath;

    assert!(filepath.exists());
    assert!(filepath.is_file());

    let bmp: Box<BMP> = from_file(&filepath, args.image_type)?.to_bmp();

    if args.save_bmp {
        let mut bmp_filepath: String = filepath.to_str().unwrap().to_owned();
        bmp_filepath.push_str(".bmp");
        println!("Saving intermediate BMP file as \"{bmp_filepath}\"");
        bmp.write_to_file(PathBuf::from(bmp_filepath))?;
    }

    if !args.no_render {
        draw(bmp, args.scaling_level)?;
    }

    Ok(())
}
