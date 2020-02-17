pub const I8_BLOCK_WIDTH: usize = 8;
pub const I8_BLOCK_HEIGHT: usize = 4;

pub use image::imageops::overlay;
pub use image::GrayImage;
pub use rusttype::gpu_cache::CacheBuilder;
pub use rusttype::Point;
pub use rusttype::{Font, Rect, Scale};

#[derive(Debug)]
pub struct Glyph {
    pub descender: f32,
    pub bounds: Rect<f32>,
}

pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

pub struct Size {
    pub size: f32,
}

pub struct Params {
    pub path: String,
    pub resolution: Option<Resolution>,
    pub size: Option<Size>,
}
