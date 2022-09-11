use proc_macro::TokenStream;
use syn::DeriveInput;

pub(crate) fn impl_command_baseline(derive_input: DeriveInput) -> TokenStream {
    let struct_name = derive_input.ident;
    quote! {
        impl CommandBaseline for #struct_name {
            fn name(&self) -> String {
                String::from(stringify!(#struct_name)).to_lowercase()
            }

            fn description(&self) -> String {
                self.description.to_string()
            }

            fn options(&self) -> &Vec<CadencyCommandOption> {
                self.options.as_ref()
            }
        }
    }
    .into()
}
