use anyhow::Result;
use std::fs::File;
use std::io::{BufReader, Read};

pub trait BitReader {
    fn new(stream: BufReader<File>) -> Self;
    fn set_next_bit(&mut self, next_bit: usize) -> ();
    fn stream(&mut self) -> &mut BufReader<File>;
    fn read_bit(&mut self) -> Result<u8>;

    fn read_byte(&mut self) -> Result<u8> {
        self.set_next_bit(0);
        let mut buffer: [u8; 1] = [0; 1];
        self.stream().read_exact(&mut buffer)?;
        Ok(buffer[0])
    }

    fn read_word(&mut self) -> Result<u16> {
        self.set_next_bit(0);
        let mut buffer: [u8; 2] = [0; 2];
        self.stream().read_exact(&mut buffer)?;
        Ok(((buffer[0] as u16) << 8) + buffer[1] as u16)
    }

    fn read_double(&mut self) -> Result<u32> {
        self.set_next_bit(0);
        let mut buffer: [u8; 4] = [0; 4];
        self.stream().read_exact(&mut buffer)?;
        Ok(((buffer[0] as u32) << 24) + ((buffer[1] as u32) << 16) + ((buffer[2] as u32) << 8) + buffer[3] as u32)
    }

    fn read_bits(&mut self, length: usize) -> Result<i32> {
        let mut bits: i32 = 0;

        for _ in 0..length {
            let bit: u8 = self.read_bit()?;
            bits = (bits << 1) | bit as i32;
        }

        Ok(bits)
    }

    fn align(&mut self) -> () {
        self.set_next_bit(0);
    }
}
