// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! For use with defensive programming where context specific defaults are needed.
//!
//! While using an [`Option`] is preferrably in most circumstances there are situations where a function call doesn't return an
//! [`Option`] and the [`Default`] of a type isn't helpful either.
//!
//! [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
//! [`Default`]: https://doc.rust-lang.org/std/default/trait.Default.html
//! [`str`]: https://doc.rust-lang.org/std/primitive.str.html
//! [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
//! [`String::is_empty()`]: https://doc.rust-lang.org/nightly/alloc/string/struct.String.html#method.is_empty
//! [`str::is_empty()`]: https://doc.rust-lang.org/std/primitive.str.html#method.is_empty
//!
//! # Examples
//!
//! Converting
//! ```
//! let foo = {
//!    let b = bar();
//!    if b.is_empty() {
//!        Bar {
//!            // set the default values for your context here
//!        }
//!    } else {
//!        b
//!    }
//! };
//! ```
//! into
//!
//! ```
//! let foo = bar().if_empty(Bar { /* ... */ });
//! ```
//!
//! # Implementation
//!
//! In this example we're using the obvious `is_empty()` function for `Foo` but we could also
//! do more elaborate checks.
//!
//! ```
//! struct Foo {
//!    val: bool,
//! }
//!
//! impl IfEmpty for Foo {
//!    fn if_empty(self, value: Foo) -> Foo {
//!        if self.is_empty() {
//!            value
//!        } else {
//!            self
//!        }
//!    }
//! }
//! ```

/// For checking IfEmpty on value semantics
pub trait IfEmpty {
    /// Returns `val` if the `self` is empty
    fn if_empty(self, val: Self) -> Self;
}

/// For checking IfEmpty on borrowed objects
pub trait IfEmptyBorrowed {
    /// Return `val` if `self` is empty
    fn if_empty<'a>(&'a self, val: &'a Self) -> &'a Self;
}

/// Implementation of `IfEmptyBorrowed` for [`str`]
impl IfEmptyBorrowed for str {
    /// Returns `input` if [`str::is_empty()`] returns true.
    /// Otherwise `self` is returned.
    fn if_empty<'a>(&'a self, input: &'a Self) -> &'a Self {
        if self.is_empty() {
            input
        } else {
            self
        }
    }
}

/// Implementation of `IfEmpty` for [`String`]
impl IfEmpty for String {
    /// Returns `input` if [`String::is_empty()`] returns true.
    /// Otherwise `self` is returned.
    fn if_empty(self, input: Self) -> Self {
        if self.is_empty() {
            input
        } else {
            self
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{IfEmpty, IfEmptyBorrowed};

    #[test]
    fn string() {
        let string = String::default();
        assert!(string.is_empty());
        let replacement = "text".to_string();
        let replaced = string.if_empty(replacement.clone());
        assert!(!replaced.is_empty());
        assert_eq!(replacement, replaced);

        let string = "not empty".to_string();
        assert!(!string.is_empty());
        assert_eq!("not empty", string.if_empty("should not be returned".to_string()));
    }
    #[test]
    fn str() {
        let string: &str = "";
        assert!(string.is_empty());
        let replacement = "text";
        let replaced = string.if_empty(replacement);
        assert!(!replaced.is_empty());
        assert_eq!(replacement, replaced);

        let string: &str = "not empty";
        assert!(!string.is_empty());
        assert_eq!("not empty", string.if_empty("should not be returned"));
    }
    #[test]
    fn custom() {
        struct Fake {
            value: bool,
        }

        impl IfEmpty for Fake {
            fn if_empty(self, value: Self) -> Self {
                if self.value {
                    self
                } else {
                    value
                }
            }
        }

        let f = Fake {
            value: false,
        };
        assert!(
            f.if_empty(Fake {
                value: true
            })
            .value
        );

        let f = Fake {
            value: true,
        };
        assert!(
            f.if_empty(Fake {
                value: false
            })
            .value
        );
    }
}
