use anyhow::Result;
use image::{DynamicImage, GenericImageView};
use std::io::{Write, stdout};
use terminal_size::{Height, Width, terminal_size};

use crate::drawer::{Drawer, ScalingLevel};

pub struct PixelDrawer;

impl PixelDrawer {
    fn level1(image: DynamicImage, terminal_width: usize, terminal_height: usize) -> Result<()> {
        let image_height: usize = image.height() as usize;
        let image_width: usize = image.width() as usize;

        let ratio_width: usize = image_width / terminal_width;
        let ratio_height: usize = image_height / terminal_height;
        let step: usize = ratio_height.max(ratio_width);

        let mut image_row: usize = 0;
        let mut image_column: usize = 0;

        let size_per_pixel: usize = Self::background(u8::MAX, u8::MAX, u8::MAX).len() + "  ".len();
        let size_per_row: usize = size_per_pixel * (image_width / step + 1) + Self::goto(0, 0).len();
        let output_size: usize = size_per_row * (image_height / step + 1) + Self::reset().len();
        let mut output: String = String::with_capacity(output_size);

        loop {
            let pixel = image.get_pixel(image_column as u32, image_row as u32);
            let [r, g, b, _] = pixel.0;

            output.push_str(&Self::background(r, g, b));
            output.push_str(&"  ");

            image_column += step;

            if image_column >= image_width {
                image_column = 0;
                image_row += step;
                output.push_str(&Self::goto(image_row / step, 0));
            }

            if image_row >= image_height {
                break;
            }
        }

        output.push_str(&Self::reset());
        Ok(stdout().write_all(output.as_bytes())?)
    }

    fn average(image: &DynamicImage, width: usize, height: usize, x: usize, y: usize, step: usize) -> (u8, u8, u8) {
        let mut sum_r: usize = 0;
        let mut sum_g: usize = 0;
        let mut sum_b: usize = 0;
        let mut count: usize = 0;

        for row in x..height.min(x + step) {
            for column in y..width.min(y + step) {
                let pixel = image.get_pixel(column as u32, row as u32);
                let [r, g, b, _] = pixel.0;
                sum_r += r as usize;
                sum_g += g as usize;
                sum_b += b as usize;
                count += 1;
            }
        }

        let average_r: usize = sum_r / count;
        let average_g: usize = sum_g / count;
        let average_b: usize = sum_b / count;

        (average_r as u8, average_g as u8, average_b as u8)
    }

    fn level2(image: DynamicImage, terminal_width: usize, terminal_height: usize) -> Result<()> {
        let image_height: usize = image.height() as usize;
        let image_width: usize = image.width() as usize;

        let ratio_width: usize = image_width / terminal_width;
        let ratio_height: usize = image_height / terminal_height;
        let step: usize = ratio_height.max(ratio_width);

        let mut image_row: usize = 0;
        let mut image_column: usize = 0;

        let size_per_pixel: usize = Self::background(u8::MAX, u8::MAX, u8::MAX).len() + "  ".len();
        let size_per_row: usize = size_per_pixel * (image_width / step + 1) + Self::goto(0, 0).len();
        let output_size: usize = size_per_row * (image_height / step + 1) + Self::reset().len();
        let mut output: String = String::with_capacity(output_size);

        loop {
            let (r, g, b): (u8, u8, u8) = Self::average(&image, image_width, image_height, image_row, image_column, step);
            output.push_str(&Self::background(r, g, b));
            output.push_str(&"  ");

            image_column += step;

            if image_column >= image_width {
                image_column = 0;
                image_row += step;
                output.push_str(&Self::goto(image_row / step, 0));
            }

            if image_row >= image_height {
                break;
            }
        }

        output.push_str(&Self::reset());
        Ok(stdout().write_all(output.as_bytes())?)
    }

    pub fn draw(&self, image: DynamicImage, scaling_level: ScalingLevel) -> Result<()> {
        Self::clean();
        Self::hide_cursor();

        if let Some((Width(width), Height(height))) = terminal_size() {
            match scaling_level {
                ScalingLevel::LEVEL1 => Self::level1(image, width as usize, height as usize)?,
                ScalingLevel::LEVEL2 => Self::level2(image, width as usize, height as usize)?,
            }
        }

        println!("");
        Self::show_cursor();
        Ok(())
    }
}

impl Drawer for PixelDrawer { }
