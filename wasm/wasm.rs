mod util;

pub mod game;
pub mod strategy;

use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    util::set_panic_hook();

    Ok(())
}
