use proc_macro::TokenStream;
use syn::{
    punctuated::Punctuated, spanned::Spanned, Attribute, DeriveInput, Expr, Lit, Meta, MetaList,
    MetaNameValue, Token,
};

use crate::{argument::Argument, command::Command};

fn parse_command_args(command: &mut Command, derive_attr: MetaNameValue) -> Result<(), syn::Error> {
    match derive_attr.path.get_ident().unwrap().to_string().as_str() {
        // #[name = "name"]
        "name" => {
            if let Expr::Lit(name_lit) = derive_attr.value {
                if let Lit::Str(name_lit) = name_lit.lit {
                    command.name(name_lit.value());
                } else {
                    return Err(syn::Error::new(
                        name_lit.lit.span(),
                        "'name' attribute must be a string",
                    ));
                }
            }
        }
        // #[description = "description"]
        "description" => {
            if let Expr::Lit(description_lit) = derive_attr.value {
                if let Lit::Str(description_lit) = description_lit.lit {
                    command.description(description_lit.value());
                } else {
                    return Err(syn::Error::new(
                        description_lit.lit.span(),
                        "'description' attribute must be a string",
                    ));
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
                    return Err(syn::Error::new(
                        deferred_lit.lit.span(),
                        "'deferred' attribute must be a bool",
                    ));
                }
            }
        }
        _ => (),
    };
    Ok(())
}

fn parse_arguments(
    command: &mut Command,
    derive_attr_list: &MetaList,
    attr: &Attribute,
) -> Result<(), syn::Error> {
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
                    match name_value_arg
                        .path
                        .get_ident()
                        .unwrap()
                        .to_string()
                        .as_str()
                    {
                        // #[argument(name = "name")]
                        "name" => {
                            if let Expr::Lit(argument_name_lit) = name_value_arg.value {
                                if let Lit::Str(argument_name_lit) = argument_name_lit.lit {
                                    name = Some(argument_name_lit.value());
                                } else {
                                    return Err(syn::Error::new(
                                        argument_name_lit.lit.span(),
                                        "Name must be a string",
                                    ));
                                }
                            }
                        }
                        // #[argument(description = "description")]
                        "description" => {
                            if let Expr::Lit(argument_description_lit) = name_value_arg.value {
                                if let Lit::Str(argument_description_lit) =
                                    argument_description_lit.lit
                                {
                                    description = Some(argument_description_lit.value());
                                } else {
                                    return Err(syn::Error::new(
                                        argument_description_lit.lit.span(),
                                        "Description must be a string",
                                    ));
                                }
                            }
                        }
                        // #[argument(kind = "kind")]
                        "kind" => {
                            if let Expr::Lit(argument_kind_lit) = name_value_arg.value {
                                if let Lit::Str(argument_kind_lit) = argument_kind_lit.lit {
                                    kind = Some(argument_kind_lit.value());
                                } else {
                                    return Err(syn::Error::new(
                                        argument_kind_lit.lit.span(),
                                        "Kind must be a string",
                                    ));
                                }
                            }
                        }
                        // #[argument(required = true)]
                        "required" => {
                            if let Expr::Lit(argument_required_lit) = name_value_arg.value {
                                if let Lit::Bool(argument_required_lit) = argument_required_lit.lit
                                {
                                    required = argument_required_lit.value;
                                } else {
                                    return Err(syn::Error::new(
                                        argument_required_lit.lit.span(),
                                        "Required must be a bool",
                                    ));
                                }
                            }
                        }
                        _ => {
                            return Err(syn::Error::new(
                                name_value_arg.path.get_ident().unwrap().span(),
                                "Only 'name', 'description', 'kind' and 'required' are supported",
                            ));
                        }
                    }
                }
                Meta::List(_) | Meta::Path(_) => {}
            }
        }
        if let (Some(name), Some(description), Some(kind)) = (name, description, kind) {
            let mut argument = Argument::new(name, description, kind);
            if !required {
                argument.is_optional();
            }
            command.add_argument(argument);
        } else {
            return Err(syn::Error::new(
                derive_attr_list.path.get_ident().span(),
                "All arguments must have a name, description and kind",
            ));
        }
    }
    Ok(())
}

pub(crate) fn impl_command_baseline(derive_input: DeriveInput) -> TokenStream {
    let struct_name = derive_input.ident;
    let mut command = Command::new(struct_name.to_string().to_lowercase());
    for attr in &derive_input.attrs {
        if let Err(err) = match attr.meta.clone() {
            Meta::NameValue(derive_attr) => parse_command_args(&mut command, derive_attr),
            Meta::List(derive_attr_list) => parse_arguments(&mut command, &derive_attr_list, attr),
            Meta::Path(_) => Ok(()),
        } {
            // If there are any parsing errors, throw them back to the compiler
            return err.to_compile_error().into();
        }
    }
    let cadency_command_option_tokens: Vec<proc_macro2::TokenStream> = command
        .arguments
        .iter()
        .map(Argument::to_cadency_command_option)
        .collect();

    let argument_functions: Vec<proc_macro2::TokenStream> = command
        .arguments
        .iter()
        .map(Argument::to_getter_fn)
        .collect();

    let command_name = &command.name;
    let description = &command.description;
    let deferred = command.deferred;

    // Implement the CadencyCommandBaseline trait for the struct
    quote! {
        use cadency_core::{CadencyCommandBaseline as __CadencyCommandBaseline, CadencyCommandOption as __CadencyCommandOption};
        use serenity::model::application::CommandOptionType as __CommandOptionType;

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
                vec![#(#cadency_command_option_tokens),*]
            }
        }

        impl #struct_name {
            #(#argument_functions)*
        }


    }
    .into()
}
