pub(crate) mod command {
    use proc_macro::TokenStream;
    use syn::{ItemFn, Stmt};

    fn log_command_usage() -> Result<Stmt, syn::Error> {
        syn::parse(
            quote!(
                debug!("Execute {} command", self.name());
            )
            .into(),
        )
    }

    fn add_start_log(function: &mut ItemFn) {
        let logger = log_command_usage().expect("Failed to parse log statement");
        function.block.stmts.insert(0, logger);
    }

    pub(crate) fn complete_command(mut function: ItemFn) -> TokenStream {
        add_start_log(&mut function);
        quote!(
            #function
        )
        .into()
    }
}
