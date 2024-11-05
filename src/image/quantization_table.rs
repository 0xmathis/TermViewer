use std::fmt::Display;
use anyhow::Result;

use super::bit_reader::BitReader;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct QuantizationTable {
    table: [u16; 64],
    table_id: u8,
}

impl QuantizationTable {
    pub fn table(&self, index: usize) -> u16 {
        self.table[index]
    }

    pub fn from_binary(&mut self, reader: &mut impl BitReader, table_id: u8, element_size: u8) -> Result<usize> {
        self.table_id = table_id;

        if element_size == 0 {
            for i in Self::zigzag_map().into_iter() {
                self.table[i] = reader.read_byte()? as u16;
            }
        } else {
            for i in Self::zigzag_map().into_iter() {
                self.table[i] = reader.read_word()?;
            }
        }

        Ok(self.table.len())
    }

    fn zigzag_map() -> [usize; 64] {
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
            table: [0u16; 64],
            table_id: 0,
        }
    }
}

impl Display for QuantizationTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Table ID: {}\n", self.table_id)?;
        write!(f, "Table data:")?;

        for j in 0..64 {
            if j % 8 == 0 {
                write!(f, "\n")?;
            }

            write!(f, "{} ", self.table[j])?;
        }

        write!(f, "\n")
    }
}
