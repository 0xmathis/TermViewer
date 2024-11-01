pub struct BitReader<'a> {
    next_bit: usize,
    next_byte: usize,
    data: &'a Vec<u8>,
}

impl <'a> BitReader<'a> {
    pub fn new(data: &'a Vec<u8>) -> Self {
        Self {
            next_bit: 0,
            next_byte: 0,
            data,
        }
    }

    pub fn read_bit(&mut self) -> Option<u8> {
        if self.next_byte >= self.data.len() {
            return None;
        }

        let bit: u8 = (self.data[self.next_byte] >> (7 - self.next_bit)) & 0x1;
        self.next_bit += 1;

        if self.next_bit == 8 {
            self.next_bit = 0;
            self.next_byte += 1;
        }

        Some(bit)
    }

    pub fn read_bits(&mut self, length: usize) -> Option<i32> {
        let mut bits: i32 = 0;

        for _ in 0..length {
            if let Some(bit) = self.read_bit() {
                bits = (bits << 1) | bit as i32;
            } else {
                return None;
            }
        }

        Some(bits)
    }

    pub fn align(&mut self) -> () {
        if self.next_byte < self.data.len() && self.next_bit != 0 {
            self.next_bit = 0;
            self.next_byte += 1;
        }
    }
}
