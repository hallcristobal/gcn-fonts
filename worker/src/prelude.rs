pub const I8_BLOCK_WIDTH: usize = 8;
pub const I8_BLOCK_HEIGHT: usize = 4;

pub use image::imageops::overlay;
pub use image::GrayImage;
pub use rusttype::gpu_cache::CacheBuilder;
pub use rusttype::Point;
pub use rusttype::{Font, Rect, Scale};
pub use syn;
pub use syn::{synom::Synom, LitFloat, LitInt, LitStr};

#[derive(Debug)]
pub struct Glyph {
    pub descender: f32,
    pub bounds: Rect<f32>,
}

pub struct Resolution {
    pub width: LitInt,
    pub height: LitInt,
}

impl Synom for Resolution {
    named!(parse -> Self, do_parse!(
        custom_keyword!(resolution) >>
        punct!(:) >>
        width: syn!(LitInt) >>
        punct!(*) >>
        height: syn!(LitInt) >>
        option!(punct!(,)) >>
        (Resolution {
            width,
            height,
        })
    ));
}

pub struct Size {
    pub size: LitFloat,
}

impl Synom for Size {
    named!(parse -> Self, do_parse!(
        custom_keyword!(size) >>
        punct!(:) >>
        size: syn!(LitFloat) >>
        option!(punct!(,)) >>
        (Size {
            size,
        })
    ));
}

pub struct Params {
    pub path: LitStr, // 💯🔥😂👌
    pub resolution: Option<Resolution>,
    pub size: Option<Size>,
}

impl Synom for Params {
    named!(parse -> Self, do_parse!(
        custom_keyword!(path) >>
        punct!(:) >>
        path: syn!(LitStr) >>
        option!(punct!(,)) >>
        resolution: option!(syn!(Resolution)) >>
        size: option!(syn!(Size)) >>
        (Params {
            path,
            resolution,
            size,
        })
    ));
}
