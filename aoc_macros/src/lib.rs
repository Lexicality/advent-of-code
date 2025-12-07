// Copyright (c) 2025 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use core::panic;

use proc_macro::TokenStream;

use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(VoidState, attributes(void))]
pub fn derive_void(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    let data = match input.data {
        syn::Data::Enum(data_enum) => data_enum,
        _ => panic!("This derive macro only works on enums!"),
    };
    let void = data
        .variants
        .iter()
        .find(|variant| {
            variant
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("void"))
        })
        .expect("A variant must be marked with #[void]");

    let enum_ident = input.ident;
    let variant_ident = &void.ident;

    quote! {
        impl Default for #enum_ident {
            fn default() -> Self {
                Self::#variant_ident
            }
        }
        impl crate::VoidState for #enum_ident {
            fn is_void(&self) -> bool {
                matches!(self, Self::#variant_ident)
            }
        }
    }
    .into()
}
