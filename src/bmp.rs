use anyhow::Result;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::jpeg::header::Header;
use crate::jpeg::mcu::MCU;
use crate::jpeg::JPEG;

pub struct BMP {
}

impl BMP {
    pub fn write_to_file(jpeg: &JPEG, filename: PathBuf) -> Result<()> {
        let header: &Header = &jpeg.header;
        let mcus: &Vec<MCU> = &jpeg.mcus;
        let mcu_height: u32 = ((header.height + 7) / 8) as u32;
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

                buffer.push(mcus[mcu_index].component3[pixel_index] as u8);
                buffer.push(mcus[mcu_index].component2[pixel_index] as u8);
                buffer.push(mcus[mcu_index].component1[pixel_index] as u8);
            }

            for _ in 0..padding_size {
                buffer.push(0u8);
            }
        }

        let mut file: File = File::create(filename)?;
        file.write_all(&buffer)?;

        Ok(())
    }
}
