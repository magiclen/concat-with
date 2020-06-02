Concat With
====================

[![Build Status](https://travis-ci.org/magiclen/concat-with.svg?branch=master)](https://travis-ci.org/magiclen/concat-with)

While the `concat!` macro in `std` is useful to create a static string slice (`&'static str`) from literals, it cannot set a separator for those. This crate therefore provides another `concat!` macro to deal with that situation.

```rust
#[macro_use] extern crate concat_with;

assert_eq!("test10btrue", concat!("test", 10, 'b', true));
assert_eq!("test, 10, b, true", concat!(with ", ", "test", 10, 'b', true));
assert_eq!("test\n10\nb\ntrue", concat_line!("test", 10, 'b', true));
```

## Crates.io

https://crates.io/crates/concat-with

## Documentation

https://docs.rs/concat-with

## License

[MIT](LICENSE)
