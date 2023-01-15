use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_v1() {
    log("wasm loaded success!");
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    log(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
