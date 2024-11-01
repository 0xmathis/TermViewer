use anyhow::{bail, Result};
use std::fmt;
use std::fs::File;
use std::io::Read;

use crate::image::bmp::header::BMPHeader;
use crate::image::huffman::HuffmanTable;
use crate::image::quantization_table::QuantizationTable;
use super::color_component::ColorComponent;
use super::segment::SegmentType;

#[derive(Clone, Debug, Default)]
pub struct JPEGHeader {
    pub quantization_tables: [QuantizationTable; 4],
    pub ac_tables: [HuffmanTable; 4],
    pub dc_tables: [HuffmanTable; 4],
    pub color_components: [ColorComponent; 3],

    pub height: u16,
    pub width: u16,
    pub components_number: u8,
    pub zero_based: bool,

    pub start_of_selection: u8,
    pub end_of_selection: u8,
    pub successive_approximation_high: u8,
    pub successive_approximation_low: u8,

    pub restart_interval: u16,
}

impl JPEGHeader {
    pub fn to_bmp(&self) -> BMPHeader {
        let padding_size: u32 = (self.width % 4) as u32;
        let bmp_size: u32 = 14u32 + 12u32 + self.width as u32 * self.height as u32 * 3 + padding_size * self.height as u32;

        BMPHeader {
            bmp_size,
            header_size: 12u32,
            height: self.height,
            width: self.width,
            components_number: 1u16,
            starting_offset: 0x1Au32,
            bits_per_pixel: 24u16,
        }
    }

    pub fn from_binary(file: &mut File) -> Result<Self> {
        let mut header: JPEGHeader = JPEGHeader::default();
        let mut marker: [u8; 2] = [0; 2];

        file.read_exact(&mut marker).unwrap();
        if SegmentType::from_marker(marker) != Some(SegmentType::SOI) {
            bail!("JPEG file start with SOI marker");
        };

        loop {
            file.read_exact(&mut marker).unwrap();

            let Some(marker) = SegmentType::from_marker(marker) else {
                bail!("marker {marker:02X?}: unknown");
            };

            println!("segment {marker:?}");

            match marker {
                SegmentType::APPN => header.read_segment_appn(file),
                SegmentType::COM |
                SegmentType::DHP |
                SegmentType::DNL |
                SegmentType::EXP |
                SegmentType::JPGN => header.read_comment(file),
                SegmentType::DHT => header.read_segment_dht(file),
                SegmentType::DQT => header.read_segment_dqt(file),
                SegmentType::DRI => header.read_segment_dri(file),
                SegmentType::SOF0 => header.read_segment_sof0(file),
                SegmentType::SOS => {
                    header.read_segment_sos(file);
                    break;
                },
                SegmentType::TEM => {},
                SegmentType::DAC => bail!("Arithmetic Coding mode not supported"),
                SegmentType::EOI => bail!("Should not encounter EOI marker before SOS marker"),
                SegmentType::RSTN => bail!("Should not encounter RSTN marker before SOS marker"),
                SegmentType::SOI => bail!("Embedded JPEG not supported"),
            }
        }

        Ok(header)
    }

    fn read_segment_appn(&mut self, file: &mut File) -> () {
        let mut buffer2: [u8; 2] = [0; 2];

        file.read_exact(&mut buffer2).unwrap();
        let length: u16 = ((buffer2[0] as u16) << 8) + buffer2[1] as u16;
        assert!(length >= 2);

        let mut count: i32 = length as i32;
        count -= 2;

        let mut data: Vec<u8> = Vec::new();
        data.resize(count as usize, 0);
        file.read_exact(&mut data).unwrap();
        count -= count;

        assert_eq!(0, count);
    }

    fn read_comment(&mut self, file: &mut File) -> () {
        let mut buffer2: [u8; 2] = [0; 2];
        file.read_exact(&mut buffer2).unwrap();
        let length: u16 = ((buffer2[0] as u16) << 8) + buffer2[1] as u16;
        assert!(length >= 2);

        let mut count: i32 = length as i32;
        count -= 2;

        let mut data: Vec<u8> = Vec::new();
        data.resize(count as usize, 0);
        file.read_exact(&mut data).unwrap();
        count -= count;

        assert_eq!(0, count);
    }

