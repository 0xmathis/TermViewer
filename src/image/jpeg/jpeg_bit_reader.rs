use anyhow::{bail, Result};
use std::fs::File;
use std::io::BufReader;

use crate::image::bit_reader::BitReader;
use super::segment::SegmentType;

#[derive(Debug)]
pub struct JpegBitReader {
    next_bit: usize,
    current_byte: u8,
    next_byte: u8,
    stream: BufReader<File>,
}

impl BitReader for JpegBitReader {
    fn new(stream: BufReader<File>) -> Self {
        Self {
            next_bit: 0,
            current_byte: 0,
            next_byte: 0,
            stream,
        }
    }

    fn read_bit(&mut self) -> Result<u8> {
        if self.next_bit == 0 {
            self.current_byte = self.read_byte()?;

            // marker is found
            while self.current_byte == 0xFF {
                self.next_byte = self.read_byte()?;

                while self.next_byte == 0xFF {
                    self.next_byte = self.read_byte()?;
                }

                if self.next_byte == 0x00 { // 0xFF00 mean 0xFF is data from huffman stream
                    break;
                } else if SegmentType::from_marker(0xFF00u16 + self.next_byte as u16) == Some(SegmentType::RSTN) {
                    self.current_byte = self.read_byte()?;
                }
                else {
                    bail!("Invalid marker during compressed data scan: 0xFF{:02X}", self.current_byte);
                }
            }
        }

        let bit: u8 = (self.current_byte >> (7 - self.next_bit)) & 0x1;
        self.next_bit += 1;
        self.next_bit %= 8;

        Ok(bit)
    }

    fn set_next_bit(&mut self, next_bit: usize) -> () {
        self.next_bit = next_bit;
    }

    fn stream(&mut self) -> &mut BufReader<File> {
        &mut self.stream
    }
}
