use terminal_size::{Width, Height, terminal_size};

use crate::image::Image;
use crate::image::bmp::BMP;

pub fn basic(image: BMP, terminal_width: u16, terminal_height: u16) -> () {
    assert!(image.width() > terminal_width);
    assert!(image.height() > terminal_height);

    let ratio_width: u16 = image.width() / terminal_width;
    let ratio_height: u16 = image.height() / terminal_height;
    let ratio: u16 = ratio_height.max(ratio_width);
    let mcu_width: u32 = ((image.width() + 7) / 8) as u32;

    let mut x: u16 = 0;
    let mut y: u16 = 0;

    loop {
        let mcu_row: u32 = y as u32 / 8;
        let pixel_row: u32 = y as u32 % 8;
        let mcu_column: u32 = x as u32 / 8;
        let pixel_column: u32 = x as u32 % 8;
        let mcu_index: usize = (mcu_row * mcu_width + mcu_column) as usize;
        let pixel_index: usize = (pixel_row * 8 + pixel_column) as usize;

        if let Some(mcu) = image.mcus().get(mcu_index) {
            let r = mcu.component(0).expect("Should exist")[pixel_index] as u8;
            let g = mcu.component(1).expect("Should exist")[pixel_index] as u8;
            let b = mcu.component(2).expect("Should exist")[pixel_index] as u8;

            goto(y / ratio, (x / ratio) * 2);
            background(r, g, b);
            print!("  ");
        }

        y += ratio;

        if y >= image.width() {
            y = 0;
            x += ratio;
        }

        if x >= image.height() {
            break;
        }
    }
}

pub fn deluxe(image: BMP) -> () {
    todo!();
}

pub fn premium(image: BMP) -> () {
    todo!();
}

pub fn draw(image: BMP) -> () {
    clean();
    show_cursor();

    if let Some((Width(width), Height(height))) = terminal_size() {
        let ratio_width: u16 = image.width() / width;
        let ratio_height: u16 = image.height() / height;
        // goto(height as usize, 0);
        // println!("window: {width}x{height}");
        // println!("image:  {}x{}", image.width(), image.height());
        // println!("ratio:  {}x{}", ratio_width, ratio_height);
        // println!("nimage: {}x{}", image.width() / ratio_width, image.height() / ratio_height);
        // println!("mult:   {}x{}", width * ratio_width, height * ratio_height);
        basic(image, width, height);
    }

    // go_home();
}

fn background(r: u8, g: u8, b: u8) -> () {
    print!("\u{001b}[48;2;{r};{g};{b}m");
}

fn show_cursor() {
    print!("\u{001b}[?25h");
}

fn hide_cursor() {
    print!("\u{001b}[?25l");
}

fn go_home() {
    print!("\u{001b}[H");
}

fn goto(row: u16, column: u16) {
    print!("\u{001b}[{row};{column}H");
}

fn clean() {
    print!("\u{001b}[2J");
}
