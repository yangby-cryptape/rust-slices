# Rust-Slices

[![License]](#license)
[![Travis CI]](https://travis-ci.com/yangby-cryptape/rust-slices)
[![Crate Badge]](https://crates.io/crates/slices)
[![Crate Doc]](https://docs.rs/slices)

Convert string literals to static unsigned integer slices in compile time.

[License]: https://img.shields.io/badge/License-Apache--2.0%20OR%20MIT-blue.svg
[Travis CI]: https://img.shields.io/travis/com/yangby-cryptape/rust-slices.svg
[Crate Badge]: https://img.shields.io/crates/v/slices.svg
[Crate Doc]: https://docs.rs/slices/badge.svg

## Usage

The input is a hexadecimal string literal with `0x` prefix.
The size of input should be an even number.

And you can use any number of `_` in the string literal to separate it for more readable.

### Examples

```rust
use slices::u8_slice;

const VAL: &[u8] = u8_slice!("0x_1234_5678_9abc_def0");
const NULL: &[u8] = u8_slice!("0x");

fn main () {
    let val = &[0x12u8, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0];
    assert_eq!(VAL, val);
    assert_eq!(NULL, &[]);
}
```

## License

Licensed under either of [Apache License, Version 2.0] or [MIT License], at
your option.

[Apache License, Version 2.0]: LICENSE-APACHE
[MIT License]: LICENSE-MIT
