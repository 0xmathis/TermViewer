use std::fmt;
use std::fs::File;
use std::io::Read;

use super::SegmentType;

#[derive(Debug, Clone)]
struct QuantizationTable {
    table: Vec<u8>,
    table_id: u8,
    element_size: u8,
}

#[derive(Debug)]
pub struct DQT {
    segment_type: SegmentType,
    length: u16,
    quantization_tables: Vec<QuantizationTable>,
}

impl QuantizationTable {
    pub fn from_binary(&mut self, file: &mut File, table_id: u8, element_size: u8) -> usize {
            self.table_id = table_id;

            if element_size == 0 {
                self.element_size = 8;
                self.table.resize(64usize, 0);

                let mut buffer: [u8; 1] = [0; 1];
                for i in QuantizationTable::get_zigzag_map().into_iter() {
                    let i = i as usize;
                    file.read_exact(&mut buffer).unwrap();
                    self.table[i] = buffer[0];
                }
            } else {
                self.element_size = 16;
                self.table.resize(128usize, 0);

                let mut buffer: [u8; 2] = [0; 2];
                for i in QuantizationTable::get_zigzag_map().into_iter() {
                    let i = i as usize;
                    file.read_exact(&mut buffer).unwrap();
                    self.table[2*i] = buffer[0];
                    self.table[2*i+1] = buffer[1];
                }
            }

            self.table.len()
    }

    fn get_zigzag_map() -> [u8; 64] {
        [
            0,   1,  8, 16,  9,  2,  3, 10,
            17, 24, 32, 25, 18, 11,  4,  5,
            12, 19, 26, 33, 40, 48, 41, 34,
            27, 20, 13,  6,  7, 14, 21, 28,
            35, 42, 49, 56, 57, 50, 43, 36,
            29, 22, 15, 23, 30, 37, 44, 51,
            58, 59, 52, 45, 38, 31, 39, 46,
            53, 60, 61, 54, 47, 55, 62, 63
        ]
    }
}

impl Default for QuantizationTable {
    fn default() -> Self {
        Self {
            table: Vec::new(),
            table_id: 0,
            element_size: 0,
        }
    }
}

impl DQT {
    pub fn from_binary(file: &mut File) -> Self {
        let mut length_buffer: [u8; 2] = [0; 2];
        file.read_exact(&mut length_buffer).unwrap();
        // We need to let length be negative to check if the JPEG is valid
        let length: u16 = ((length_buffer[0] as u16) << 8) + length_buffer[1] as u16;
        let mut count: i32 = length as i32;
        count -= 2;
        
        let mut quantization_tables: Vec<QuantizationTable> = Vec::new();
        quantization_tables.resize(4, QuantizationTable::default());

        while count > 0 {
            let mut table_info_buffer: [u8; 1] = [0; 1];
            file.read_exact(&mut table_info_buffer).unwrap();
            let table_info: u8 = table_info_buffer[0];
            count -= 1;

            let table_id: u8 = table_info & 0x0F;
            let element_size: u8 = (table_info >> 4) & 0x0F;
            assert!(table_id <= 3);
            assert!(element_size <= 1);

            let len: usize = quantization_tables
                .get_mut(table_id as usize)
                .expect("Should not panic because vec initialized")
                .from_binary(file, table_id, element_size);
            count -= len as i32;
        }

        assert_eq!(count, 0);

        Self {
            segment_type: SegmentType::DQT,
            length,
            quantization_tables,
        }
    }
}

impl fmt::Display for DQT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "segment_type: {:?} | length: {}\n", self.segment_type, self.length)?;

        for i in 0..4 {
            let quantization_table: &QuantizationTable = &self.quantization_tables[i];
            let table: &Vec<u8> = &quantization_table.table;
            let len: usize = table.len();

            if len == 0 {
                continue;
            }

            write!(f, "Table ID: {i}\n")?;
            write!(f, "Table data:")?;

            for j in 0..len {
                if j % 8 == 0 {
                    write!(f, "\n")?;
                }

                write!(f, "0x{:02X} ", table[j])?;
            }
        }

        Ok(())
    }
}
