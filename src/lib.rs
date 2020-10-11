//! # Rete
//! 
//! rete implementation coming soon 👍

mod alpha_network;
mod float;
mod hash_trie;
mod map_reduce_map;
mod map_reduce_set;
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