    fn read_segment_dht(&mut self, file: &mut File) -> () {
        let mut buffer1: [u8; 1] = [0; 1];
        let mut buffer2: [u8; 2] = [0; 2];

        file.read_exact(&mut buffer2).unwrap();
        let length: u16 = ((buffer2[0] as u16) << 8) + buffer2[1] as u16;
        let mut count: i32 = length as i32;
        count -= 2;

        while count > 0 {
            file.read_exact(&mut buffer1).unwrap();
            let table_infos: u8 = buffer1[0];

            let table_id: u8 = table_infos & 0x0F;
            let is_ac_table: bool = (table_infos >> 4) & 0x0F == 1;
            assert!(table_id <= 3);

            let len: usize;

            if is_ac_table {
                len = self.ac_tables
                    .get_mut(table_id as usize)
                    .expect("Should not panic")
                    .from_binary(file, table_id, is_ac_table);
            } else {
                len = self.dc_tables
                    .get_mut(table_id as usize)
                    .expect("Should not panic")
                    .from_binary(file, table_id, is_ac_table);
            }

            count -= len as i32;
        }

        assert_eq!(0, count);
    }

    fn read_segment_dqt(&mut self, file: &mut File) -> () {
        let mut buffer1: [u8; 1] = [0; 1];
        let mut buffer2: [u8; 2] = [0; 2];

        file.read_exact(&mut buffer2).unwrap();
        let length: u16 = ((buffer2[0] as u16) << 8) + buffer2[1] as u16;
        let mut count: i32 = length as i32;
        count -= 2;
        
        while count > 0 {
            file.read_exact(&mut buffer1).unwrap();
            let table_infos: u8 = buffer1[0];
            count -= 1;

            let table_id: u8 = table_infos & 0x0F;
            let element_size: u8 = (table_infos >> 4) & 0x0F;
            assert!(table_id <= 3);
            assert!(element_size <= 1);

            let len: usize = self.quantization_tables
                .get_mut(table_id as usize)
                .expect("Should not panic because vec initialized")
                .from_binary(file, table_id, element_size);
            count -= len as i32;
        }

        assert_eq!(0, count);
    }

    fn read_segment_dri(&mut self, file: &mut File) -> () {
        let mut buffer2: [u8; 2] = [0; 2];

        file.read_exact(&mut buffer2).unwrap();
        let length: u16 = ((buffer2[0] as u16) << 8) + buffer2[1] as u16;
        assert_eq!(4, length);
        let mut count: i32 = length as i32;
        count -= 2;

        file.read_exact(&mut buffer2).unwrap();
        self.restart_interval = ((buffer2[0] as u16) << 8) + buffer2[1] as u16;
        count -= 2;

        assert_eq!(0, count);
    }

    fn read_segment_sof0(&mut self, file: &mut File) -> () {
        let mut buffer1: [u8; 1] = [0; 1];
        let mut buffer2: [u8; 2] = [0; 2];

        file.read_exact(&mut buffer2).unwrap();
        let length: u16 = ((buffer2[0] as u16) << 8) + buffer2[1] as u16;
        let mut count: i32 = length as i32;
        count -= 2;

        file.read_exact(&mut buffer1).unwrap();
        let precision: u8 = buffer1[0];
        assert_eq!(8, precision);
        count -= 1;

        file.read_exact(&mut buffer2).unwrap();
        let height: u16 = ((buffer2[0] as u16) << 8) + buffer2[1] as u16;
        assert_ne!(0, height);
        self.height = height;
        count -= 2;

        file.read_exact(&mut buffer2).unwrap();
        let width: u16 = ((buffer2[0] as u16) << 8) + buffer2[1] as u16;
        assert_ne!(0, width);
        self.width = width;
        count -= 2;

        file.read_exact(&mut buffer1).unwrap();
        let component_numbers: u8 = buffer1[0];
        assert!(component_numbers == 1 || component_numbers == 3);
        count -= 1;

        let mut zero_based: bool = false;

        for i in 0..component_numbers {
            file.read_exact(&mut buffer1).unwrap();
            let mut component_id: u8 = buffer1[0];
            assert!(component_id <= 3);

            if component_id == 0 && i == 0 {
                zero_based = true;
            }

            if zero_based {
                component_id += 1;
            }

            assert_ne!(0, component_id);
            assert!(component_id <= component_numbers);

            let len: usize = self.color_components
                .get_mut((component_id - 1) as usize)
                .expect("Should not panic because vec initialized")
                .from_binary(file);
            count -= len as i32;
        }

        self.components_number = component_numbers;
        self.zero_based = zero_based;

        assert_eq!(0, count);
    }

