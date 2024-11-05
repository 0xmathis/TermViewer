use std::fmt::Display;
use anyhow::Result;

use crate::image::bit_reader::BitReader;

#[derive(Debug, Clone)]
pub struct ColorComponent {
    horizontal_sampling_factor: u8,
    vertical_sampling_factor: u8,
    quantization_table_id: u8,
    huffman_ac_table_id: u8,
    huffman_dc_table_id: u8,
    used_scan: bool,
    used_frame: bool,
}

impl ColorComponent {
    pub fn from_binary(&mut self, reader: &mut impl BitReader) -> Result<()> {
        assert_eq!(false, self.used_frame);

        let sampling_factor: u8 = reader.read_byte()?;
        let quantization_table_id: u8 = reader.read_byte()?;
        assert!(quantization_table_id <= 3);

        self.horizontal_sampling_factor = (sampling_factor >> 4) & 0x0F;
        self.vertical_sampling_factor = sampling_factor & 0x0F;
        self.quantization_table_id = quantization_table_id;
        self.used_frame = true;

        Ok(())
    }

    pub fn set_huffman_ac_table_id(&mut self, id: u8) -> () {
        self.huffman_ac_table_id = id;
    }

    pub fn set_huffman_dc_table_id(&mut self, id: u8) -> () {
        self.huffman_dc_table_id = id;
    }

    pub fn quantization_table_id(&self) -> u8 {
        self.quantization_table_id
    }

    pub fn huffman_ac_table_id(&self) -> u8 {
        self.huffman_ac_table_id
    }

    pub fn huffman_dc_table_id(&self) -> u8 {
        self.huffman_dc_table_id
    }

    pub fn set_used_scan(&mut self, used_scan: bool) -> () {
        self.used_scan = used_scan;
    }

    pub fn used_scan(&self) -> bool {
        self.used_scan
    }

    pub fn used_frame(&self) -> bool {
        self.used_frame
    }
}

impl Default for ColorComponent {
    fn default() -> Self {
        Self {
            horizontal_sampling_factor: 1,
            vertical_sampling_factor: 1,
            quantization_table_id: 0,
            huffman_ac_table_id: 0,
            huffman_dc_table_id: 0,
            used_frame: false,
            used_scan: false,
        }
    }
}

impl Display for ColorComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Horizontal Sampling Factor: {}\n", self.horizontal_sampling_factor)?;
        write!(f, "Vertical Sampling Factor: {}\n", self.vertical_sampling_factor)?;
        write!(f, "Quantization Table ID: {}\n", self.quantization_table_id)?;

        Ok(())
    }
}
