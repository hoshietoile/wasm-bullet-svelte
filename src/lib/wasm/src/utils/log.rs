use wasm_bindgen::prelude::*;
use web_sys::console::log_1;

#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub fn log(s: &String) {
    log_1(&JsValue::from(s));
}

#[wasm_bindgen]
pub fn log_front(s: &str) {
    log(&format!("core: {}", s));
}
