mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello {}, wasm-vector-benchmark!", name));
}


use easybench_wasm::{bench,bench_env};
use web_sys::console;

// Simple benchmarks are performed with `bench`.
console::log_1(&format!("fib 200: {}", bench(|| fib(200) )).into());
console::log_1(&format!("fib 500: {}", bench(|| fib(500) )).into());

// If a function needs to mutate some state, use `bench_env`.
console::log_1(&format!("reverse: {}", bench_env(vec![0;100], |xs| xs.reverse() )).into());
console::log_1(&format!("sort:    {}", bench_env(vec![0;100], |xs| xs.sort()    )).into());

