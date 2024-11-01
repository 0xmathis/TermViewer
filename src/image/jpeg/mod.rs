use anyhow::{bail, Result};
use std::f32::consts::PI;
use std::fmt;
use std::fs::File;
use std::io::Read;

use color_component::ColorComponent;
use header::JPEGHeader;
use segment::SegmentType;
use super::Image;
use super::bit_reader::BitReader;
use super::bmp::BMP;
use super::mcu::MCU;
use super::quantization_table::QuantizationTable;

mod color_component;
mod segment;
mod header;

#[derive(Debug, Default)]
pub struct JPEG {
    header: JPEGHeader,
    huffman_data: Vec<u8>,
    mcus: Vec<MCU>,
}

impl JPEG {
    fn read_huffman_data(file: &mut File) -> Result<Vec<u8>> {
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
                    bail!("Invalid marker during compressed data scan: 0xFF{}", current[0]);
                }
            } else {
                huffman_data.push(previous[0]);
            }
        }

        Ok(huffman_data)
    }

    fn huffman_decode(&mut self) -> Result<()> {
        let header: &mut JPEGHeader = &mut self.header;

        let mcu_height: usize = ((header.height + 7) / 8) as usize;
        let mcu_width: usize = ((header.width + 7) / 8) as usize;

        self.mcus.resize(mcu_height * mcu_width, MCU::default());

        for i in 0..4 {
            header.ac_tables
                .get_mut(i)
                .expect("Should not panic")
                .generate_codes();

            header.dc_tables
                .get_mut(i)
                .expect("Should not panic")
                .generate_codes();
            }

        let mut bit_reader: BitReader = BitReader::new(&self.huffman_data);
        let mut previous_dcs: [i32; 3] = [0; 3];

        // Refactor these loops ?
        for i in 0..mcu_height*mcu_width {
            for j in 0..header.components_number as usize{
                if header.restart_interval != 0 && i % header.restart_interval as usize == 0 {
                    previous_dcs[0] = 0;
                    previous_dcs[1] = 0;
                    previous_dcs[2] = 0;
                    bit_reader.align();
                }

                let ac_table_id: usize = header
                    .color_components[j]
                    .huffman_ac_table_id as usize;
                let dc_table_id: usize = header
                    .color_components[j]
                    .huffman_dc_table_id as usize;
                let ac_table = &header.ac_tables[ac_table_id];
                let dc_table = &header.dc_tables[dc_table_id];

                let previous_dc: &mut i32 = previous_dcs
                    .get_mut(j)
                    .expect("Should not panic");

                let result: bool = self.mcus
                    .get_mut(i)
                    .expect("Should not panic")
                    .decode(j, &mut bit_reader, previous_dc, ac_table, dc_table);

                if !result {
                    // return Ok(());
                    bail!("Decoding MCU {} failed", j);
                }
            }
        }

        Ok(())
    }

    fn dequantize(&mut self) -> Result<()> {
        let header: &mut JPEGHeader = &mut self.header;

        let mcu_height: usize = ((header.height + 7) / 8) as usize;
        let mcu_width: usize = ((header.width + 7) / 8) as usize;

        let color_components: &[ColorComponent; 3] = &header.color_components;

        // Refactor these loops ?
        for i in 0..mcu_height*mcu_width {
            for j in 0..header.components_number as usize {
                let table_id: u8 = color_components
                    .get(j)
                    .expect("Should not panic")
                    .quantization_table_id;
                let table: &QuantizationTable = header
                    .quantization_tables
                    .get(table_id as usize)
                    .expect("Should not panic");
                self.mcus
                    .get_mut(i)
                    .expect("Should not panic")
                    .component_mut(j)
                    .expect("Should not panic")
                    .dequantize(table);
                }
        }

        Ok(())
    }

    fn dct_m() -> [f32; 6] {
        [
            2.0 * (1.0 / 16.0 * 2.0 * PI).cos(),
            2.0 * (2.0 / 16.0 * 2.0 * PI).cos(),
            2.0 * (1.0 / 16.0 * 2.0 * PI).cos() - 2.0 * (3.0 / 16.0 * 2.0 * PI).cos(),
            2.0 * (2.0 / 16.0 * 2.0 * PI).cos(),
            2.0 * (1.0 / 16.0 * 2.0 * PI).cos() + 2.0 * (3.0 / 16.0 * 2.0 * PI).cos(),
            2.0 * (3.0 / 16.0 * 2.0 * PI).cos(),
        ]
    }

    fn dct_s() -> [f32; 8] {
        [
            (0.0 / 16.0 * PI).cos() / 8f32.sqrt(),
            (1.0 / 16.0 * PI).cos() / 2.0,
            (2.0 / 16.0 * PI).cos() / 2.0,
            (3.0 / 16.0 * PI).cos() / 2.0,
            (4.0 / 16.0 * PI).cos() / 2.0,
            (5.0 / 16.0 * PI).cos() / 2.0,
            (6.0 / 16.0 * PI).cos() / 2.0,
            (7.0 / 16.0 * PI).cos() / 2.0,
        ]
    }

    fn inverse_dct(&mut self) -> Result<()> {
        let header: &mut JPEGHeader = &mut self.header;

        let mcu_height: usize = ((header.height + 7) / 8) as usize;
        let mcu_width: usize = ((header.width + 7) / 8) as usize;

        let dct_m: [f32; 6] = Self::dct_m();
        let dct_s: [f32; 8] = Self::dct_s();

        // Refactor these loops ?
        for i in 0..mcu_height*mcu_width {
            let mcu: &mut MCU = self.mcus
                .get_mut(i)
                .expect("Should not panic");

            for j in 0..header.components_number as usize {
                mcu
                    .component_mut(j)
                    .expect("Should not panic")
                    .inverse_dct(&dct_m, &dct_s);
                }
        }

        Ok(())
    }

    fn ycbcr_to_rgb(&mut self) -> Result<()> {
        let header: &mut JPEGHeader = &mut self.header;

        let mcu_height: usize = ((header.height + 7) / 8) as usize;
        let mcu_width: usize = ((header.width + 7) / 8) as usize;

        // Refactor these loops ?
        for i in 0..mcu_height*mcu_width {
            self.mcus
                .get_mut(i)
                .expect("Should not panic")
                .ycbcr_to_rgb();
        }

        Ok(())
    }
}

impl Image for JPEG {
    fn from_file(mut file: File) -> Result<Self> {
        Ok(Self {
            header: JPEGHeader::from_binary(&mut file)?,
            huffman_data: Self::read_huffman_data(&mut file)?,
            mcus: Vec::new(),
        })
    }

    fn to_bmp(&mut self) -> Result<BMP> {
        self.huffman_decode()?;
        self.dequantize()?;
        self.inverse_dct()?;
        self.ycbcr_to_rgb()?;

        Ok(BMP::new(self.header.to_bmp(), self.mcus.clone()))
    }

    fn width(&self) -> u16 {
        self.header.width
    }

    fn height(&self) -> u16 {
        self.header.height
    }

    fn mcus(&self) -> &Vec<MCU> {
        &self.mcus
    }

    fn mcus_mut(&mut self) -> &mut Vec<MCU> {
        &mut self.mcus
    }
}

impl fmt::Display for JPEG {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Header:\n\n{}\n", self.header)?;
        write!(f, "Huffman data length: {}\n", self.huffman_data.len())
    }
}
