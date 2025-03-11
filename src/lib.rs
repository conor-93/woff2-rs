#![doc = include_str!("../readme.md")]
pub mod decode;

mod buffer_util;
mod checksum;
mod glyf_decoder;
mod magic_numbers;
mod ttf_header;
mod woff2;

#[cfg(test)]
mod test_resources;

pub use decode::convert_woff2_to_ttf;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::console;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn convert_woff2_wasm(data: &[u8]) -> usize {
    let start = now();

    let mut cursor = std::io::Cursor::new(data);
    let ttf = match convert_woff2_to_ttf(&mut cursor) {
        Ok(ttf) => ttf,
        Err(_) => return 0,
    };

    let duration = now() - start;
    console::log_1(&format!("Conversion completed in {:.2} ms", duration).into());

    ttf.len()
}

#[cfg(target_arch = "wasm32")]
fn now() -> f64 {
    web_sys::window().unwrap().performance().unwrap().now()
}