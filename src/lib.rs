use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn fibonacci(n: u32) -> Vec<u32> {
    let mut sequence = vec![0, 1];
    for i in 2..=n as usize {
        let next = sequence[i - 1] + sequence[i - 2];
        sequence.push(next);
    }
    sequence.truncate((n + 1) as usize);
    sequence
}
