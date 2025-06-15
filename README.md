# statuses

[![Crates.io](https://img.shields.io/crates/v/statuses.svg)](https://crates.io/crates/statuses)
[![Documentation](https://docs.rs/statuses/badge.svg)](https://docs.rs/statuses)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

> HTTP status code utility for Rust — simple and lightweight mapping between HTTP status codes and their standard messages.

---

## Installation

Add `statuses` to your `Cargo.toml`:

```toml
[dependencies]
statuses = "0.2"
```

## Usage

Get message from status code:

```rust
use statuses::message;

fn main() -> Result<(), statuses::StatusError> {
    let msg = message("422")?;
    println!("{}", msg); // Output: Unprocessable Entity
    Ok(())
}
```

Get code from status message:

```rust
use statuses::code;

fn main() -> Result<(), statuses::StatusError> {
    let code = code("Forbidden")?;
    println!("{}", code); // Output: 403
    Ok(())
}
```

## License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.