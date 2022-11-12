# Satuses

HTTP status utility for Rust.

## Cargo

Put this in your Cargo.toml:

```yaml
[dependencies]
statuses = "0.1"
```

## Usage

### `message(code)`

Returns the status message string for a known HTTP status code.

```rust
fn main() {
    // Unprocessable Entity
    println!("{}", statuses::message("422"));
}
```

### `code(message)`

Returns the status code string for a known HTTP status message.

```rust
fn main() {
    // 403
    println!("{}", statuses::code("Forbidden"));
}