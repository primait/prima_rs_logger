# PrimaRsLogger

[![Version](https://img.shields.io/crates/v/prima_rs_logger.svg)](https://crates.io/crates/prima_rs_logger)
[![Downloads](https://img.shields.io/crates/d/prima_rs_logger.svg)](https://crates.io/crates/prima_rs_logger)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/prima_rs_logger)

## Installation

Just include `prima_rs_logger = "^0.1"` in your Cargo.toml

## Code example

There are two different type of guard:
- term guard => log stuff as plain string
- json guard => encode everything in json format

```rust
use prima_rs_logger::{info, GuardLoggerCell};

// Singleton logger. Used to free user from manually passing Logger objects around.
static LOGGER_GUARD: GuardLoggerCell = GuardLoggerCell::new();

fn main() {
    let app_name: &str = "myapp";
    let guard = prima_rs_logger::term_guard(app_name);
    LOGGER_GUARD.set(guard).expect("Cannot set global logger guard");
    
    info!("Starting {}", app_name; "meta" => "data");
}
```
