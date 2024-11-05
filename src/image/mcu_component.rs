use std::ops::{Index, IndexMut};

use super::quantization_table::QuantizationTable;

#[derive(Debug, Clone, Copy)]
pub struct MCUComponent([i32; 64]);

impl MCUComponent {
    pub fn dequantize(&mut self, table: &QuantizationTable) -> () {
        for i in 0..64 {
            self[i] *= table.table(i) as i32;
        }
    }

    fn dct_aan(input: [f32; 8], dct_m: &[f32; 6]) -> [f32; 8] {
        let f0: f32 = input[0];
        let f1: f32 = input[1];
        let f2: f32 = input[2];
        let f3: f32 = input[3];
        let f4: f32 = input[4] - input[7];
        let f5: f32 = input[5] + input[6];
        let f6: f32 = input[5] - input[6];
        let f7: f32 = input[4] + input[7];

        let e0: f32 = f0;
        let e1: f32 = f1;
        let e2: f32 = f2 - f3;
        let e3: f32 = f2 + f3;
        let e4: f32 = f4;
        let e5: f32 = f5 - f7;
        let e6: f32 = f6;
        let e7: f32 = f5 + f7;
        let e8: f32 = f4 + f6;

        let d0: f32 = e0;
        let d1: f32 = e1;
        let d2: f32 = e2 * dct_m[1];
        let d3: f32 = e3;
        let d4: f32 = e4 * dct_m[2];
        let d5: f32 = e5 * dct_m[3];
        let d6: f32 = e6 * dct_m[4];
        let d7: f32 = e7;
        let d8: f32 = e8 * dct_m[5];

        let c0: f32 = d0 + d1;
        let c1: f32 = d0 - d1;
        let c2: f32 = d2 - d3;
        let c3: f32 = d3;
        let c4: f32 = d4 + d8;
        let c5: f32 = d5 + d7;
        let c6: f32 = d6 - d8;
        let c7: f32 = d7;
        let c8: f32 = c5 - c6;

        [
            c0 + c3,
            c1 + c2,
            c1 - c2,
            c0 - c3,
            c4 - c8,
            c8,
            c6 - c7,
            c7,
        ]

    }

    // Use of the AAN algorithm
    // https://unix4lyfe.org/dct/
    // https://unix4lyfe.org/dct-1d/
    pub fn inverse_dct(&mut self, dct_m: &[f32; 6], dct_s: &[f32; 8]) -> () {
        let mut intermediate: [f32; 64] = [0f32; 64];

        for j in 0..8 {
            let aan_input: [f32; 8] = [
                self[0 * 8 + j] as f32 * dct_s[0],
                self[4 * 8 + j] as f32 * dct_s[4],
                self[2 * 8 + j] as f32 * dct_s[2],
                self[6 * 8 + j] as f32 * dct_s[6],
                self[5 * 8 + j] as f32 * dct_s[5],
                self[1 * 8 + j] as f32 * dct_s[1],
                self[7 * 8 + j] as f32 * dct_s[7],
                self[3 * 8 + j] as f32 * dct_s[3],
            ];

            let aan_output: [f32; 8] = Self::dct_aan(aan_input, dct_m);

            intermediate[0 * 8 + j] = aan_output[0] + aan_output[7];
            intermediate[1 * 8 + j] = aan_output[1] + aan_output[6];
            intermediate[2 * 8 + j] = aan_output[2] + aan_output[5];
            intermediate[3 * 8 + j] = aan_output[3] + aan_output[4];
            intermediate[4 * 8 + j] = aan_output[3] - aan_output[4];
            intermediate[5 * 8 + j] = aan_output[2] - aan_output[5];
            intermediate[6 * 8 + j] = aan_output[1] - aan_output[6];
            intermediate[7 * 8 + j] = aan_output[0] - aan_output[7];
        }

        for i in 0..8 {
            let aan_input: [f32; 8] = [
                intermediate[i * 8 + 0] * dct_s[0],
                intermediate[i * 8 + 4] * dct_s[4],
                intermediate[i * 8 + 2] * dct_s[2],
                intermediate[i * 8 + 6] * dct_s[6],
                intermediate[i * 8 + 5] * dct_s[5],
                intermediate[i * 8 + 1] * dct_s[1],
                intermediate[i * 8 + 7] * dct_s[7],
                intermediate[i * 8 + 3] * dct_s[3],
            ];

            let aan_output: [f32; 8] = Self::dct_aan(aan_input, dct_m);

            self[i * 8 + 0] = (aan_output[0] + aan_output[7] + 0.5f32) as i32;
            self[i * 8 + 1] = (aan_output[1] + aan_output[6] + 0.5f32) as i32;
            self[i * 8 + 2] = (aan_output[2] + aan_output[5] + 0.5f32) as i32;
            self[i * 8 + 3] = (aan_output[3] + aan_output[4] + 0.5f32) as i32;
            self[i * 8 + 4] = (aan_output[3] - aan_output[4] + 0.5f32) as i32;
            self[i * 8 + 5] = (aan_output[2] - aan_output[5] + 0.5f32) as i32;
            self[i * 8 + 6] = (aan_output[1] - aan_output[6] + 0.5f32) as i32;
            self[i * 8 + 7] = (aan_output[0] - aan_output[7] + 0.5f32) as i32;
        }
    }
}

impl Default for MCUComponent {
    fn default() -> Self {
        Self::from([0; 64])
    }
}

impl Index<usize> for MCUComponent {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for MCUComponent {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl From<[i32; 64]> for MCUComponent {
    fn from(value: [i32; 64]) -> Self {
        Self {
            0: value,
        }
    }
}
