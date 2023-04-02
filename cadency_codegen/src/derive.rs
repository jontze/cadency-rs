use proc_macro::TokenStream;
use syn::{punctuated::Punctuated, spanned::Spanned, DeriveInput, Expr, Lit, Meta, Token};

use crate::{argument::Argument, command::Command};

pub(crate) fn impl_command_baseline(derive_input: DeriveInput) -> TokenStream {
    let struct_name = derive_input.ident;
    let mut command = Command::new(struct_name.to_string().to_lowercase());
    for attr in derive_input.attrs.iter() {
        match attr.meta.to_owned() {
            Meta::NameValue(derive_attr) => {
                match derive_attr.path.get_ident().unwrap().to_string().as_str() {
                    // #[name = "name"]
                    "name" => {
                        if let Expr::Lit(name_lit) = derive_attr.value {
                            if let Lit::Str(name_lit) = name_lit.lit {
                                command.name(name_lit.value());
                            } else {
                                return syn::Error::new(
                                    name_lit.lit.span(),
                                    "'name' attribute must be a string",
                                )
                                .to_compile_error()
                                .into();
                            }
                        }
                    }
                    // #[description = "description"]
                    "description" => {
                        if let Expr::Lit(description_lit) = derive_attr.value {
                            if let Lit::Str(description_lit) = description_lit.lit {
                                command.description(description_lit.value());
                            } else {
                                return syn::Error::new(
                                    description_lit.lit.span(),
                                    "'description' attribute must be a string",
                                )
                                .to_compile_error()
                                .into();
                            }
                        }
                    }
                    // #[deferred = true]
                    "deferred" => {
                        if let Expr::Lit(deferred_lit) = derive_attr.value {
                            if let Lit::Bool(deferred_lit) = deferred_lit.lit {
                                if deferred_lit.value {
                                    command.is_deferred();
                                }
                            } else {
                                return syn::Error::new(
                                    deferred_lit.lit.span(),
                                    "'deferred' attribute must be a bool",
                                )
                                .to_compile_error()
                                .into();
                            }
                        }
                    }
                    _ => (),
                }
            }
            Meta::List(derive_attr_list) => {
                // #[argument(..., ...)]
                if derive_attr_list.path.is_ident("argument") {
                    let mut name: Option<String> = None;
                    let mut description: Option<String> = None;
                    let mut kind: Option<String> = None;
                    let mut required = true;

                    let nested = attr
                        .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                        .unwrap();

                    for meta in nested {
                        match meta {
                            Meta::NameValue(name_value_arg) => {
                                match name_value_arg.path.get_ident().unwrap().to_string().as_str() {
                                    // #[argument(name = "name")]
                                    "name" => {
                                        if let Expr::Lit(argument_name_lit) = name_value_arg.value {
                                            if let Lit::Str(argument_name_lit) = argument_name_lit.lit {
                                                name = Some(argument_name_lit.value());
                                            } else {
                                                return syn::Error::new(
                                                    argument_name_lit.lit.span(),
                                                    "Name must be a string",
                                                )
                                                .to_compile_error()
                                                .into();
                                            }
                                        }
                                    }
                                    // #[argument(description = "description")]
                                    "description" => {
                                        if let Expr::Lit(argument_description_lit) = name_value_arg.value {
                                            if let Lit::Str(argument_description_lit) = argument_description_lit.lit {
                                                description = Some(argument_description_lit.value());
                                            } else {
                                                return syn::Error::new(
                                                    argument_description_lit.lit.span(),
                                                    "Description must be a string",
                                                )
                                                .to_compile_error()
                                                .into();
                                            }
                                        }
                                    }
                                    // #[argument(kind = "kind")]
                                    "kind" => {
                                        if let Expr::Lit(argument_kind_lit) = name_value_arg.value {
                                            if let Lit::Str(argument_kind_lit) = argument_kind_lit.lit {
                                                kind = Some(argument_kind_lit.value());
                                            } else {
                                                return syn::Error::new(
                                                    argument_kind_lit.lit.span(),
                                                    "Kind must be a string",
                                                )
                                                .to_compile_error()
                                                .into();
                                            }
                                        }
                                    }
                                    // #[argument(required = true)]
                                    "required" => {
                                        if let Expr::Lit(argument_required_lit) = name_value_arg.value {
                                            if let Lit::Bool(argument_required_lit) = argument_required_lit.lit {
                                                required = argument_required_lit.value;
                                            } else {
                                                return syn::Error::new(
                                                    argument_required_lit.lit.span(),
                                                    "Required must be a bool",
                                                )
                                                .to_compile_error()
                                                .into();
                                            }
                                        }
                                    }
                                    _ => {
                                        return syn::Error::new(name_value_arg.path.get_ident().unwrap().span(), "Only 'name', 'description', 'kind' and 'required' are supported")
                                            .to_compile_error()
                                            .into()
                                    }
                                }
                            }
                            Meta::List(_) => {}
                            Meta::Path(_) => {}
                        }
                    }
                    if let (Some(name), Some(description), Some(kind)) = (name, description, kind) {
                        let mut argument = Argument::new(name, description, kind);
                        if !required {
                            argument.is_optional();
                        }
                        command.add_argument(argument);
                    } else {
                        return syn::Error::new(
                            derive_attr_list.path.get_ident().span(),
                            "All arguments must have a name, description and kind",
                        )
                        .to_compile_error()
                        .into();
                    }
                }
            }
            _ => (),
        }
    }
    let argument_tokens = command.arguments.iter().map(|arg| {
        let name = &arg.name;
        let description = &arg.description;
        let kind_token: proc_macro2::TokenStream = arg.kind.parse().unwrap();
        let required = arg.required;
        quote! {
            __CadencyCommandOption {
                name: #name,
                description: #description,
                kind: __CommandOptionType::#kind_token,
                required: #required
            }
        }
    });

    let command_name = &command.name;
    let description = &command.description;
    let deferred = command.deferred;
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
