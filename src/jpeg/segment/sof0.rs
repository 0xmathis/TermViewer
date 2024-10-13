use std::fmt;
use std::fs::File;
use std::io::Read;

use super::SegmentType;

#[derive(Debug, Clone)]
struct ColorComponent {
    horizontal_sampling_factor: u8,
    vertical_sampling_factor: u8,
    quantization_table_id: u8,
}

impl ColorComponent {
    pub fn from_binary(&mut self, file: &mut File) -> usize {
        let mut buffer: [u8; 1] = [0; 1];
        file.read_exact(&mut buffer).unwrap();
        let sampling_factor: u8 = buffer[0];

        file.read_exact(&mut buffer).unwrap();
        let quantization_table_id: u8 = buffer[0];
        assert!(quantization_table_id <= 3);

        self.horizontal_sampling_factor = (sampling_factor >> 4) & 0x0F;
        self.vertical_sampling_factor = sampling_factor & 0x0F;
        self.quantization_table_id = quantization_table_id;

        3
    }
}

impl Default for ColorComponent {
    fn default() -> Self {
        Self {
            horizontal_sampling_factor: 1,
            vertical_sampling_factor: 1,
            quantization_table_id: 0,
        }
    }
}

#[derive(Debug)]
pub struct SOF0 {
    segment_type: SegmentType,
    length: u16,
    precision: u8,
    height: u16,
    width: u16,
    component_numbers: u8,
    components: Vec<ColorComponent>,
    data: Vec<u8>,
}

impl SOF0 {
    pub fn from_binary(file: &mut File) -> Self {
        let mut length_buffer: [u8; 2] = [0; 2];
        file.read_exact(&mut length_buffer).unwrap();
        let length: u16 = ((length_buffer[0] as u16) << 8) + length_buffer[1] as u16;
        let mut count: i32 = length as i32;
        count -= 2;

        let mut precision_buffer: [u8; 1] = [0; 1];
        file.read_exact(&mut precision_buffer).unwrap();
        let precision: u8 = precision_buffer[0];
        assert_eq!(precision, 8);
        count -= 1;

        let mut height_buffer: [u8; 2] = [0; 2];
        file.read_exact(&mut height_buffer).unwrap();
        let height: u16 = ((height_buffer[0] as u16) << 8) + height_buffer[1] as u16;
        assert_ne!(height, 0);
        count -= 2;

        let mut width_buffer: [u8; 2] = [0; 2];
        file.read_exact(&mut width_buffer).unwrap();
        let width: u16 = ((width_buffer[0] as u16) << 8) + width_buffer[1] as u16;
        assert_ne!(width, 0);
        assert_ne!(width, 4);
        count -= 2;

        let mut component_numbers_buffer: [u8; 1] = [0; 1];
        file.read_exact(&mut component_numbers_buffer).unwrap();
        let component_numbers: u8 = component_numbers_buffer[0];
        assert!(component_numbers == 1 || component_numbers == 3);
        count -= 1;

        let mut components: Vec<ColorComponent> = Vec::new();
        components.resize(4, ColorComponent::default());

        for _ in 0..component_numbers {
            let mut component_id_buffer: [u8; 1] = [0; 1];
            file.read_exact(&mut component_id_buffer).unwrap();
            let component_id: u8 = component_id_buffer[0];
            assert!(1 <= component_id && component_id <= 3);

            let len: usize = components
                .get_mut((component_id - 1) as usize)
                .expect("Should not panic because vec initialized")
                .from_binary(file);
            count -= len as i32;
        }

        let mut data: Vec<u8> = Vec::new();
        data.resize(count as usize, 0);
        file.read_exact(&mut data).unwrap();
        count -= count;

        assert_eq!(count, 0);

        Self {
            segment_type: SegmentType::SOF0,
            length,
            precision,
            height,
            width,
            component_numbers,
            components,
            data,
        }
    }
}

impl fmt::Display for SOF0 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "segment_type: {:?} | length: {} | precision: {} | height x width: {}x{}, component_numbers: {}\n", self.segment_type, self.length, self.precision, self.height, self.width, self.component_numbers)?;

        for i in 0..self.component_numbers {
            let i = i as usize;
            let component: &ColorComponent = &self.components[i];

            write!(f, "Component ID: {}\n", i+1)?;
            write!(f, "Component data:\n")?;
            write!(f, "\tHorizontal sampling factor: {:01X}\n", component.horizontal_sampling_factor)?;
            write!(f, "\tVertical sampling factor: {:01X}\n", component.vertical_sampling_factor)?;
            write!(f, "\tQuantization table ID: {:02X}\n", component.quantization_table_id)?;
        }

        Ok(())
    }
}
