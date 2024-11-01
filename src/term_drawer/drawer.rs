use terminal_size::{Width, Height, terminal_size};

use crate::image::mcu::MCU;
use crate::image::Image;
use crate::image::bmp::BMP;

pub fn level1(image: &BMP, terminal_width: u16, terminal_height: u16) -> () {
    let image_height: u16 = image.height();
    let image_width: u16 = image.width();
    let mcus: &Vec<MCU> = image.mcus();

    let ratio_width: u16 = image_width / terminal_width;
    let ratio_height: u16 = image_height / terminal_height;
    let step: u16 = ratio_height.max(ratio_width);
    let mcu_width: usize = (image_width as usize + 7) / 8;

    let mut row: u16 = 0;
    let mut column: u16 = 0;

    loop {
        let mcu_row: usize = row as usize / 8;
        let pixel_row: usize = row as usize % 8;
        let mcu_column: usize = column as usize / 8;
        let pixel_column: usize = column as usize % 8;
        let mcu_index: usize = mcu_row * mcu_width + mcu_column;
        let pixel_index: usize = pixel_row * 8 + pixel_column;

        if let Some(mcu) = mcus.get(mcu_index) {
            let r = mcu.component(0).expect("Should exist")[pixel_index] as u8;
            let g = mcu.component(1).expect("Should exist")[pixel_index] as u8;
            let b = mcu.component(2).expect("Should exist")[pixel_index] as u8;

            goto(row / step, (column / step) * 2);
            background(r, g, b);
            print!("  ");
        }

        column += step;

        if column >= image_width {
            column = 0;
            row += step;
        }

        if row >= image_height {
            break;
        }
    }

    reset();
}

pub fn level2(image: BMP, terminal_width: u16, terminal_height: u16) -> () {
    todo!();
}

pub fn level3(image: BMP, terminal_width: u16, terminal_height: u16) -> () {
    todo!();
}

pub fn draw(image: BMP) -> () {
    clean();
    hide_cursor();

    if let Some((Width(width), Height(height))) = terminal_size() {
        level1(&image, width, height);
    }

    println!("");
    show_cursor();
}

fn background(r: u8, g: u8, b: u8) -> () {
    print!("\u{001b}[48;2;{r};{g};{b}m");
}

fn reset() -> () {
    print!("\u{001b}[0m");
}

fn show_cursor() {
    print!("\u{001b}[?25h");
}

fn hide_cursor() {
    print!("\u{001b}[?25l");
}

fn goto(row: u16, column: u16) {
    print!("\u{001b}[{row};{column}H");
}

fn clean() {
    print!("\u{001b}[2J");
}
