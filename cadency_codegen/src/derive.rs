use proc_macro::TokenStream;
use syn::{DeriveInput, Lit, Meta, NestedMeta};

pub(crate) fn impl_command_baseline(derive_input: DeriveInput) -> TokenStream {
    let struct_name = derive_input.ident;
    let mut command_name = struct_name.to_string().to_lowercase();
    let mut description = "".to_string();
    let mut deferred = false;
    let mut arguments: Vec<(String, String, String, bool)> = vec![];
    for attr in derive_input.attrs.iter() {
        let attr_meta = attr.parse_meta().unwrap();
        match attr_meta {
            Meta::NameValue(derive_attr) => {
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
            Meta::List(derive_attr_list) => {
                if derive_attr_list
                    .path
                    .get_ident()
                    .unwrap()
                    .to_string()
                    .as_str()
                    == "argument"
                {
                    let mut name: Option<String> = None;
                    let mut description: Option<String> = None;
                    let mut kind: Option<String> = None;
                    let mut required = true;

                    for arguments_attr_meta in derive_attr_list.nested.iter() {
                        if let NestedMeta::Meta(Meta::NameValue(argument_item)) =
                            arguments_attr_meta
                        {
                            match argument_item.path.get_ident().unwrap().to_string().as_str() {
                                    "name" => {
                                        if let Lit::Str(argument_name) = argument_item.lit.to_owned() {
                                            name = Some(argument_name.value());
                                        } else {
                                            return syn::Error::new(argument_item.lit.span(), "Name must be a string").to_compile_error().into()
                                        }
                                    }
                                    "description" => {
                                        if let Lit::Str(argument_description) = argument_item.lit.to_owned() {
                                            description = Some(argument_description.value());
                                        } else {
                                            return syn::Error::new(argument_item.lit.span(), "Description must be a string").to_compile_error().into()
                                        }
                                    }
                                    "kind" => {
                                        if let Lit::Str(argument_kind) = argument_item.lit.to_owned() {
                                            kind = Some(argument_kind.value());
                                        } else {
                                            return syn::Error::new(argument_item.lit.span(), "Kind must be a string").to_compile_error().into()
                                        }
                                    }
                                    "required" => {
                                        if let Lit::Bool(argument_required) = argument_item.lit.to_owned() {
                                            required = argument_required.value();
                                        }
                                    }
                                    _ => {
                                        return syn::Error::new(argument_item.path.get_ident().unwrap().span(), "Only 'name', 'description', 'kind' and 'required' are supported")
                                            .to_compile_error()
                                            .into()
                                    }
                                }
                        }
                    }
                    match (name, description, kind) {
                        (Some(name), Some(description), Some(kind)) => {
                            arguments.push((name, description, kind, required));
                        }
                        _ => {
                            return syn::Error::new(
                                derive_attr_list.path.get_ident().unwrap().span(),
                                "You need to specify at least 'name', 'description' and 'kind'",
                            )
                            .to_compile_error()
                            .into();
                        }
                    }
                }
            }
            _ => (),
        }
    }
    let argument_tokens = arguments.iter().map(|(name, description, kind, required)| {
        let kind_token: proc_macro2::TokenStream = kind.parse().unwrap();
        quote! {
            __CadencyCommandOption {
                name: #name,
                description: #description,
                kind: __CommandOptionType::#kind_token,
                required: #required
            }
        }
    });
    quote! {
        use cadency_core::{CadencyCommandBaseline as __CadencyCommandBaseline, CadencyCommandOption as __CadencyCommandOption};
        use serenity::model::application::command::CommandOptionType as __CommandOptionType;
        impl __CadencyCommandBaseline for #struct_name {
            fn name(&self) -> String {
                String::from(#command_name)
            }

            fn description(&self) -> String {
                String::from(#description)
            }

            fn deferred(&self) -> bool {
                #deferred
            }

            fn options(&self) -> Vec<__CadencyCommandOption> {
                vec![#(#argument_tokens),*]
            }
        }
    }
    .into()
}
