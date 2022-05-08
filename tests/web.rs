//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use wasm_bindgen_test::*;

use gameboy_rust_webassembly_emulator::hardware::memory::{MAX_RAM, Memory};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn assert_memory_max() {
    assert_eq!(MAX_RAM, 65535)
}

#[wasm_bindgen_test]
fn assert_memory_cells_size() {
    let memory = Memory::new();
    assert_eq!(memory.length(), MAX_RAM)
}

