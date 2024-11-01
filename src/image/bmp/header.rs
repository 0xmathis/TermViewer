use anyhow::Result;
use std::fmt;
use std::fs::File;
use std::io::Read;

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
    pub fn from_binary(file: &mut File) -> Result<Self> {
        let mut header: BMPHeader = BMPHeader::default();
        let mut buffer2: [u8; 2] = [0; 2];
        let mut buffer4: [u8; 4] = [0; 4];
        let mut count: u32 = 0;

        file.read_exact(&mut buffer2)?;
        assert_eq!([b'B', b'M'], buffer2);
        count += 2;

        file.read_exact(&mut buffer4)?;
        header.bmp_size = ((buffer4[3] as u32) << 24) + ((buffer4[2] as u32) << 16) + ((buffer4[1] as u32) << 8) + buffer4[0] as u32;
        count += 4;

        file.read_exact(&mut buffer4)?;
        assert_eq!([0; 4], buffer4);
        count += 4;

        file.read_exact(&mut buffer4)?;
        header.starting_offset = ((buffer4[3] as u32) << 24) + ((buffer4[2] as u32) << 16) + ((buffer4[1] as u32) << 8) + buffer4[0] as u32;
        count += 4;

        file.read_exact(&mut buffer4)?;
        header.header_size = ((buffer4[3] as u32) << 24) + ((buffer4[2] as u32) << 16) + ((buffer4[1] as u32) << 8) + buffer4[0] as u32;
        count += 4;

        file.read_exact(&mut buffer2)?;
        header.width = ((buffer2[1] as u16) << 8) + buffer2[0] as u16;
        count += 2;

        file.read_exact(&mut buffer2)?;
        header.height = ((buffer2[1] as u16) << 8) + buffer2[0] as u16;
        count += 2;

        file.read_exact(&mut buffer2)?;
        header.components_number = ((buffer2[1] as u16) << 8) + buffer2[0] as u16;
        assert_eq!(1u16, header.components_number);
        count += 2;

        file.read_exact(&mut buffer2)?;
        header.bits_per_pixel = ((buffer2[1] as u16) << 8) + buffer2[0] as u16;
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
