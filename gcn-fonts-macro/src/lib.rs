extern crate proc_macro;
#[macro_use]
extern crate syn;
extern crate worker;
use worker::prelude;

use proc_macro::TokenStream;

use syn::{synom::Synom, LitFloat, LitInt, LitStr};

struct Resolution {
    width: LitInt,
    height: LitInt,
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

struct Size {
    size: LitFloat,
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

struct Params {
    path: LitStr, // 💯🔥😂👌
    resolution: Option<Resolution>,
    size: Option<Size>,
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

#[proc_macro]
pub fn include_font(input: TokenStream) -> TokenStream {
    let params: Params = syn::parse(input).unwrap();
    let font = worker::work(prelude::Params {
        path: params.path.value(),
        resolution: params
            .resolution
            .map(|r| {
                Some(prelude::Resolution {
                    width: r.width.value() as u32,
                    height: r.height.value() as u32,
                })
            })
            .unwrap_or(None),
        size: params.size.map(|s| prelude::Size {
            size: s.size.value() as f32,
        }),
    });

    let tokens = format!(
        r#"Font {{
        width: {}.0,
        height: {}.0,
        size: {:?},
        space_advance: {:?},
        data: {{
            static DATA: AlignedData<[u8; {} * {}]> = AlignedData({:?});
            &DATA.0
        }},
        glyphs: &{:?},
    }}"#,
        font.width,
        font.height,
        font.size,
        font.space_advance,
        font.width,
        font.height,
        &font.data,
        font.glyphs
    );

    tokens.parse().unwrap()
}
