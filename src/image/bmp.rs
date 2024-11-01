use anyhow::Result;
use header::BMPHeader;
use std::fmt;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use super::Image;
use super::jpeg::mcu::MCU;

pub mod header;

#[derive(Clone, Debug)]
pub struct BMP {
    header: BMPHeader,
    mcus: Vec<MCU>,
}

impl BMP {
    pub fn new(header: BMPHeader, mcus: Vec<MCU>) -> Self {
        Self {
            header,
            mcus,
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

    fn read_components(&mut self, file: &mut File) -> Result<()> {
        let mut buffer: [u8; 1] = [0; 1];
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

                file.read_exact(&mut buffer)?;
                self.mcus[mcu_index].component_mut(2).expect("Should exist")[pixel_index] = buffer[0] as i32;
                count += 1;

                file.read_exact(&mut buffer)?;
                self.mcus[mcu_index].component_mut(1).expect("Should exist")[pixel_index] = buffer[0] as i32;
                count += 1;

                file.read_exact(&mut buffer)?;
                self.mcus[mcu_index].component_mut(0).expect("Should exist")[pixel_index] = buffer[0] as i32;
                count += 1;
            }

            for _ in 0..padding_size {
                file.read_exact(&mut buffer)?;
                count += 1;
            }
        }

        assert_eq!(self.header.bmp_size - self.header.starting_offset, count);

        Ok(())
    }
}

impl Image for BMP {
    fn from_file(mut file: File) -> Result<Self> {
        let mut bmp: Self = Self {
            header: BMPHeader::from_binary(&mut file)?,
            mcus: Vec::new(),
        };

        bmp.read_components(&mut file)?;

        Ok(bmp)
    }

    fn to_bmp(&mut self) -> Result<BMP> {
        Ok(self.clone())
    }
}

impl fmt::Display for BMP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Header:\n{}\n", self.header)
    }
}