    fn read_segment_sos(&mut self, file: &mut File) -> () {
        assert_ne!(0, self.components_number);

        let mut buffer1: [u8; 1] = [0; 1];
        let mut buffer2: [u8; 2] = [0; 2];

        file.read_exact(&mut buffer2).unwrap();
        let length: u16 = ((buffer2[0] as u16) << 8) + buffer2[1] as u16;
        let mut count: i32 = length as i32;
        count -= 2;

        for component in self.color_components.iter_mut() {
            component.used_scan = false;
        }

        file.read_exact(&mut buffer1).unwrap();
        let components_number: u8 = buffer1[0];
        count -= 1;

        for _ in 0..components_number {
            file.read_exact(&mut buffer1).unwrap();
            let mut component_id: u8 = buffer1[0];
            count -= 1;

            if self.zero_based {
                component_id += 1;
            }

            assert!(component_id <= self.components_number);

            let color_component: &mut ColorComponent = self.color_components
                .get_mut(component_id as usize - 1usize)
                .expect("Should not panic");

            assert_eq!(true, color_component.used_frame);
            assert_eq!(false, color_component.used_scan);
            color_component.used_scan = true;

            file.read_exact(&mut buffer1).unwrap();
            let huffman_table_ids: u8 = buffer1[0];
            count -= 1;

            let huffman_ac_table_id: u8 = huffman_table_ids & 0x0F;
            let huffman_dc_table_id: u8 = (huffman_table_ids >> 4) & 0x0F;

            assert!(huffman_ac_table_id <= 3);
            assert!(huffman_dc_table_id <= 3);

            color_component.set_huffman_ac_table_id(huffman_ac_table_id);
            color_component.set_huffman_dc_table_id(huffman_dc_table_id);
        }

        file.read_exact(&mut buffer1).unwrap();
        let start_of_selection: u8 = buffer1[0];
        assert_eq!(0, start_of_selection);
        self.start_of_selection = start_of_selection;
        count -= 1;

        file.read_exact(&mut buffer1).unwrap();
        let end_of_selection: u8 = buffer1[0];
        assert_eq!(63, end_of_selection);
        self.end_of_selection = end_of_selection;
        count -= 1;

        file.read_exact(&mut buffer1).unwrap();
        let successive_approximation_low: u8 = buffer1[0] & 0x0F;
        let successive_approximation_high: u8 = (buffer1[0] >> 4) & 0x0F;
        assert_eq!(0, successive_approximation_low);
        assert_eq!(0, successive_approximation_high);
        self.successive_approximation_low = successive_approximation_low;
        self.successive_approximation_high = successive_approximation_high;
        count -= 1;

        assert_eq!(0, count);
    }
}

impl fmt::Display for JPEGHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "================================================\n")?;
        write!(f, "SOF=============\n")?;
        write!(f, "Height: {}\n", self.height)?;
        write!(f, "Width: {}\n", self.width)?;

        write!(f, "Color Components:\n")?;

        for i in 0..self.components_number as usize {
            let component: &ColorComponent = &self.color_components[i];

            write!(f, "Component ID: {}\n", i+1)?;
            write!(f, "{component}")?;
        }

        write!(f, "DQT=============\n")?;
        for i in 0..4 {
            let quantization_table: &QuantizationTable = &self.quantization_tables[i];

            if QuantizationTable::default().eq(quantization_table) {
                continue;
            }

            write!(f, "{quantization_table}")?;
        }

        write!(f, "SOS=============\n")?;
        write!(f, "Start of selection: {}\n", self.start_of_selection)?;
        write!(f, "End of selection: {}\n", self.end_of_selection)?;
        write!(f, "Successive Approximation High: {}\n", self.successive_approximation_high)?;
        write!(f, "Successive Approximation Low: {}\n", self.successive_approximation_low)?;

        write!(f, "Color Components:\n")?;

        for i in 0..self.components_number as usize {
            let color_component: &ColorComponent = &self.color_components[i];

            write!(f, "Component ID: {}\n", i + 1)?;
            write!(f, "Huffman DC Table ID: {}\n", color_component.huffman_dc_table_id)?;
            write!(f, "Huffman AC Table ID: {}\n", color_component.huffman_ac_table_id)?;
        }

        write!(f, "DHT=============\n")?;

        write!(f, "DC Tables:\n")?;
        for i in 0..4 {
            let table: &HuffmanTable = &self.dc_tables[i];

            if HuffmanTable::default().eq(table) {
                continue;
            }

            write!(f, "{table}")?;
        }

        write!(f, "AC Tables:\n")?;
        for i in 0..4 {
            let table: &HuffmanTable = &self.ac_tables[i];

            if HuffmanTable::default().eq(table) {
                continue;
            }

            write!(f, "{table}")?;
        }

        write!(f, "DRI=============\n")?;
        write!(f, "Restart interval: {}\n", self.restart_interval)?;

        Ok(())
    }
}
