/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

pub trait IfEmpty {
    fn if_empty(self, val: Self) -> Self;
}

pub trait IfEmptyBorrowed {
    fn if_empty<'a>(&'a self, val: &'a Self) -> &'a Self;
}

impl IfEmptyBorrowed for str {
    fn if_empty<'a>(&'a self, input: &'a Self) -> &'a Self {
        if self.is_empty() {
            input
        } else {
            self
        }
    }
}

impl IfEmpty for String {
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
    use crate::{ IfEmpty, IfEmptyBorrowed };

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

        let f = Fake { value: false };
        assert!(f.if_empty(Fake {value: true}).value);

        let f = Fake { value: true };
        assert!(f.if_empty(Fake {value: false}).value);

    }
}

