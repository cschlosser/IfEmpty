use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(IfEmpty)]
pub fn if_empty(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);

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
