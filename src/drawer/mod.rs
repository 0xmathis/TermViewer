use clap::ValueEnum;
use serde::Serialize;

mod mcu_drawer;
mod pixel_drawer;

pub use mcu_drawer::McuDrawer;
pub use pixel_drawer::PixelDrawer;

#[derive(ValueEnum, Clone, Debug, Serialize)]
pub enum ScalingLevel {
    LEVEL1,
    LEVEL2,
}

trait Drawer {
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
}
