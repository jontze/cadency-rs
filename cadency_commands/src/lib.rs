#[macro_use]
extern crate serde;
#[macro_use]
extern crate log;
#[macro_use]
extern crate cadency_codegen;

mod fib;
pub use fib::Fib;
mod slap;
pub use slap::Slap;
mod inspire;
pub use inspire::Inspire;
mod now;
pub use now::Now;
mod pause;
pub use pause::Pause;
mod ping;
pub use ping::Ping;
mod play;
pub use play::Play;
mod resume;
pub use resume::Resume;
mod skip;
pub use skip::Skip;
mod stop;
pub use stop::Stop;
mod tracks;
pub use tracks::Tracks;
mod urban;
pub use urban::Urban;

#[cfg(test)]
mod test {
    #[test]
    fn impl_commandbaseline_trait_with_macro() {
        #[derive(cadency_codegen::CommandBaseline)]
        struct Test {}
        assert!(true)
    }

    #[test]
    fn return_lowercase_struct_name_as_name() {
        #[derive(cadency_codegen::CommandBaseline)]
        #[description = "123"]
        struct Test {}
        let test = Test {};
        let name: String = test.name();
        assert_eq!(name, "test", "Test command name to be lowercase {name}")
    }

    #[test]
    fn return_attribute_name_as_struct_name() {
        #[derive(cadency_codegen::CommandBaseline)]
        #[name = "my_test"]
        #[description = "123"]
        struct Test {}
        let test = Test {};
        let name: String = test.name();
        assert_eq!(
            name, "my_test",
            "Test command name should match with name attribute"
        )
    }

    #[test]
    fn not_return_uppercase_struct_name_as_name() {
        #[derive(cadency_codegen::CommandBaseline)]
        #[description = "123"]
        struct Test {}
        let test = Test {};
        let name: String = test.name();
        assert_ne!(
            name, "Test",
            "Testing that the first char is not uppercase: {name}"
        )
    }

    #[test]
    fn return_attribute_description() {
        #[derive(cadency_codegen::CommandBaseline)]
        #[description = "123"]
        struct Test {}
        let test = Test {};
        assert_eq!(
            test.description(),
            "123",
            "Test command description should match"
        )
    }

    #[test]
    fn return_default_deferred_config() {
        #[derive(cadency_codegen::CommandBaseline)]
        #[description = "123"]
        struct Test {}
        let test = Test {};
        assert_eq!(
            test.deferred(),
            false,
            "Test command should not be deferred"
        )
    }

    #[test]
    fn return_deferred_attribute() {
        #[derive(cadency_codegen::CommandBaseline)]
        #[description = "123"]
        #[deferred = true]
        struct Test {}
        let test = Test {};
        assert!(test.deferred(), "Test command should be deferred")
    }

    #[test]
    fn return_empty_options_by_default() {
        #[derive(cadency_codegen::CommandBaseline)]
        struct Test {}
        let test = Test {};
        assert_eq!(test.options().len(), 0);
    }

    #[test]
    fn return_derived_option() {
        use serenity::model::application::command::CommandOptionType;
        #[derive(cadency_codegen::CommandBaseline)]
        #[argument(
            name = "say",
            description = "Word to say",
            kind = "String",
            required = false
        )]
        struct Test {}
        let test = Test {};
        let arguments = test.options();
        assert_eq!(arguments.len(), 1);
        let argument = arguments.get(0).unwrap();
        assert_eq!(argument.name, "say");
        assert_eq!(argument.description, "Word to say");
        assert_eq!(argument.kind, CommandOptionType::String);
        assert_eq!(argument.required, false);
    }

    #[test]
    fn return_required_option_by_default() {
        #[derive(cadency_codegen::CommandBaseline)]
        #[argument(name = "say", description = "Word to say", kind = "String")]
        struct Test {}
        let test = Test {};
        let arguments = test.options();
        assert_eq!(arguments.len(), 1);
        let argument = arguments.get(0).unwrap();
        assert!(argument.required);
    }

    #[test]
    fn return_multiple_options() {
        use serenity::model::application::command::CommandOptionType;

        #[derive(cadency_codegen::CommandBaseline)]
        #[argument(name = "say", description = "Word to say", kind = "String")]
        #[argument(name = "target", description = "The target user", kind = "User")]
        struct Test {}
        let test = Test {};
        let arguments = test.options();
        assert_eq!(arguments.len(), 2);
        let first_argument = arguments.get(0).unwrap();
        let second_argument = arguments.get(1).unwrap();
        assert_eq!(first_argument.name, "say");
        assert_eq!(first_argument.description, "Word to say");
        assert_eq!(first_argument.kind, CommandOptionType::String);
        assert_eq!(second_argument.name, "target");
        assert_eq!(second_argument.description, "The target user");
        assert_eq!(second_argument.kind, CommandOptionType::User);
    }
}
