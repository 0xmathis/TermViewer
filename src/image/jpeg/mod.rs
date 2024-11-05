use anyhow::Result;
use std::f32::consts::PI;
use std::fmt;
use std::fs::File;
use std::io::BufReader;

use header::JPEGHeader;
use jpeg_bit_reader::JpegBitReader;
use super::huffman::HuffmanTable;
use super::Image;
use super::bit_reader::BitReader;
use super::bmp::BMP;
use super::mcu::MCU;
use super::quantization_table::QuantizationTable;

mod color_component;
mod header;
mod jpeg_bit_reader;
mod segment;

#[derive(Debug)]
pub struct JPEG {
    header: JPEGHeader,
    mcus: Vec<MCU>,
    reader: JpegBitReader,
}

impl JPEG {
    fn huffman_decode(&mut self) -> Result<()> {
        let header: &mut JPEGHeader = &mut self.header;

        let mcu_height: usize = header.mcu_height();
        let mcu_width: usize = header.mcu_width();

        self.mcus.resize(mcu_height * mcu_width, MCU::default());
        header.generate_tables_codes();
        let mut previous_dcs: [i32; 3] = [0; 3];
        let restart_interval: usize = header.restart_interval() as usize;

        for i in 0..mcu_height*mcu_width {
            if restart_interval != 0 && i % restart_interval == 0 {
                previous_dcs[0] = 0;
                previous_dcs[1] = 0;
                previous_dcs[2] = 0;
                self.reader.align();
            }

            let mcu: &mut MCU = self.mcus
                .get_mut(i)
                .expect("Should not panic");

            for j in 0..header.components_number() as usize{
                let ac_table_id: usize = header
                    .color_component(j)
                    .expect("Should exist")
                    .huffman_ac_table_id() as usize;
                let ac_table: &HuffmanTable = header
                    .ac_table(ac_table_id)
                    .expect("Should exist");
                let dc_table_id: usize = header
                    .color_component(j)
                    .expect("Should exist")
                    .huffman_dc_table_id() as usize;
                let dc_table: &HuffmanTable = header
                    .dc_table(dc_table_id)
                    .expect("Should exist");

                let previous_dc: &mut i32 = previous_dcs
                    .get_mut(j)
                    .expect("Should not panic");

                mcu.decode(j, &mut self.reader, previous_dc, ac_table, dc_table)?;
            }
        }

        Ok(())
    }

    fn dequantize(&mut self) -> Result<()> {
        let header: &mut JPEGHeader = &mut self.header;

        let mcu_height: usize = header.mcu_height();
        let mcu_width: usize = header.mcu_width();

        for i in 0..mcu_height*mcu_width {
            let mcu: &mut MCU = self.mcus
                .get_mut(i)
                .expect("Should not panic");

            for j in 0..header.components_number() as usize {
                let table_id: u8 = header
                    .color_component(j)
                    .expect("Should not panic")
                    .quantization_table_id();
                let table: &QuantizationTable = header
                    .quantization_tables(table_id as usize)
                    .expect("Should not panic");
                mcu
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

        let mcu_height: usize = header.mcu_height();
        let mcu_width: usize = header.mcu_width();

        let dct_m: [f32; 6] = Self::dct_m();
        let dct_s: [f32; 8] = Self::dct_s();

        for i in 0..mcu_height*mcu_width {
            let mcu: &mut MCU = self.mcus
                .get_mut(i)
                .expect("Should not panic");

            for j in 0..header.components_number() as usize {
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

        let mcu_height: usize = header.mcu_height();
        let mcu_width: usize = header.mcu_width();

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
    fn from_stream(stream: BufReader<File>, debug: bool) -> Result<Self> {
        let mut reader: JpegBitReader = JpegBitReader::new(stream);
        let mut jpeg: Self = Self {
            header: JPEGHeader::from_binary(&mut reader, debug)?,
            mcus: Vec::new(),
            reader,
        };

        jpeg.huffman_decode()?;
        jpeg.dequantize()?;
        jpeg.inverse_dct()?;
        jpeg.ycbcr_to_rgb()?;

        Ok(jpeg)
    }

    fn to_bmp(self: Box<Self>) -> Box<BMP> {
        Box::new(BMP::new(self.header.to_bmp(), self.mcus))
    }
}

impl fmt::Display for JPEG {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Header:\n\n{}\n", self.header)
    }
}
