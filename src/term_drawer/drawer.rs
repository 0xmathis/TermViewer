use std::u16;

use terminal_size::{Width, Height, terminal_size};

use crate::image::mcu::MCU;
use crate::image::Image;
use crate::image::bmp::BMP;

pub fn level1(image: &BMP, terminal_width: usize, terminal_height: usize) -> () {
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

            output += &background(r, g, b);
            output += "  ";
        }

        image_column += step;

        if image_column >= image_width {
            image_column = 0;
            image_row += step;
            output += &goto(image_row / step, 0);
        }

        if image_row >= image_height {
            break;
        }
    }

    output += &reset();
    println!("cap: {} ; len: {}", output.capacity(), output.len());
    // print!("{output}");
}

// pub fn level2(image: BMP, terminal_width: u16, terminal_height: u16) -> () {
//     todo!();
// }

// pub fn level3(image: BMP, terminal_width: u16, terminal_height: u16) -> () {
//     todo!();
// }

pub fn draw(image: BMP) -> () {
    clean();
    hide_cursor();

    if let Some((Width(width), Height(height))) = terminal_size() {
        level1(&image, width as usize, height as usize);
    }

    println!("");
    show_cursor();
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
