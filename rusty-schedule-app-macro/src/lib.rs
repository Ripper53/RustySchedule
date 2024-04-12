extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn preload_icon(_item: TokenStream) -> TokenStream {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/images/icon.png");
    //println!("{path:?}");
    let icon = match image::open(path) {
        Ok(icon) => icon.into_rgb8(),
        Err(e) => {
            panic!("{e:?}");
        },
    };
    let (width, height) = icon.dimensions();
    let rgba = icon.into_raw();
    quote::quote!((vec![#(#rgba),*], #width, #height)).into()
}
