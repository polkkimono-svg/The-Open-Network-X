#![no_main]

use libfuzzer_sys::fuzz_target;
use onx_state_model::BagOfCells;

// Exercises untrusted BoC parsing, including declared cell counts, cell
// lengths, reference resolution, hash validation, and DAG cycle detection.
fuzz_target!(|data: &[u8]| {
    let _ = BagOfCells::from_bytes(data);
});
