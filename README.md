## 🦀 detrend: Signal Detrending for Rust

[](https://www.google.com/search?q=https://crates.io/crates/detrend)
[](https://www.google.com/search?q=https://docs.rs/detrend)

A lightweight and efficient Rust crate for **removing underlying trends and baseline drift** from signals (time series data). This is essential in signal processing and data analysis to isolate meaningful fluctuations from long-term bias.

-----

### 💡 Example

```rust
use detrend::{detrend_f64, DetrendingMethod, DetrendError};

fn main() -> Result<(), DetrendError> {
    // A signal with a clear upward trend: y = 0.5 * x + 1.0
    let signal = vec![1.0, 1.5, 2.0, 2.5, 3.0];

    // Remove the linear trend using Least Squares
    let detrended_ls = detrend_f64(&signal, DetrendingMethod::LeastSquares)?;

    Ok(())
}
```

### 🧩 Functions

The crate provides type-specific functions for precision and performance:

* `detrend_f32(signal: &[f32], method: DetrendingMethod)`
* `detrend_f64(signal: &[f64], method: DetrendingMethod)`

Both functions return a `Result<Vec<T>, DetrendError>`, primarily for handling potential **`DetrendError::AllocationError`** in low-memory environments.

-----

This project is licensed under either of

- BSD-3-Clause License (see [LICENSE](LICENSE.md))
- Apache License, Version 2.0 (see [LICENSE](LICENSE-APACHE.md))

at your option.