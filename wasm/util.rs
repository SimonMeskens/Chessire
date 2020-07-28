use wasm_bindgen::JsValue;

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn random() -> f64 {
    let result = js_sys::Math::random();
    result
}

pub fn log(message: &str) {
    web_sys::console::log_1(&JsValue::from(message));
}

pub fn display_fen(fen: String) {
    let board = fen.parse::<chess::Board>().unwrap();
}
