#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use is_valid_strftime::is_valid_strftime;

wasm_bindgen_test_configure!();

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(is_valid_strftime("%Y-%m-%d %H:%M:%S"), true);
}
