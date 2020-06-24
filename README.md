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

Prefixes and suffixes can also be added.

```rust
#[macro_use] extern crate concat_with;

assert_eq!("Someone says: Hello.\nSomeone says: Nice to meet you!", concat_line!(prefix "Someone says: ", "Hello.", "Nice to meet you!"));
```

## Create Your Own Macros

The `concat_impl!` macro can be used to create your own macros like `concat_line!` which concatenates literals separated by a specific literal.

```rust
#[macro_use] extern crate concat_with;

concat_impl! {
    #[macro_export]
    /// Concatenates literals into a static string slice separated by a comma, `,`. Prefixes and suffixes can also be added.
    concat_with_comma => ", ",
    #[macro_export]
    /// Concatenates literals into a static string slice separated by a colon, `:`. Prefixes and suffixes can also be added.
    concat_with_colon => ':',
}

assert_eq!("test, 10, b, true", concat_with_comma!("test", 10, 'b', true));
assert_eq!("test:10:b:true", concat_with_colon!("test", 10, 'b', true));
```

## Crates.io

https://crates.io/crates/concat-with

## Documentation

https://docs.rs/concat-with

## License

[MIT](LICENSE)
