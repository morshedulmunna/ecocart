#![doc = include_str!("../docs/index.md")]
pub mod cmd;
pub mod configs;
pub mod interfaces;
pub mod pkg;
use crate::cmd::root::RootCommand;

pub fn execute() -> Result<(), Box<dyn std::error::Error>> {
    let root_cmd = RootCommand::new();
    root_cmd.run()
}

/// API documentation modules included via rustdoc from the `docs/` directory.
///
/// These modules exist solely to surface the Markdown documentation in
/// `cargo doc` output. They do not contain runtime code.
pub mod api_docs {
    #[doc = include_str!("../docs/auth.md")]
    pub mod auth {}

    #[doc = include_str!("../docs/users.md")]
    pub mod users {}

    #[doc = include_str!("../docs/categories.md")]
    pub mod categories {}

    #[doc = include_str!("../docs/products.md")]
    pub mod products {}

    #[doc = include_str!("../docs/health.md")]
    pub mod health {}

    #[doc = include_str!("../docs/errors.md")]
    pub mod errors {}
}
