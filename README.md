# Rogger

Logging macros for Rinrin.rs  
This crate is VERY optimized for Rinrin.rs
so it may not be suitable for other projects.  
but usage is very simple and easy.

# Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
rogger = "*"
chrono = "0.4"
colored = "2"
```

Choose the version of the crate freely
but it is safer to do as above.

# Usage

```rust
use rogger::*;
use chrono::Utc;
use colored::Colorize;

fn main() {
    let ver = "0.1.0";
    info!("Version: {}", ver);

    let dev = "Foo PC";
    warn!("Your device \"{}\" is deprecated", dev);

    let err = "Operating System is not found";
    error!("Fatal: {}", err);

    let buf = 0x12345678;
    debug!("Buffer: 0x{:x}", buf);

    let age = 17;
    trace!("Age: {}", age);

    flag!();
    flag!("i wake up!");
}
```

![example](./assets/images/ex.png)

[![MIT](https://img.shields.io/github/license/Rinrin0413/rogger?color=%23A11D32&style=for-the-badge)](./LICENSE)
