#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

pub mod utils;
pub mod hardware;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(target_family = "wasm")]
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub fn greet() {
    alert("Hello, gameboy_rust_webassembly_emulator!");
}