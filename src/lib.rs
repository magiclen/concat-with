/*!
# Concat-With

While the `concat!` macro in `std` is useful to create a static string slice (`&'static str`) from literals, it cannot set a separator for those. This crate therefore provides another `concat!` macro to deal with that situation.

```rust
assert_eq!("test10btrue", concat_with::concat!("test", 10, 'b', true));
assert_eq!("test, 10, b, true", concat_with::concat!(with ", ", "test", 10, 'b', true));
assert_eq!("test\n10\nb\ntrue", concat_with::concat_line!("test", 10, 'b', true));
```

Prefixes and suffixes can also be added.

```rust
assert_eq!("Someone says: Hello.\nSomeone says: Nice to meet you!", concat_with::concat_line!(prefix "Someone says: ", "Hello.", "Nice to meet you!"));
```

## Create Your Own Macros

The `concat_impl!` macro can be used to create your own macros like `concat_line!` which concatenates literals separated by a specific literal.

```rust
#[doc(hidden)]
pub use concat_with::{concat, concat_impl}; // re-export `concat!` and `concat_impl!` if your custom macros use `#[macro_export]`

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
*/

/**
Concatenates literals into a static string slice.

This macro is like the `concat!` macro in `std`, but can also be used to concatenate literals separated by a specific literal. Prefixes and suffixes can also be added.

```rust
assert_eq!("test10btrue", concat_with::concat!("test", 10, 'b', true));
assert_eq!("test, 10, b, true", concat_with::concat!(with ", ", "test", 10, 'b', true));
assert_eq!("test\n10\nb\ntrue", concat_with::concat_line!("test", 10, 'b', true));
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
Create macros used for concatenating literals separated by a specific literal.

```rust
concat_with::concat_impl! {
    #[macro_export]
    /// Concatenates literals into a static string slice separated by a comma and a whitespace, `, `. Prefixes and suffixes can also be added.
    concat_with_comma => ", ",
    #[macro_export]
    /// Concatenates literals into a static string slice separated by a colon, `:`. Prefixes and suffixes can also be added.
    concat_with_colon => ':',
}

assert_eq!("test, 10, b, true", concat_with_comma!("test", 10, 'b', true));
assert_eq!("test:10:b:true", concat_with_colon!("test", 10, 'b', true));
```
*/
#[macro_export]
macro_rules! concat_impl {
    (@inner $(#[$attr:meta])* $name:ident, $w: expr, $dollar:tt) => {
        $(#[$attr])*
        macro_rules! $name {
            ($dollar($dollar e:expr),* $dollar(,)*) => {
                $crate::concat!(with $w $dollar(, $dollar e)*)
            };
            (prefix $dollar p:expr $dollar(, $dollar e:expr)* $dollar(,)*) => {
                $crate::concat!(with $w, prefix $dollar p $dollar (, $dollar e)*)
            };
            (suffix $dollar s:expr $dollar(, $e:expr)* $dollar(,)*) => {
                $crate::concat!(with $w, suffix $dollar s $dollar(, $dollar e)*)
            };
            (prefix $dollar p:expr, suffix $dollar s:expr $dollar(, $dollar e:expr)* $dollar(,)*) => {
                $crate::concat!(with $w, prefix $dollar p, suffix $dollar s $dollar(, $dollar e)*)
            };
            (suffix $dollar s:expr, prefix $dollar p:expr $dollar(, $dollar e:expr)* $dollar(,)*) => {
                $crate::concat!(with $w, prefix $dollar p, suffix $dollar s $dollar(, $dollar e)*)
            };
        }
    };
    ($($(#[$attr:meta])* $name:ident => $w:expr),* $(,) *) => {
        $(
            $crate::concat_impl!(@inner $(#[$attr])* $name, $w, $);
        )*
    };
}

concat_impl! {
    #[macro_export]
    /// Concatenates literals into a static string slice separated by a line break, `\n`. Prefixes and suffixes can also be added.
    ///
    /// ```rust
    /// assert_eq!("test\n10\nb\ntrue", concat_with::concat_line!("test", 10, 'b', true));
    /// ```
    /// */
    concat_line => "\n"
}
