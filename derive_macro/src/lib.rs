// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Derive macro complementing [`if_empty`]
//!
//! [`if_empty`]: https://docs.rs/if_empty/
//!
//! This crate provides a derive macro implementing the `if_empty` function if the type has a
//! `is_empty` function.
//!
//! # Examples
//! ```
//! # use if_empty_derive::IfEmpty;
//! #[derive(IfEmpty)]
//! struct Example {
//!     value: String,
//! }
//!
//! impl Example {
//!     fn is_empty(&self) -> bool {
//!         self.value.is_empty()
//!     }
//! }
//!
//! let example = Example {
//!     value: String::new(),
//! };
//!
//! assert!(example.value.is_empty());
//! assert_eq!(example.if_empty(Example {value: "a default".to_string()}).value, "a default");
//! ```

use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Implement `if_empty` on types with `is_empty` functions
///
/// [`if_empty`]: https://docs.rs/if_empty/
/// [`if_empty_derive`]: https://docs.rs/if_empty_derive/
///
/// See [`if_empty`] for usage guidelines and [`if_empty_derive`] for implementation constraints.
#[proc_macro_derive(IfEmpty)]
pub fn if_empty(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, ..
    } = parse_macro_input!(input);

    let output = quote! {
        impl #ident {
            fn if_empty(self, input: Self) -> Self {
                if self.is_empty() {
                    input
                } else {
                    self
                }
            }
        }
    };

    output.into()
}
