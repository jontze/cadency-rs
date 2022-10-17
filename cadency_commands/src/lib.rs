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
            description: String,
            options: Vec<CadencyCommandOption>,
        }
        assert!(true)
    }

    #[test]
    fn return_lowercase_struct_name_as_name() {
        #[derive(cadency_codegen::CommandBaseline)]
        struct Test {
            description: String,
            options: Vec<CadencyCommandOption>,
        }
        let test = Test {
            description: "123".to_string(),
            options: Vec::new(),
        };
        let name: String = test.name();
        assert_eq!(name, "test", "Test command name to be lowercase {name}")
    }

    #[test]
    fn not_return_uppercase_struct_name_as_name() {
        #[derive(cadency_codegen::CommandBaseline)]
        struct Test {
            description: String,
            options: Vec<CadencyCommandOption>,
        }
        let test = Test {
            description: "123".to_string(),
            options: Vec::new(),
        };
        let name: String = test.name();
        assert_ne!(
            name, "Test",
            "Testing that the first char is not uppercase: {name}"
        )
    }
}
