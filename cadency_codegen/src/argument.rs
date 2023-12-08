use proc_macro2::Ident;
use syn::spanned::Spanned;

pub(crate) struct Argument {
    pub name: String,
    pub description: String,
    pub kind: String,
    pub required: bool,
}

impl Argument {
    pub fn new(name: String, description: String, kind: String) -> Self {
        Self {
            name,
            description,
            kind,
            required: true,
        }
    }

    fn arg_name(&self) -> String {
        self.name.trim().to_lowercase()
    }

    fn kind_token(&self) -> proc_macro2::TokenStream {
        self.kind.parse().unwrap()
    }

    fn kind_ident(&self) -> Ident {
        Ident::new(&self.kind, self.name.span())
    }

    fn rust_type(&self) -> proc_macro2::TokenStream {
        match self.kind.as_str() {
            "Boolean" => quote! { bool },
            "Integer" => quote! { i64 },
            "Number" => quote! { f64 },
            "String" => quote! { String },
            "SubCommand" | "SubCommandGroup" => {
                quote! { Vec<serenity::model::application::CommandDataOption> }
            }
            "Attachment" => quote! { serenity::model::id::AttachmentId },
            "Channel" => quote! { serenity::model::id::ChannelId },
            "Mentionable" => quote! { serenity::model::id::GenericId },
            "Role" => quote! { serenity::model::id::RoleId },
            "User" => quote! { serenity::model::id::UserId },
            "Unknown" => quote! { u8 },
            _ => panic!("Unknown argument kind: {}", self.kind),
        }
    }

    pub fn is_optional(&mut self) {
        self.required = false;
    }

    pub fn to_cadency_command_option(&self) -> proc_macro2::TokenStream {
        let name = self.arg_name();
        let description = &self.description;
        let kind_token = self.kind_token();
        let required = self.required;
        quote! {
            __CadencyCommandOption {
                name: #name,
                description: #description,
                kind: __CommandOptionType::#kind_token,
                required: #required
            }
        }
    }

    pub fn to_getter_fn(&self) -> proc_macro2::TokenStream {
        let arg_kind_ident = self.kind_ident();
        let arg_rust_type = self.rust_type();

        let arg_name = &self.name;
        let fn_name_ident = Ident::new(&format!("arg_{arg_name}"), self.name.span());

        let (fn_return_type, value_unwrap) = if self.required {
            (quote! { #arg_rust_type }, quote! {.unwrap()})
        } else {
            (quote! { Option<#arg_rust_type> }, quote! {})
        };

        // Create a function to extract the argument from the command
        quote! {
            fn #fn_name_ident(
                &self,
                command: &serenity::model::application::CommandInteraction
            ) -> #fn_return_type {
                command
                    .data
                    .options
                    .iter()
                    .find(|option| option.name == #arg_name)
                    .map(|option| option.value.to_owned())
                    .map(|value| {
                        match value {
                            serenity::model::application::CommandDataOptionValue::#arg_kind_ident(value) => value,
                            _ => unreachable!("Incorrect Type"),
                        }
                    })
                    #value_unwrap
            }
        }
    }
}
