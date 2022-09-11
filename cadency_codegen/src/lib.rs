#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, ItemFn};

mod attribute;
mod derive;

#[proc_macro_derive(CommandBaseline)]
pub fn derive_command_baseline(input_item: TokenStream) -> TokenStream {
    // Parse token stream into derive syntax tree
    let tree: DeriveInput = parse_macro_input!(input_item as DeriveInput);
    // Implement command trait
    derive::impl_command_baseline(tree)
}

#[proc_macro_attribute]
pub fn command(_: TokenStream, input_item: TokenStream) -> TokenStream {
    // Parse function
    let input_function = parse_macro_input!(input_item as ItemFn);
    // Return modified function
    attribute::command::complete_command(input_function)
}
