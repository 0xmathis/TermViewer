use std::fmt::Display;
use std::fs::File;
use std::io::Read;

use super::bit_reader::BitReader;
use super::header::Header;
use super::mcu::MCU;
use super::JPEG;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HuffmanTable {
    table_id: u8,
    pub offsets: [u8; 17],
    pub symbols: [u8; 162],
    pub codes: [u32; 162],
    is_ac_table: bool,
    is_set: bool,
}

impl Default for HuffmanTable {
    fn default() -> Self {
        Self {
            table_id: 0,
            offsets: [0; 17],
            symbols: [0; 162],
            codes: [0; 162],
            is_ac_table: true,
            is_set: false,
        }
    }
}

impl HuffmanTable {
    pub fn from_binary(&mut self, file: &mut File, table_id: u8, is_ac_table: bool) -> usize {
        self.symbols[0] = 0;
        self.table_id = table_id;
        self.is_ac_table = is_ac_table;
        self.is_set = true;

        let mut symbols_count: usize = 0;
        let mut buffer: [u8; 1] = [0; 1];

        for i in 1..17 {
            file.read_exact(&mut buffer).unwrap();
            symbols_count += buffer[0] as usize;
            assert!(symbols_count <= 162);
            self.offsets[i] = symbols_count as u8;
        }

        for i in 0..symbols_count {
            file.read_exact(&mut buffer).unwrap();
            self.symbols[i as usize] = buffer[0];
        }

        17 + symbols_count
    }

    pub fn generate_codes(&mut self) -> () {
        if !self.is_set {
            return;
        }

        let mut code: u32 = 0;

        for i in 0..16usize {
            for j in self.offsets[i]..self.offsets[i+1] {
                self.codes[j as usize] = code;
                code += 1;
            }

            code <<= 1;
        }
    }
}

impl Display for HuffmanTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Table ID: {}\n", self.table_id)?;
        write!(f, "Symbols:\n")?;

        for j in 0..16 {
            write!(f, "{}: ", j+1)?;

            for k in self.offsets[j]..self.offsets[j+1] {
                write!(f, "{:X} ", self.symbols[k as usize])?;
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}

pub fn huffman_decoder(jpeg: &mut JPEG) -> Option<Vec<MCU>> {
    let header: &mut Header = &mut jpeg.header;

    let mcu_height: usize = ((header.height + 7) / 8) as usize;
    let mcu_width: usize = ((header.width + 7) / 8) as usize;

    let mut mcus: Vec<MCU> = Vec::new();
    mcus.resize(mcu_height * mcu_width, MCU::default());

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

    let mut bit_reader: BitReader = BitReader::new(&jpeg.huffman_data);
    let mut previous_dcs: [i32; 3] = [0; 3];

    for i in 0..mcu_height*mcu_width {
        if header.restart_interval != 0 && i % header.restart_interval as usize == 0 {
            previous_dcs[0] = 0;
            previous_dcs[1] = 0;
            previous_dcs[2] = 0;
            bit_reader.align();
        }

        for j in 0..header.components_number as usize{
            let ac_table_id: usize = header
                .color_components[j]
                .huffman_ac_table_id as usize;
            let dc_table_id: usize = header
                .color_components[j]
                .huffman_dc_table_id as usize;
            let previous_dc: &mut i32 = previous_dcs
                .get_mut(j)
                .expect("Should not panic");
            let ac_table = &header.ac_tables[ac_table_id];
            let dc_table = &header.dc_tables[dc_table_id];

            let result: bool = mcus
                .get_mut(i)
                .expect("Should not panic")
                .decode(j, &mut bit_reader, previous_dc, ac_table, dc_table);

            if !result {
                return None;
            }
        }
    }

    Some(mcus)
}
