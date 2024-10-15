use std::fmt::Display;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HuffmanTable {
    table_id: u8,
    is_ac_table: bool,
    offsets: [u8; 17],
    symbols: [u8; 162],
}

impl Default for HuffmanTable {
    fn default() -> Self {
        Self {
            table_id: 0,
            is_ac_table: true,
            offsets: [0; 17],
            symbols: [0; 162],
        }
    }
}

impl HuffmanTable {
    pub fn from_binary(&mut self, file: &mut File, table_id: u8, is_ac_table: bool) -> usize {
        self.symbols[0] = 0;
        self.table_id = table_id;
        self.is_ac_table = is_ac_table;

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
}

impl Display for HuffmanTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Table ID: {}\n", self.table_id)?;
        write!(f, "Symbols:\n")?;

        for j in 0..16 {
            write!(f, "{}: ", j+1)?;

            for k in self.offsets[j]..self.offsets[j+1] {
                write!(f, "0x{:02X} ", self.symbols[k as usize])?;
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}
