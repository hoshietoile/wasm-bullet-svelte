// NOTE: これがあるとビルドが反映されない
// extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello {}~", name));
}

#[cfg(test)]
pub mod tests {
    #[test]
    pub fn test1() {
        assert_eq!(2 + 2, 4);
    }
}
