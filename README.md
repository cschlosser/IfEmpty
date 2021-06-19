# IfEmpty

[![CI Status](https://github.com/cschlosser/ifempty/actions/workflows/ci.yml/badge.svg)](https://github.com/cschlosser/IfEmpty/actions/workflows/ci.yml)

A crate to use for defensive programming where context specific defaults are needed.

## Motivation

While using an `Option` is preferrably in most circumstances there are situations where a function call
doesn't return a `Option` and the `Default` of a type isn't helpful either.

You now have the option to wrap this special function call or write something along these lines:

```rust
let foo = {
    let b = bar();
    if b.is_empty() {
        Bar {
            // set the default values for your context here
        }
    } else {
        b
    }
};
```

Using this crate you can reduce this down to:

```rust
let foo = bar().if_empty(Bar { /* ... */ });
```

## Implementing this for your own type

In order to take advantage of this feature you have to implement this behavior for your types:

```rust

use if_empty::*;

struct Foo {
    val: bool,
}

impl IfEmpty for Foo {
    fn if_empty(self, value: Foo) -> Foo {
        if self.is_empty() {
            value
        } else {
            self
        }
    }
}

```

Once [#1](https://github.com/cschlosser/IfEmpty/issues/1) is implemented you can simply use `derive`.

## Provided types

The crate provides this functionality for `String` and `&str`.

