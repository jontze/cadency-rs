#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(CommandBaseline)]
pub fn derive_command_baseline(input_item: TokenStream) -> TokenStream {
    // Parse token stream into rust syntax tree
    let tree: DeriveInput = parse_macro_input!(input_item as DeriveInput);
    // Implement command trait
    let struct_name = tree.ident;
    quote! {
        impl CommandBaseline for #struct_name {
            fn name(&self) -> String {
                String::from(stringify!(#struct_name)).to_lowercase()
            }
        }
    }
    .into()
}
