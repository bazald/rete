//! # Rete
//! 
//! rete implementation coming soon 👍

mod hash_trie;
mod parallelization;
mod symbol;
mod wme;

use std::sync::atomic::Ordering;
use wasm_bindgen::prelude::*;
use xarc::AtomicXarc;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let x = AtomicXarc::new(42);
    let y = x.load(Ordering::Relaxed);
    alert(&format!("Hello, {}!", name));
}
