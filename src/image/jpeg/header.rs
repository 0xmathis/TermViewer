use anyhow::{bail, Result};
use std::fmt;

use crate::image::bit_reader::BitReader;
use crate::image::bmp::header::BMPHeader;
use crate::image::huffman::HuffmanTable;
use crate::image::quantization_table::QuantizationTable;
use super::color_component::ColorComponent;
use super::jpeg_bit_reader::JpegBitReader;
use super::segment::SegmentType;

#[derive(Clone, Debug, Default)]
pub struct JPEGHeader {
    quantization_tables: [QuantizationTable; 4],
    ac_tables: [HuffmanTable; 4],
    dc_tables: [HuffmanTable; 4],
    color_components: [ColorComponent; 3],

    height: u16,
    width: u16,
    components_number: u8,
    zero_based: bool,

    start_of_selection: u8,
    end_of_selection: u8,
    successive_approximation_high: u8,
    successive_approximation_low: u8,

    restart_interval: u16,
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

    pub fn from_binary(reader: &mut JpegBitReader, debug: bool) -> Result<Self> {
        let mut header: JPEGHeader = JPEGHeader::default();
        let mut marker: u16 = reader.read_word()?;

        if SegmentType::from_marker(marker) != Some(SegmentType::SOI) {
            bail!("JPEG file start with SOI marker");
        };

        loop {
            marker = reader.read_word()?;

            let Some(marker) = SegmentType::from_marker(marker) else {
                bail!("marker {marker:02X?}: unknown");
            };

            if debug {
                println!("segment {marker:?}");
            }

            match marker {
                SegmentType::APPN => header.read_segment_appn(reader)?,
                SegmentType::COM |
                SegmentType::DHP |
                SegmentType::DNL |
                SegmentType::EXP |
                SegmentType::JPGN => header.read_comment(reader)?,
                SegmentType::DHT  => header.read_segment_dht(reader)?,
                SegmentType::DQT  => header.read_segment_dqt(reader)?,
                SegmentType::DRI  => header.read_segment_dri(reader)?,
                SegmentType::SOF0 => header.read_segment_sof0(reader)?,
                SegmentType::SOS  => {
                    header.read_segment_sos(reader)?;
                    break;
                },
                SegmentType::TEM  => (),
                SegmentType::DAC  => bail!("Arithmetic Coding mode not supported"),
                SegmentType::EOI  => bail!("Should not encounter EOI marker before SOS marker"),
                SegmentType::RSTN => bail!("Should not encounter RSTN marker before SOS marker"),
                SegmentType::SOI  => bail!("Embedded JPEG not supported"),
            }
        }

        Ok(header)
    }

    fn read_segment_appn(&mut self, reader: &mut JpegBitReader) -> Result<()> {
        let length: u16 = reader.read_word()?;
        assert!(length >= 2);

        let mut count: i32 = length as i32;
        count -= 2;

        for _ in 0..count {
            reader.read_byte()?;
        }
        count -= count;

        assert_eq!(0, count);

        Ok(())
    }

    fn read_comment(&mut self, reader: &mut JpegBitReader) -> Result<()> {
        let length: u16 = reader.read_word()?;
        assert!(length >= 2);

        let mut count: i32 = length as i32;
        count -= 2;

        for _ in 0..count {
            reader.read_byte()?;
        }
        count -= count;

        assert_eq!(0, count);

        Ok(())
    }

    fn read_segment_dht(&mut self, reader: &mut JpegBitReader) -> Result<()> {
        let length: u16 = reader.read_word()?;
        let mut count: i32 = length as i32;
        count -= 2;

        while count > 0 {
            let table_infos: u8 = reader.read_byte()?;

            let table_id: u8 = table_infos & 0x0F;
            let is_ac_table: bool = (table_infos >> 4) & 0x0F == 1;
            assert!(table_id <= 3);

            let len: usize;

            if is_ac_table {
                len = self.ac_tables
                    .get_mut(table_id as usize)
                    .expect("Should not panic")
                    .from_binary(reader, table_id, is_ac_table)?;
            } else {
                len = self.dc_tables
                    .get_mut(table_id as usize)
                    .expect("Should not panic")
                    .from_binary(reader, table_id, is_ac_table)?;
            }

            count -= len as i32;
        }

        assert_eq!(0, count);

        Ok(())
    }

    fn read_segment_dqt(&mut self, reader: &mut JpegBitReader) -> Result<()> {
        let length: u16 = reader.read_word()?;
        let mut count: i32 = length as i32;
        count -= 2;
        
        while count > 0 {
            let table_infos: u8 = reader.read_byte()?;
            count -= 1;

            let table_id: u8 = table_infos & 0x0F;
            let element_size: u8 = (table_infos >> 4) & 0x0F;
            assert!(table_id <= 3);
            assert!(element_size <= 1);

            let len: usize = self.quantization_tables
                .get_mut(table_id as usize)
                .expect("Should not panic because vec initialized")
                .from_binary(reader, table_id, element_size)?;
            count -= len as i32;
        }

        assert_eq!(0, count);

        Ok(())
    }

