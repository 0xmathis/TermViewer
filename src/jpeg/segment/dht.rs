use std::fs::File;
use std::io::Read;

use super::SegmentType;

#[derive(Debug)]
pub struct DHT {
    segment_type: SegmentType,
    length: u16,
    ht_infos: u8,
    symbols_counts: [u8; 16],
    symbols: Vec<Vec<u8>>,
}

impl DHT {
    pub fn from_binary(file: &mut File) -> Self {
        let mut length_buffer: [u8; 2] = [0; 2];
        file.read_exact(&mut length_buffer).unwrap();
        let length: u16 = ((length_buffer[0] as u16) << 8) + length_buffer[1] as u16;
        let mut count: i32 = length as i32;
        count -= 2;

        let mut ht_infos_buffer: [u8; 1] = [0; 1];
        file.read_exact(&mut ht_infos_buffer).unwrap();
        let ht_infos: u8 = ht_infos_buffer[0];
        count -= 1;

        let mut symbols_counts: [u8; 16] = [0; 16];
        file.read_exact(&mut symbols_counts).unwrap();
        let symbol_count: u8 = symbols_counts.iter().sum();
        count -= 16;

        let mut symbols: Vec<Vec<u8>> = Vec::new();

        for i in symbols_counts {
            let mut symbol: Vec<u8> = Vec::new();
            symbol.resize(i as usize, 0);
            file.read_exact(&mut symbol).unwrap();
            count -= i as i32;

            symbols.push(symbol);
        }

        assert_eq!(count, 0);

        Self {
            segment_type: SegmentType::DHT,
            length,
            ht_infos,
            symbols_counts,
            symbols,
        }
    }
}
