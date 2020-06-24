/*!
# Concat-With

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
*/

/**
Concatenates literals into a static string slice.

This macro is like the `concat!` macro in `std`, but can also be used to concatenate literals separated by a specific literal. Prefixes and suffixes can also be added.

```rust
#[macro_use] extern crate concat_with;

assert_eq!("test10btrue", concat!("test", 10, 'b', true));
assert_eq!("test, 10, b, true", concat!(with ", ", "test", 10, 'b', true));
assert_eq!("test\n10\nb\ntrue", concat_line!("test", 10, 'b', true));
```
*/
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
    (prefix $p:expr $(,)*) => {
        ""
    };
    (prefix $p:expr, $e:expr $(,)*) => {
        ::std::concat!($p, $e)
    };
    (prefix $p:expr, $e:expr $(, $es:expr)+ $(,)*) => {
        ::std::concat!($p, $e $(, $p, $es)+ )
    };
    (suffix $s:expr $(,)*) => {
        ""
    };
    (suffix $s:expr, $e:expr $(,)*) => {
        ::std::concat!($e, $s)
    };
    (suffix $s:expr, $e:expr $(, $es:expr)+ $(,)*) => {
        ::std::concat!($e, $s $(, $es, $s)+ )
    };
    (prefix $p:expr, suffix $s:expr $(,)*) => {
        ""
    };
    (prefix $p:expr, suffix $s:expr, $e:expr $(,)*) => {
        ::std::concat!($p, $e, $s)
    };
    (prefix $p:expr, suffix $s:expr, $e:expr $(, $es:expr)+ $(,)*) => {
        ::std::concat!($p, $e, $s $(, $p, $es, $s)+ )
    };
    (suffix $s:expr, prefix $p:expr $(,)*) => {
        $crate::concat!(prefix $p, suffix $s)
    };
    (suffix $s:expr, prefix $p:expr, $e:expr $(,)*) => {
        $crate::concat!(prefix $p, suffix $s, $e)
    };
    (suffix $s:expr, prefix $p:expr, $e:expr $(, $es:expr)+ $(,)*) => {
        $crate::concat!(prefix $p, suffix $s, $e $(, $es)+)
    };
    (with $w:expr, prefix $p:expr $(,)*) => {
        ""
    };
    (with $w:expr, prefix $p:expr, $e:expr $(,)*) => {
        ::std::concat!($p, $e)
    };
    (with $w:expr, prefix $p:expr, $e:expr $(, $es:expr)+ $(,)*) => {
        ::std::concat!($p, $e $(, $w, $p, $es)+ )
    };
    (prefix $p:expr, with $w:expr $(,)*) => {
        $crate::concat!(with $w, prefix $p)
    };
    (prefix $p:expr, with $w:expr, $e:expr $(,)*) => {
        $crate::concat!(with $w, prefix $p, $e)
    };
    (prefix $p:expr, with $w:expr, $e:expr $(, $es:expr)+ $(,)*) => {
        $crate::concat!(with $w, prefix $p, $e $(, $es)+)
    };
    (with $w:expr, suffix $s:expr $(,)*) => {
        ""
    };
    (with $w:expr, suffix $s:expr, $e:expr $(,)*) => {
        ::std::concat!($e, $s)
    };
    (with $w:expr, suffix $s:expr, $e:expr $(, $es:expr)+ $(,)*) => {
        ::std::concat!($e, $s $(, $w, $es, $s)+ )
    };
    (suffix $s:expr, with $w:expr $(,)*) => {
        $crate::concat!(with $w, suffix $s)
    };
    (suffix $s:expr, with $w:expr, $e:expr $(,)*) => {
        $crate::concat!(with $w, suffix $s, $e)
    };
    (suffix $s:expr, with $w:expr, $e:expr $(, $es:expr)+ $(,)*) => {
        $crate::concat!(with $w, suffix $s, $e $(, $es)+)
    };
    (with $w:expr, prefix $p:expr, suffix $s:expr $(,)*) => {
        ""
    };
    (with $w:expr, prefix $p:expr, suffix $s:expr, $e:expr $(,)*) => {
        ::std::concat!($p, $e, $s)
    };
    (with $w:expr, prefix $p:expr, suffix $s:expr, $e:expr $(, $es:expr)+ $(,)*) => {
        ::std::concat!($p, $e, $s $(, $w, $p, $es, $s)+ )
    };
    (prefix $p:expr, with $w:expr, suffix $s:expr $(,)*) => {
        $crate::concat!(with $w, prefix $p, suffix $s)
    };
    (prefix $p:expr, with $w:expr, suffix $s:expr, $e:expr $(,)*) => {
        $crate::concat!(with $w, prefix $p, suffix $s, $e)
    };
    (prefix $p:expr, with $w:expr, suffix $s:expr, $e:expr $(, $es:expr)+ $(,)*) => {
        $crate::concat!(with $w, prefix $p, suffix $s, $e $(, $es)+)
    };
    (prefix $p:expr, suffix $s:expr, with $w:expr $(,)*) => {
        $crate::concat!(with $w, prefix $p, suffix $s)
    };
    (prefix $p:expr, suffix $s:expr, with $w:expr, $e:expr $(,)*) => {
        $crate::concat!(with $w, prefix $p, suffix $s, $e)
    };
    (prefix $p:expr, suffix $s:expr, with $w:expr, $e:expr $(, $es:expr)+ $(,)*) => {
        $crate::concat!(with $w, prefix $p, suffix $s, $e $(, $es)+)
    };
    (with $w:expr, suffix $s:expr, prefix $p:expr $(,)*) => {
        $crate::concat!(with $w, prefix $p, suffix $s)
    };
    (with $w:expr, suffix $s:expr, prefix $p:expr, $e:expr $(,)*) => {
        $crate::concat!(with $w, prefix $p, suffix $s, $e)
    };
    (with $w:expr, suffix $s:expr, prefix $p:expr, $e:expr $(, $es:expr)+ $(,)*) => {
        $crate::concat!(with $w, prefix $p, suffix $s, $e $(, $es)+)
    };
    (suffix $s:expr, with $w:expr, prefix $p:expr $(,)*) => {
        $crate::concat!(with $w, prefix $p, suffix $s)
    };
    (suffix $s:expr, with $w:expr, prefix $p:expr, $e:expr $(,)*) => {
        $crate::concat!(with $w, prefix $p, suffix $s, $e)
    };
    (suffix $s:expr, with $w:expr, prefix $p:expr, $e:expr $(, $es:expr)+ $(,)*) => {
        $crate::concat!(with $w, prefix $p, suffix $s, $e $(, $es)+)
    };
    (suffix $s:expr, prefix $p:expr, with $w:expr $(,)*) => {
        $crate::concat!(with $w, prefix $p, suffix $s)
    };
    (suffix $s:expr, prefix $p:expr, with $w:expr, $e:expr $(,)*) => {
        $crate::concat!(with $w, prefix $p, suffix $s, $e)
    };
    (suffix $s:expr, prefix $p:expr, with $w:expr, $e:expr $(, $es:expr)+ $(,)*) => {
        $crate::concat!(with $w, prefix $p, suffix $s, $e $(, $es)+)
    };
}

/**
Concatenates literals into a static string slice separated by a line break, `\n`. Prefixes and suffixes can also be added.

```rust
#[macro_use] extern crate concat_with;

assert_eq!("test\n10\nb\ntrue", concat_line!("test", 10, 'b', true));
```
*/
#[macro_export]
macro_rules! concat_line {
    ($($e:expr),* $(,)*) => {
        $crate::concat!(with "\n" $(, $e)*)
    };
    (prefix $p:expr $(, $e:expr)* $(,)*) => {
        $crate::concat!(with "\n", prefix $p $(, $e)*)
    };
    (suffix $s:expr $(, $e:expr)* $(,)*) => {
        $crate::concat!(with "\n", suffix $s $(, $e)*)
    };
    (prefix $p:expr, suffix $s:expr $(, $e:expr)* $(,)*) => {
        $crate::concat!(with "\n", prefix $p, suffix $s $(, $e)*)
    };
    (suffix $s:expr, prefix $p:expr $(, $e:expr)* $(,)*) => {
        $crate::concat!(with "\n", prefix $p, suffix $s $(, $e)*)
    };
}