    fn read_segment_dri(&mut self, reader: &mut JpegBitReader) -> Result<()> {
        let length: u16 = reader.read_word()?;
        assert_eq!(4, length);
        let mut count: i32 = length as i32;
        count -= 2;

        self.restart_interval = reader.read_word()?;
        count -= 2;

        assert_eq!(0, count);

        Ok(())
    }

    fn read_segment_sof0(&mut self, reader: &mut JpegBitReader) -> Result<()> {
                
        let length: u16 = reader.read_word()?;
        let mut count: i32 = length as i32;
        count -= 2;

        let precision: u8 = reader.read_byte()?;
        assert_eq!(8, precision);
        count -= 1;

        let height: u16 = reader.read_word()?;
        assert_ne!(0, height);
        self.height = height;
        count -= 2;

        let width: u16 = reader.read_word()?;
        assert_ne!(0, width);
        self.width = width;
        count -= 2;

        let component_numbers: u8 = reader.read_byte()?;
        assert!(component_numbers == 1 || component_numbers == 3);
        count -= 1;

        let mut zero_based: bool = false;

        for i in 0..component_numbers {
            let mut component_id: u8 = reader.read_byte()?;
            assert!(component_id <= 3);

            if component_id == 0 && i == 0 {
                zero_based = true;
            }

            if zero_based {
                component_id += 1;
            }

            assert_ne!(0, component_id);
            assert!(component_id <= component_numbers);

            self.color_components
                .get_mut((component_id - 1) as usize)
                .expect("Should not panic because vec initialized")
                .from_binary(reader)?;
            count -= 3;
        }

        self.components_number = component_numbers;
        self.zero_based = zero_based;

        assert_eq!(0, count);

        Ok(())
    }

    fn read_segment_sos(&mut self, reader: &mut JpegBitReader) -> Result<()> {
        assert_ne!(0, self.components_number);

        let length: u16 = reader.read_word()?;
        let mut count: i32 = length as i32;
        count -= 2;

        for component in self.color_components.iter_mut() {
            component.set_used_scan(false);
        }

        let components_number: u8 = reader.read_byte()?;
        count -= 1;

        for _ in 0..components_number {
            let mut component_id: u8 = reader.read_byte()?;
            count -= 1;

            if self.zero_based {
                component_id += 1;
            }

            assert!(component_id <= self.components_number);

            let color_component: &mut ColorComponent = self.color_components
                .get_mut(component_id as usize - 1usize)
                .expect("Should not panic");

            assert_eq!(true, color_component.used_frame());
            assert_eq!(false, color_component.used_scan());
            color_component.set_used_scan(true);

            let huffman_table_ids: u8 = reader.read_byte()?;
            count -= 1;

            let huffman_ac_table_id: u8 = huffman_table_ids & 0x0F;
            let huffman_dc_table_id: u8 = (huffman_table_ids >> 4) & 0x0F;

            assert!(huffman_ac_table_id <= 3);
            assert!(huffman_dc_table_id <= 3);

            color_component.set_huffman_ac_table_id(huffman_ac_table_id);
            color_component.set_huffman_dc_table_id(huffman_dc_table_id);
        }

        let start_of_selection: u8 = reader.read_byte()?;
        assert_eq!(0, start_of_selection);
        self.start_of_selection = start_of_selection;
        count -= 1;

        let end_of_selection: u8 = reader.read_byte()?;
        assert_eq!(63, end_of_selection);
        self.end_of_selection = end_of_selection;
        count -= 1;

        let successive_approximation: u8 = reader.read_byte()?;
        let successive_approximation_low: u8 = successive_approximation & 0x0F;
        let successive_approximation_high: u8 = (successive_approximation >> 4) & 0x0F;
        assert_eq!(0, successive_approximation_low);
        assert_eq!(0, successive_approximation_high);
        self.successive_approximation_low = successive_approximation_low;
        self.successive_approximation_high = successive_approximation_high;
        count -= 1;

        assert_eq!(0, count);

        Ok(())
    }

    pub fn quantization_tables(&self, index: usize) -> Option<&QuantizationTable> {
        self.quantization_tables.get(index)
    }

    pub fn ac_table(&self, index: usize) -> Option<&HuffmanTable> {
        self.ac_tables.get(index)
    }

    pub fn dc_table(&self, index: usize) -> Option<&HuffmanTable> {
        self.dc_tables.get(index)
    }

    pub fn generate_tables_codes(&mut self) -> () {
        for i in 0..4 {
            self.ac_tables
                .get_mut(i)
                .expect("Should not panic")
                .generate_codes();

            self.dc_tables
                .get_mut(i)
                .expect("Should not panic")
                .generate_codes();
        }
    }

    pub fn color_component(&self, index: usize) -> Option<&ColorComponent> {
        self.color_components.get(index)
    }

    pub fn components_number(&self) -> u8 {
        self.components_number
    }

    pub fn restart_interval(&self) -> u16 {
        self.restart_interval
    }

    pub fn mcu_width(&self) -> usize {
        (self.width as usize + 7) / 8
    }

    pub fn mcu_height(&self) -> usize {
        (self.height as usize + 7) / 8
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
            write!(f, "Huffman DC Table ID: {}\n", color_component.huffman_dc_table_id())?;
            write!(f, "Huffman AC Table ID: {}\n", color_component.huffman_ac_table_id())?;
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
