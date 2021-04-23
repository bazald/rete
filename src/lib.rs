//! # Rete
//! 
//! rete implementation coming soon 👍

mod alpha_network;
mod float;
mod hash_trie;
mod map_reduce_map;
mod map_reduce_set;
mod node_id;
mod parallelization;
mod symbol;
#[macro_use] mod wasm_parallel;
mod wme;

use futures_channel::oneshot;
use js_sys::Promise;
use rayon::prelude::*;
use std::future::Future;
use std::sync::atomic::Ordering;
use wasm_bindgen::prelude::*;
use xarc::AtomicXarc;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn rete_init() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
    console_log!("Hello, log!");
}

#[wasm_bindgen]
pub fn par_sum(
    concurrency: usize,
    thread_pool: &wasm_parallel::pool::WorkerPool,
    data: Vec<i32>,
) -> Result<Promise, JsValue> {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(concurrency)
        .spawn_handler(|thread| Ok(thread_pool.run(|| thread.run()).unwrap()))
        .build()
        .unwrap();

    // And now execute the render! The entire render happens on our worker
    // threads so we don't lock up the main thread, so we ship off a thread
    // which actually does the whole rayon business. When our returned
    // future is resolved we can pull out the final version of the image.
    let (tx, rx) = oneshot::channel();
    let mut sum = 0;
    thread_pool.run(move || {
        pool.install(|| {
            sum = data
                .par_chunks(32)
                .map(|x| {let mut r = 0; for y in x {r += y;} r})
                .reduce(|| 0, |x, y| x + y);
        });
        let _ = tx.send(sum);
    })?;

    Ok(wasm_bindgen_futures::future_to_promise(async move {
        match rx.await {
            Ok(_data) => Ok(JsValue::from(_data)),
            Err(_) => Err(JsValue::undefined()),
        }
    }))
}
