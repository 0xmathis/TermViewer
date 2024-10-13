use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use clap::Parser;
use jpeg::segment::SegmentType;
use jpeg::segment::appn::APPN;
use jpeg::segment::com::COM;
use jpeg::segment::dht::DHT;
use jpeg::segment::dqt::DQT;
use jpeg::segment::sof0::SOF0;
use jpeg::segment::sos::SOS;

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

    loop {
        let mut marker: [u8; 2] = [0; 2];
        file.read_exact(&mut marker).unwrap();

        let Some(marker) = SegmentType::from_marker(marker) else {
                println!("{marker:02X?}: unknown");
                break;
        };

        match marker {
            SegmentType::APPN => {
                let segment: APPN = APPN::from_binary(&mut file);
                println!("segment {marker:?}:\n{segment:?}\n");
            },
            SegmentType::COM  => {
                let segment: COM = COM::from_binary(&mut file);
                println!("segment {marker:?}:\n{segment:?}\n");
            },
            SegmentType::DHT  => {
                let segment: DHT = DHT::from_binary(&mut file);
                println!("segment {marker:?}:\n{segment:?}\n");
            },
            SegmentType::DQT  => {
                let segment: DQT = DQT::from_binary(&mut file);
                println!("segment {marker:?}:\n{segment}\n");
            },
            SegmentType::EOI  => {
                println!("segment {marker:?}\n");
            },
            SegmentType::SOF0 => {
                let segment: SOF0 = SOF0::from_binary(&mut file);
                println!("segment {marker:?}:\n{segment}\n");
            },
            SegmentType::SOI  => {
                println!("segment {marker:?}\n");
            },
            SegmentType::SOS  => {
                let segment: SOS = SOS::from_binary(&mut file);
                println!("segment {marker:?}:\n{segment:?}\n");
            },
        }
    }
}
