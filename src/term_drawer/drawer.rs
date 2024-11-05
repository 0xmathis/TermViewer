use anyhow::Result;
use clap::ValueEnum;
use serde::Serialize;
use std::io::{stdout, Write};
use terminal_size::{Width, Height, terminal_size};

use crate::image::bmp::BMP;
use crate::image::mcu::MCU;

#[derive(ValueEnum, Clone, Debug, Serialize)]
pub enum ScalingLevel {
    LEVEL1,
    LEVEL2,
}

pub fn level1(image: Box<BMP>, terminal_width: usize, terminal_height: usize) -> Result<()> {
    let image_height: usize = image.height() as usize;
    let image_width: usize = image.width() as usize;
    let mcus: &Vec<MCU> = image.mcus();
    let mcu_width: usize = (image_width as usize + 7) / 8;

    let ratio_width: usize = image_width / terminal_width;
    let ratio_height: usize = image_height / terminal_height;
    let step: usize = ratio_height.max(ratio_width);

    let mut image_row: usize = 0;
    let mut image_column: usize = 0;

    let size_per_pixel: usize = background(u8::MAX, u8::MAX, u8::MAX).len() + "  ".len();
    let size_per_row: usize = size_per_pixel * (image_width / step + 1) + goto(0, 0).len();
    let output_size: usize = size_per_row * (image_height / step + 1) + reset().len();
    let mut output: String = String::with_capacity(output_size);

    loop {
        let mcu_row: usize = image_row / 8;
        let pixel_row: usize = image_row % 8;
        let mcu_column: usize = image_column / 8;
        let pixel_column: usize = image_column % 8;
        let mcu_index: usize = mcu_row * mcu_width + mcu_column;
        let pixel_index: usize = pixel_row * 8 + pixel_column;

        if let Some(mcu) = mcus.get(mcu_index) {
            let r = mcu.component(0).expect("Should exist")[pixel_index] as u8;
            let g = mcu.component(1).expect("Should exist")[pixel_index] as u8;
            let b = mcu.component(2).expect("Should exist")[pixel_index] as u8;

            output.push_str(&background(r, g, b));
            output.push_str(&"  ");
        }

        image_column += step;

        if image_column >= image_width {
            image_column = 0;
            image_row += step;
            output.push_str(&goto(image_row / step, 0));
        }

        if image_row >= image_height {
            break;
        }
    }

    output.push_str(&reset());
    Ok(stdout().write_all(output.as_bytes())?)
}

fn average(mcus: &Vec<MCU>, mcu_width: usize, width: usize, height: usize, x: usize, y: usize, step: usize) -> (u8, u8, u8) {
    let mut sum_r: usize = 0;
    let mut sum_g: usize = 0;
    let mut sum_b: usize = 0;
    let mut count: usize = 0;

    for row in x..height.min(x + step) {
        for column in y..width.min(y + step) {
            let mcu_row: usize = row / 8;
            let pixel_row: usize = row % 8;
            let mcu_column: usize = column / 8;
            let pixel_column: usize = column % 8;
            let mcu_index: usize = mcu_row * mcu_width + mcu_column;
            let pixel_index: usize = pixel_row * 8 + pixel_column;

            if let Some(mcu) = mcus.get(mcu_index) {
                sum_r += mcu.component(0).expect("Should exist")[pixel_index] as usize;
                sum_g += mcu.component(1).expect("Should exist")[pixel_index] as usize;
                sum_b += mcu.component(2).expect("Should exist")[pixel_index] as usize;
                count += 1;
            }
        }
    }

    let average_r: usize = sum_r / count;
    let average_g: usize = sum_g / count;
    let average_b: usize = sum_b / count;

    (average_r as u8, average_g as u8, average_b as u8)
}

pub fn level2(image: Box<BMP>, terminal_width: usize, terminal_height: usize) -> Result<()> {
    let image_height: usize = image.height() as usize;
    let image_width: usize = image.width() as usize;
    let mcus: &Vec<MCU> = image.mcus();
    let mcu_width: usize = (image_width as usize + 7) / 8;

    let ratio_width: usize = image_width / terminal_width;
    let ratio_height: usize = image_height / terminal_height;
    let step: usize = ratio_height.max(ratio_width);

    let mut image_row: usize = 0;
    let mut image_column: usize = 0;

    let size_per_pixel: usize = background(u8::MAX, u8::MAX, u8::MAX).len() + "  ".len();
    let size_per_row: usize = size_per_pixel * (image_width / step + 1) + goto(0, 0).len();
    let output_size: usize = size_per_row * (image_height / step + 1) + reset().len();
    let mut output: String = String::with_capacity(output_size);

    loop {
        let (r, g, b): (u8, u8, u8) = average(mcus, mcu_width, image_width, image_height, image_row, image_column, step);
        output.push_str(&background(r, g, b));
        output.push_str(&"  ");

        image_column += step;

        if image_column >= image_width {
            image_column = 0;
            image_row += step;
            output.push_str(&goto(image_row / step, 0));
        }

        if image_row >= image_height {
            break;
        }
    }

    output.push_str(&reset());
    Ok(stdout().write_all(output.as_bytes())?)
}

pub fn draw(image: Box<BMP>, scaling_level: ScalingLevel) -> Result<()> {
    clean();
    hide_cursor();

    if let Some((Width(width), Height(height))) = terminal_size() {
        match scaling_level {
            ScalingLevel::LEVEL1 => level1(image, width as usize, height as usize)?,
            ScalingLevel::LEVEL2 => level2(image, width as usize, height as usize)?,
        }
    }

    println!("");
    show_cursor();
    Ok(())
}

fn background(r: u8, g: u8, b: u8) -> String {
    format!("\u{001b}[48;2;{r};{g};{b}m")
}

fn reset() -> String {
    format!("\u{001b}[0m")
}

fn show_cursor() {
    print!("\u{001b}[?25h");
}

fn hide_cursor() {
    print!("\u{001b}[?25l");
}

fn goto(row: usize, column: usize) -> String {
    format!("\u{001b}[{row};{column}H")
}

fn clean() {
    print!("\u{001b}[2J");
}
