extern crate proc_macro;
extern crate worker;
use worker::prelude::*;

use proc_macro::TokenStream;

#[proc_macro]
pub fn include_font(input: TokenStream) -> TokenStream {
    let params: Params = syn::parse(input).unwrap();
    let font = worker::work(params);

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
