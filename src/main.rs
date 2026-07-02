use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use image::{bmp::BMP, from_file, ImageType};
use term_drawer::drawer::{draw, ScalingLevel};

mod image;
mod term_drawer;

/// TermViewer
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// File path
    filepath: PathBuf,

    /// Type of the file to process
    image_type: ImageType,

    /// Type of the file to process
    #[clap(short, long, default_value="level2")]
    scaling_level: ScalingLevel,

    /// Save intermediate BMP file
    #[clap(long)]
    save_bmp: bool,

    /// Disable rendering
    #[clap(long)]
    no_render: bool,
}

// TODO: GIF
// TODO: The drawer ask to image/video to decode next frame
// TODO: Read data from STDIN
// TODO: Be able to decode video stream
// https://yasoob.me/posts/understanding-and-writing-jpeg-decoder-in-python/#jpeg-decoding
// https://koushtav.me/jpeg/tutorial/c++/decoder/2019/03/02/lets-write-a-simple-jpeg-library-part-2/#detailed-description-of-the-markers
// https://imrannazar.com/series/lets-build-a-jpeg-decoder/huffman-tables

fn main() -> Result<()> {
    let cli: Cli = Cli::parse();
    let filepath: PathBuf = cli.filepath;

    assert_eq!(true, filepath.exists());
    assert_eq!(true, filepath.is_file());

    let file: File = File::open(&filepath)?;
    let stream: BufReader<File> = BufReader::new(file);

    let bmp: BMP<BufReader<File>> = from_file(stream, cli.image_type)?;

    if cli.save_bmp {
        let bmp_filepath: String = filepath.to_str().unwrap().to_owned() + ".bmp";
        println!("Saving intermediate BMP file as \"{bmp_filepath}\"");
        bmp.write_to_file(PathBuf::from(bmp_filepath))?;
    }

    if !cli.no_render {
        draw(bmp, cli.scaling_level)?;
    }

    Ok(())
}
