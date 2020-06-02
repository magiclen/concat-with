/*!
# Concat-With

While the `concat!` macro in `std` is useful to create a static string slice (`&'static str`) from literals, it cannot set a separator for those. This crate therefore provides another `concat!` macro to deal with that situation.

```rust
#[macro_use] extern crate concat_with;

assert_eq!("test10btrue", concat!("test", 10, 'b', true));
assert_eq!("test, 10, b, true", concat!(with ", ", "test", 10, 'b', true));
assert_eq!("test\n10\nb\ntrue", concat_line!("test", 10, 'b', true));
```
*/

/// Concatenates literals into a static string slice.
///
/// This macro is like the `concat!` macro in `std`, but can also be used to concatenate literals separated by a specific literal.
///
/// ```rust
/// #[macro_use] extern crate concat_with;
///
/// assert_eq!("test10btrue", concat!("test", 10, 'b', true));
/// assert_eq!("test, 10, b, true", concat!(with ", ", "test", 10, 'b', true));
/// assert_eq!("test\n10\nb\ntrue", concat_line!("test", 10, 'b', true));
/// ```
#[macro_export]
macro_rules! concat {
    ($(,)*) => {
        ""
    };
    ( $e:expr $(,)*) => {
        ::std::concat!($e)
    };
    ( $e:expr $(, $es:expr)+ $(,)*) => {
        ::std::concat!( $e $(, $es)+ )
    };
    (with $w:expr $(,)*) => {
        ""
    };
    (with $w:expr, $e:expr $(,)*) => {
        ::std::concat!($e)
    };
    (with $w:expr, $e:expr $(, $es:expr)+ $(,)*) => {
        ::std::concat!( $e $(, $w, $es)+ )
    };
}

/// Concatenates literals into a static string slice separated by a line break, `\n`.
///
/// ```rust
/// #[macro_use] extern crate concat_with;
///
/// assert_eq!("test\n10\nb\ntrue", concat_line!("test", 10, 'b', true));
/// ```
#[macro_export]
macro_rules! concat_line {
    ($($e:expr),* $(,)*) => {
        $crate::concat!(with "\n" $(, $e)*)
    };
}
