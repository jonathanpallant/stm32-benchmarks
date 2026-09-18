# stm32-benchmarks

Benchmarks on the STM32N657

Type `cargo run --release`. You might want
https://github.com/rust-lang/rust/pull/162519 to get the
`thumbv8.1m.main-none-eabihf` target.

Results at default clock speeds:

| Target                        | opt-level | target-cpu   | Cycles  |
|-------------------------------|-----------|--------------|---------|
| `thumbv8m.main-none-eabihf`   | `3`       | None         | 155,702 |
| `thumbv8.1m.main-none-eabihf` | `3`       | None         | 147,512 |
| `thumbv8m.main-none-eabihf`   | `3`       | `cortex-m55` | 47,174  |
| `thumbv8.1m.main-none-eabihf` | `3`       | `cortex-m55` | 47,174  |
| `thumbv8m.main-none-eabihf`   | `s`       | None         | 262,181 |
| `thumbv8.1m.main-none-eabihf` | `s`       | None         | 163,885 |
| `thumbv8m.main-none-eabihf`   | `s`       | `cortex-m55` | 163,885 |
| `thumbv8.1m.main-none-eabihf` | `s`       | `cortex-m55` | 163,884 |
