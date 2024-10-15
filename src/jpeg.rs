use std::fmt::Display;
use std::fs::File;
use std::io::Read;

use header::Header;
use segment::SegmentType;

mod header;
mod huffman_table;
mod color_component;
mod quantization_table;
mod segment;

#[derive(Debug, Default)]
pub struct JPEG {
    header: Header,
    huffman_data: Vec<u8>,
}

impl JPEG {
    pub fn from_file(file: &mut File) -> Self {
        Self {
            header: Header::from_binary(file),
            huffman_data: Self::read_huffman_data(file),
        }
    }

    fn read_huffman_data(file: &mut File) -> Vec<u8> {
        let mut huffman_data: Vec<u8> = Vec::new();
        let mut current: [u8; 1] = [0; 1];
        let mut previous: [u8; 1];

        file.read_exact(&mut current).unwrap();

        loop {
            previous = current;
            file.read_exact(&mut current).unwrap();

            // marker is found
            if previous == [0xFF] {
                if SegmentType::from_marker([0xFF, current[0]]) == Some(SegmentType::EOI) {
                    break;
                } else if current == [0x00] { // 0xFF00 mean 0xFF is data from huffman stream
                    huffman_data.push(previous[0]);
                    file.read_exact(&mut current).unwrap();
                } else if SegmentType::from_marker([0xFF, current[0]]) == Some(SegmentType::RSTN) {
                    file.read_exact(&mut current).unwrap();
                } else if current == [0xFF] { // ignore multiple 0xFF in a row
                    continue;
                } else {
                    panic!("Invalid marker during compressed data scan: 0xFF{}", current[0]);
                }
            } else {
                huffman_data.push(previous[0]);
            }
        }

        huffman_data
    }
}

impl Display for JPEG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Header:\n\n{}\n", self.header)?;
        write!(f, "Huffman data length: {}\n", self.huffman_data.len())
    }
}
