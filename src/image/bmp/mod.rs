use anyhow::Result;
use std::fmt;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use bmp_bit_reader::BmpBitReader;
use crate::image::bit_reader::BitReader;
use header::BMPHeader;
use super::Image;
use super::mcu::MCU;

pub mod bmp_bit_reader;
pub mod header;

#[derive(Debug)]
pub struct BMP<T: Read> {
    header: BMPHeader,
    mcus: Vec<MCU>,
    stream: BmpBitReader<T>,
}

impl<T: Read> BMP<T> {
    pub fn new(header: BMPHeader, mcus: Vec<MCU>, stream: BmpBitReader<T>) -> Self {
        Self {
            header,
            mcus,
            stream,
        }
    }

    pub fn write_to_file(&self, filename: PathBuf) -> Result<()> {
        let header: &BMPHeader = &self.header;
        let mcus: &Vec<MCU> = &self.mcus;
        let mcu_width: u32 = ((header.width + 7) / 8) as u32;
        let padding_size: u32 = (header.width % 4) as u32;
        let size: u32 = 14u32 + 12u32 + header.width as u32 * header.height as u32 * 3 + padding_size * header.height as u32;
        let mut buffer: Vec<u8> = Vec::with_capacity(size as usize);

        buffer.push(b'B');
        buffer.push(b'M');
        buffer.extend_from_slice(&size.to_le_bytes());
        buffer.extend_from_slice(&0u32.to_le_bytes());
        buffer.extend_from_slice(&0x1Au32.to_le_bytes());
        buffer.extend_from_slice(&12u32.to_le_bytes());
        buffer.extend_from_slice(&header.width.to_le_bytes());
        buffer.extend_from_slice(&header.height.to_le_bytes());
        buffer.extend_from_slice(&1u16.to_le_bytes());
        buffer.extend_from_slice(&24u16.to_le_bytes());

        for y in (0..header.height).rev() {
            let mcu_row: u32 = y as u32 / 8;
            let pixel_row: u32 = y as u32 % 8;

            for x in 0..header.width {
                let mcu_column: u32 = x as u32 / 8;
                let pixel_column: u32 = x as u32 % 8;
                let mcu_index: usize = (mcu_row * mcu_width + mcu_column) as usize;
                let pixel_index: usize = (pixel_row * 8 + pixel_column) as usize;

                buffer.push(mcus[mcu_index].component(2).expect("Should exist")[pixel_index] as u8);
                buffer.push(mcus[mcu_index].component(1).expect("Should exist")[pixel_index] as u8);
                buffer.push(mcus[mcu_index].component(0).expect("Should exist")[pixel_index] as u8);
            }

            for _ in 0..padding_size {
                buffer.push(0u8);
            }
        }

        let mut file: File = File::create(filename)?;
        file.write_all(&buffer)?;

        Ok(())
    }

    fn read_components(&mut self) -> Result<()> {
        let mut count: u32 = 0;

        let padding_size: u32 = (self.header.width % 4) as u32;
        let mcu_width: u32 = ((self.header.width + 7) / 8) as u32;
        let mcu_height: u32 = ((self.header.height + 7) / 8) as u32;

        self.mcus.resize(mcu_height as usize * mcu_width as usize, MCU::default());

        for y in (0..self.header.height).rev() {
            let mcu_row: u32 = y as u32 / 8;
            let pixel_row: u32 = y as u32 % 8;

            for x in 0..self.header.width {
                let mcu_column: u32 = x as u32 / 8;
                let pixel_column: u32 = x as u32 % 8;
                let mcu_index: usize = (mcu_row * mcu_width + mcu_column) as usize;
                let pixel_index: usize = (pixel_row * 8 + pixel_column) as usize;

                self.mcus[mcu_index]
                    .component_mut(2)
                    .expect("Should exist")
                    [pixel_index] = self.stream.read_byte()? as i32;
                count += 1;

                self.mcus[mcu_index]
                    .component_mut(1)
                    .expect("Should exist")
                    [pixel_index] = self.stream.read_byte()? as i32;
                count += 1;

                self.mcus[mcu_index]
                    .component_mut(0)
                    .expect("Should exist")
                    [pixel_index] = self.stream.read_byte()? as i32;
                count += 1;
            }

            for _ in 0..padding_size {
                self.stream.read_byte()?;
                count += 1;
            }
        }

        assert_eq!(self.header.bmp_size - self.header.starting_offset, count);

        Ok(())
    }

    pub fn width(&self) -> u16 {
        self.header.width
    }

    pub fn height(&self) -> u16 {
        self.header.height
    }

    pub fn mcus(&self) -> &Vec<MCU> {
        &self.mcus
    }
}

impl<T: Read> Image<T> for BMP<T> {
    fn from_stream(stream: T, _debug: bool) -> Result<Self> {
        let mut stream = BmpBitReader::new(stream);
        let mut bmp: Self = Self {
            header: BMPHeader::from_binary(&mut stream)?,
            mcus: Vec::new(),
            stream,
        };

        bmp.read_components()?;

        Ok(bmp)
    }

    fn to_bmp(self: Self) -> BMP<T> {
        self
    }
}

impl<T: Read> fmt::Display for BMP<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Header:\n{}", self.header)
    }
}
