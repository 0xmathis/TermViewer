use std::fs::File;
use std::io::Read;

use super::SegmentType;

#[derive(Debug)]
pub struct COM {
    segment_type: SegmentType,
    length: u16,
    data: Vec<u8>,
}

impl COM {
    pub fn from_binary(file: &mut File) -> Self {
        let mut length_buffer: [u8; 2] = [0; 2];
        file.read_exact(&mut length_buffer).unwrap();
        let length: u16 = ((length_buffer[0] as u16) << 8) + length_buffer[1] as u16;
        let mut count: i32 = length as i32;
        count -= 2;

        let mut data: Vec<u8> = Vec::new();
        data.resize(count as usize, 0);
        file.read_exact(&mut data).unwrap();
        count -= count;

        assert_eq!(count, 0);

        Self {
            segment_type: SegmentType::COM,
            length,
            data,
        }
    }
}
