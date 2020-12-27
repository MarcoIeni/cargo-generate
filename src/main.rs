use anyhow::Result;
use cargo_generate::{generate, Cli};
use structopt::StructOpt;

/// Generate a new Cargo project from a given template
///
/// Right now, only git repositories can be used as templates. Just execute
///
/// ```sh
/// $ cargo generate --git https://github.com/user/template.git --name foo
/// ```
///
/// or
///
/// ```sh
/// $ cargo generate --git https://github.com/user/template.git --name foo
/// ```
///
/// and a new Cargo project called foo will be generated.
///
/// TEMPLATES:
///
/// In templates, the following placeholders can be used:
///
/// - `project-name`: Name of the project, in dash-case
///
/// - `crate_name`: Name of the project, but in a case valid for a Rust
///   identifier, i.e., snake_case
///
/// - `authors`: Author names, taken from usual environment variables (i.e.
///   those which are also used by Cargo and git)
fn main() -> Result<()> {
    let Cli::Generate(args) = Cli::from_args();
    generate(args)?;

    Ok(())
}
