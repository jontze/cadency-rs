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
    use cadency_core::CadencyCommandOption;

    #[test]
    fn impl_commandbaseline_trait_with_macro() {
        #[derive(cadency_codegen::CommandBaseline)]
        struct Test {
            options: Vec<CadencyCommandOption>,
        }
        assert!(true)
    }

    #[test]
    fn return_lowercase_struct_name_as_name() {
        #[derive(cadency_codegen::CommandBaseline)]
        #[description = "123"]
        struct Test {
            options: Vec<CadencyCommandOption>,
        }
        let test = Test {
            options: Vec::new(),
        };
        let name: String = test.name();
        assert_eq!(name, "test", "Test command name to be lowercase {name}")
    }

    #[test]
    fn return_attribute_name_as_struct_name() {
        #[derive(cadency_codegen::CommandBaseline)]
        #[name = "my_test"]
        #[description = "123"]
        struct Test {
            options: Vec<CadencyCommandOption>,
        }
        let test = Test {
            options: Vec::new(),
        };
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
        struct Test {
            options: Vec<CadencyCommandOption>,
        }
        let test = Test {
            options: Vec::new(),
        };
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
        struct Test {
            options: Vec<CadencyCommandOption>,
        }
        let test = Test {
            options: Vec::new(),
        };
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
        struct Test {
            options: Vec<CadencyCommandOption>,
        }
        let test = Test {
            options: Vec::new(),
        };
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
        struct Test {
            options: Vec<CadencyCommandOption>,
        }
        let test = Test {
            options: Vec::new(),
        };
        assert!(test.deferred(), "Test command should be deferred")
    }
}
