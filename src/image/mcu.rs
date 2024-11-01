use super::bit_reader::BitReader;
use super::huffman::HuffmanTable;
use super::mcu_component::MCUComponent;

#[derive(Debug, Clone, Copy, Default)]
pub struct MCU {
    components: [MCUComponent; 3],
}

impl MCU {
    pub fn component(&self, i: usize) -> Option<&MCUComponent> {
        self.components.get(i)
    }

    pub fn component_mut(&mut self, i: usize) -> Option<&mut MCUComponent> {
        self.components.get_mut(i)
    }

    fn next_symbol(reader: &mut BitReader, table: &HuffmanTable) -> Option<u8> {
        let mut code: u32 = 0;

        for i in 0..16 {
            let bit: u8 = reader.read_bit()?;
            code = (code << 1) | bit as u32;

            for j in table.offsets(i)..table.offsets(i+1) {
                if code == table.codes(j as usize) {
                    return Some(table.symbols(j as usize));
                }
            }
        }

        None
    }

    pub fn decode(&mut self, component_id: usize, reader: &mut BitReader, previous_dc: &mut i32, ac_table: &HuffmanTable, dc_table: &HuffmanTable) -> bool {
        let Some(component) = self.component_mut(component_id) else {
            return false;
        };

        let Some(length) = Self::next_symbol(reader, dc_table) else {
            return false;
        };
        assert!(length <= 11);

        let Some(mut dc_coefficient) = reader.read_bits(length as usize) else {
            return false;
        };

        if length != 0 && dc_coefficient < (1 << (length - 1)) {
            dc_coefficient -= (1 << length) - 1;
        }

        component[0] = dc_coefficient + *previous_dc;
        *previous_dc = component[0];

        // Get AC values for the component
        let mut i: usize = 1;
        let zigzag_map: [usize; 64] = Self::zigzag_map();

        while i < 64 {
            let Some(symbol) = Self::next_symbol(reader, ac_table) else {
                return false;
            };

            if symbol == 0x00 {
                return true;
            }

            let coefficient_length: u8 = symbol & 0x0F;
            if coefficient_length > 10 {
                return false;
            }

            let skip_zeros: u8 = (symbol >> 4) & 0x0F;

            if i + skip_zeros as usize >= 64 {
                return false;
            }

            i += skip_zeros as usize;

            if coefficient_length != 0 {
                let Some(mut coefficient) = reader.read_bits(coefficient_length as usize) else {
                    return false;
                };

                if coefficient < (1 << (coefficient_length - 1)) {
                    coefficient -= (1 << coefficient_length) - 1;
                }

                component[zigzag_map[i]] = coefficient;
            }

            i += 1;
        }

        true
    }

    const fn zigzag_map() -> [usize; 64] {
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

    pub fn ycbcr_to_rgb(&mut self) -> () {
        for i in 0..64 {
            let mut r: i32 = self.components[0][i] + (1.402 * self.components[2][i] as f32) as i32 + 128;
            let mut g: i32 = self.components[0][i] - (0.344 * self.components[1][i] as f32) as i32 - (0.714 * self.components[2][i] as f32) as i32 + 128;
            let mut b: i32 = self.components[0][i] + (1.772 * self.components[1][i] as f32) as i32 + 128;

            if r < 0 { r = 0; } else if r > 255 { r = 255; }
            if g < 0 { g = 0; } else if g > 255 { g = 255; }
            if b < 0 { b = 0; } else if b > 255 { b = 255; }

            self.components[0][i] = r;
            self.components[1][i] = g;
            self.components[2][i] = b;
        }
    }
}