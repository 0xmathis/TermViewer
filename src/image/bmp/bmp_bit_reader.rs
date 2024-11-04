use anyhow::Result;
use std::fs::File;
use std::io::BufReader;

use crate::image::bit_reader::BitReader;

#[derive(Debug)]
pub struct BmpBitReader {
    next_bit: usize,
    current_byte: u8,
    stream: BufReader<File>,
}

impl BitReader for BmpBitReader {
    fn new(stream: BufReader<File>) -> Self {
        Self {
            next_bit: 0,
            current_byte: 0,
            stream,
        }
    }

    fn read_bit(&mut self) -> Result<u8> {
        if self.next_bit == 0 {
            self.current_byte = self.read_byte()?;
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
