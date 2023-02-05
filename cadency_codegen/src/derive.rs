use proc_macro::TokenStream;
use syn::{DeriveInput, Lit, Meta};

pub(crate) fn impl_command_baseline(derive_input: DeriveInput) -> TokenStream {
    let struct_name = derive_input.ident;
    let mut command_name = struct_name.to_string().to_lowercase();
    let mut description = "".to_string();
    let mut deferred = false;
    for attr in derive_input.attrs.iter() {
        let attr_meta = attr.parse_meta().unwrap();
        if let Meta::NameValue(derive_attr) = attr_meta {
            match derive_attr.path.get_ident().unwrap().to_string().as_str() {
                "name" => {
                    if let Lit::Str(name_attr_value) = derive_attr.lit {
                        command_name = name_attr_value.value();
                    } else {
                        return syn::Error::new(
                            derive_attr.lit.span(),
                            "'name' attribute must be a string",
                        )
                        .to_compile_error()
                        .into();
                    }
                }
                "description" => {
                    if let Lit::Str(description_attr_value) = derive_attr.lit {
                        description = description_attr_value.value();
                    } else {
                        return syn::Error::new(
                            derive_attr.lit.span(),
                            "'description' attribute must be a string",
                        )
                        .to_compile_error()
                        .into();
                    }
                }
                "deferred" => {
                    if let Lit::Bool(deferred_attr_value) = derive_attr.lit {
                        deferred = deferred_attr_value.value();
                    } else {
                        return syn::Error::new(
                            derive_attr.lit.span(),
                            "'deferred' attribute must be a bool",
                        )
                        .to_compile_error()
                        .into();
                    }
                }
                _ => (),
            }
        }
    }
    quote! {
        use cadency_core::{self, CadencyCommandBaseline};
        impl cadency_core::CadencyCommandBaseline for #struct_name {
            fn name(&self) -> String {
                String::from(#command_name)
            }

            fn description(&self) -> String {
                String::from(#description)
            }

            fn deferred(&self) -> bool {
                #deferred
            }

            fn options(&self) -> &Vec<CadencyCommandOption> {
                self.options.as_ref()
            }
        }
    }
    .into()
}
