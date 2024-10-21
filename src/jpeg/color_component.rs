use std::fmt::Display;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct ColorComponent {
    horizontal_sampling_factor: u8,
    vertical_sampling_factor: u8,
    quantization_table_id: u8,
    pub huffman_ac_table_id: u8,
    pub huffman_dc_table_id: u8,
    pub used: bool
}

impl ColorComponent {
    pub fn from_binary(&mut self, file: &mut File) -> usize {
        assert_eq!(self.used, false);

        let mut buffer: [u8; 1] = [0; 1];
        file.read_exact(&mut buffer).unwrap();
        let sampling_factor: u8 = buffer[0];

        file.read_exact(&mut buffer).unwrap();
        let quantization_table_id: u8 = buffer[0];
        assert!(quantization_table_id <= 3);

        self.horizontal_sampling_factor = (sampling_factor >> 4) & 0x0F;
        self.vertical_sampling_factor = sampling_factor & 0x0F;
        self.quantization_table_id = quantization_table_id;
        self.used = true;

        3
    }

    pub fn set_huffman_ac_table_id(&mut self, id: u8) -> () {
        self.huffman_ac_table_id = id;
    }

    pub fn set_huffman_dc_table_id(&mut self, id: u8) -> () {
        self.huffman_dc_table_id = id;
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
            used: false,
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
