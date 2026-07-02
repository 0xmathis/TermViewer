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
