use anyhow::Result;
use std::fmt;

use crate::image::bit_reader::BitReader;

use super::bmp_bit_reader::BmpBitReader;

#[derive(Clone, Debug, Default)]
pub struct BMPHeader {
    pub bmp_size: u32,
    pub header_size: u32,
    pub height: u16,
    pub width: u16,
    pub components_number: u16,
    pub starting_offset: u32,
    pub bits_per_pixel: u16,
}

impl BMPHeader {
    pub fn from_binary(reader: &mut BmpBitReader) -> Result<Self> {
        let mut header: BMPHeader = BMPHeader::default();
        let mut count: u32 = 0;

        assert_eq!(0x424D, reader.read_word()?);
        count += 2;

        header.bmp_size = reader.read_double()?.swap_bytes();
        count += 4;

        assert_eq!(0u32, reader.read_double()?);
        count += 4;

        header.starting_offset = reader.read_double()?.swap_bytes();
        count += 4;

        header.header_size = reader.read_double()?.swap_bytes();
        count += 4;

        header.width = reader.read_word()?.swap_bytes();
        count += 2;

        header.height = reader.read_word()?.swap_bytes();
        count += 2;

        header.components_number = reader.read_word()?.swap_bytes();
        assert_eq!(1u16, header.components_number);
        count += 2;

        header.bits_per_pixel = reader.read_word()?.swap_bytes();
        count += 2;

        assert_eq!(header.starting_offset, count);

        Ok(header)
    }
}

impl fmt::Display for BMPHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bmp_size: {:04X}\n", self.bmp_size)?;
        write!(f, "starting_offset: {:04X}\n", self.starting_offset)?;
        write!(f, "header_size: {:04X}\n", self.header_size)?;
        write!(f, "width: {:02X}\n", self.width)?;
        write!(f, "height: {:02X}\n", self.height)?;
        write!(f, "components_number: {:02X}\n", self.components_number)?;
        write!(f, "bits_per_pixel: {:02X}\n", self.bits_per_pixel)?;
        Ok(())
    }
}
