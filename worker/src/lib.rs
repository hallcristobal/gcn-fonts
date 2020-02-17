extern crate image;
extern crate proc_macro;
extern crate rusttype;

pub mod prelude;

use crate::prelude::*;

use std::fs;
use std::io::prelude::*;

#[repr(align(32))]
pub struct AlignedData<T>(pub T);

pub struct GCNFont {
    pub width: u32,
    pub height: u32,
    pub size: f32,
    pub space_advance: f32,
    pub data: Vec<u8>,
    pub glyphs: Vec<Glyph>,
}

pub fn work(params: Params) -> GCNFont {
    let (width, height) = params
        .resolution
        .map(|r| (r.width, r.height))
        .unwrap_or((256, 256));
    let size = params.size.map(|s| s.size).unwrap_or(50.0);
    let scale = Scale::uniform(size);

    let font_data = fs::read(params.path).unwrap();
    let font = Font::from_bytes(&font_data).expect("Error constructing Font");
    let mut atlas = GrayImage::new(width, height);

    let mut cache = CacheBuilder {
        width,
        height,
        ..CacheBuilder::default()
    }
    .build();

    let space_advance = font.glyph(' ').scaled(scale).h_metrics().advance_width;

    let glyphs = font
        .glyphs_for((0x21..=0x7E).map(|i: u8| i as char))
        .map(|g| g.scaled(scale).positioned(Point { x: 0.0, y: 0.0 }))
        .collect::<Vec<_>>();

    for glyph in &glyphs {
        cache.queue_glyph(0, glyph.clone());
    }

    cache
        .cache_queued(|rect, data| {
            let glyph = GrayImage::from_raw(rect.width(), rect.height(), data.to_vec())
                .expect("Bad GrayImage");
            overlay(&mut atlas, &glyph, rect.min.x, rect.min.y);
        })
        .expect("cache queue");

    let rects = glyphs
        .iter()
        .map(|glyph| {
            Glyph {
                descender: glyph.pixel_bounding_box().unwrap().max.y as f32,
                bounds: cache
                    .rect_for(0, glyph)
                    .unwrap() //expect("Failed to get rect.")
                    .unwrap() //expect("Failed to unwrap TextureCoords")
                    .0,
            }
        })
        .collect::<Vec<_>>();

    let mut buffer = Vec::with_capacity(width as usize * height as usize);

    {
        for row in 0..(height as usize / I8_BLOCK_HEIGHT) {
            let row_y = row * I8_BLOCK_HEIGHT;
            for column in 0..(width as usize / I8_BLOCK_WIDTH) {
                let column_x = column * I8_BLOCK_WIDTH;
                for y in 0..I8_BLOCK_HEIGHT {
                    let y = row_y + y;
                    let x = column_x;
                    let pixel_index = y * width as usize + x;
                    let src = &(*atlas)[pixel_index..][..I8_BLOCK_WIDTH];
                    buffer.write_all(src).unwrap();
                }
            }
        }
    }

    GCNFont {
        width,
        height,
        size,
        space_advance,
        data: buffer,
        glyphs: rects,
    }
}
