//! The Birb CLI
//!
//! ```ignore
//! cargo install --path crates/birb-cli --force
//! bb --help
//! ```

#![deny(missing_docs)]

#[macro_use]
extern crate structopt;
#[macro_use]
extern crate failure;

mod bash_completion;
mod deploy;
mod docker;
mod bb_filesystem;
mod update;

use crate::bash_completion::BashCompletionGenerator;
use crate::deploy::Deploy;
use crate::docker::Docker;
use crate::bb_filesystem::{cargo_toml_version, bb_dot_dir};
use crate::update::Update;
use colored::*;
use std::fs::DirBuilder;
use std::iter::repeat;
use std::process::Command;
use structopt::StructOpt;

/// The Birb Eng CLI
#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub enum Bb {
    /// Interact with docker
    #[structopt(name = "docker")]
    Docker(Docker),
    /// Used to deploy different applications or services
    #[structopt(name = "deploy")]
    Deploy(Deploy),
    /// Generate the file that powers autocompleting the `bb` command in
    /// your bash shell.
    #[structopt(name = "generate-bash-completions")]
    GenerateBashCompletions(BashCompletionGenerator),
    /// Update your Birb CLI to the latest version
    #[structopt(name = "update")]
    Update(Update),
}

/// Used to create a Birb CLI subcommand
///
/// ```sh,ignore
/// # An example of running a subcommand in your terminal
///
/// bb help # bb [subcommand].. in this case `help` is the subcommand.
///
/// bb deploy plan # deploy and plan are both subcommands (subcommands can nest)
/// ```
pub trait Subcommand
where
    Self: 'static,
{
    /// Run a subcommand in the Birb CLI
    fn run(&self) -> Result<(), failure::Error>;
}

/// Run the Birb CLI program, typically via the `bb` command
/// in your shell.
pub fn run() -> Result<(), failure::Error> {
    let bb = Bb::from_args();

    // Create the `.bb` directory where the CLI stores information
    let bb_dot = bb_dot_dir();
    DirBuilder::new().recursive(true).create(&bb_dot).unwrap();

    let subcmd: Box<dyn Subcommand> = match bb {
        Bb::GenerateBashCompletions(bash_completion_generator) => {
            boxed_cmd(bash_completion_generator)
        }
        Bb::Update(update) => boxed_cmd(update),
        Bb::Docker(docker) => boxed_cmd(docker),
        Bb::Deploy(deploy) => boxed_cmd(deploy),
    };

    let result = subcmd.run();

    maybe_print_new_version_available_message();

    result
}

/// Wrap anything that implements Subcommand in a Box
fn boxed_cmd<S: Subcommand + 'static>(subcmd: S) -> Box<dyn Subcommand> {
    Box::new(subcmd) as Box<dyn Subcommand>
}

/// Run a command in a bash shell
///
/// ```ignore
/// run_in_bash("echo 'hello world'");
/// run_in_bash("docker-compose exec app bash");
/// run_in_bash("node ./bin/some-script.js");
/// ```
fn run_str_in_bash(bash_str: &str) -> Result<std::process::ExitStatus, std::io::Error> {
    Command::new("bash")
        .arg("-c")
        .args(&[&bash_str])
        .spawn()
        .unwrap()
        .wait()
}

/// Print a message if there's a new version of the command-line available.
fn maybe_print_new_version_available_message() {
    let compiled_version = env!("CARGO_PKG_VERSION");

    let compiled_version_pieces: Vec<u16> = compiled_version
        .split(".")
        .map(|piece| piece.parse().unwrap())
        .collect();
    let cargo_toml_version_pieces: Vec<u16> = cargo_toml_version()
        .split(".")
        .map(|piece| piece.parse().unwrap())
        .collect();

    // Make sure that each major/minor/patch in our cargo toml is greater than or equal to
    // what we compiled with.
    // If this is true, and the versions are different, we know that there is a new, greater version
    // available
    let version_gteq = compiled_version_pieces
        .iter()
        .zip(cargo_toml_version_pieces)
        .all(|(compiled, toml)| toml >= *compiled);

    if version_gteq && cargo_toml_version() != compiled_version {
        println!(
            "{}",
            update_message(compiled_version, &cargo_toml_version())
        );
    }
}

// The out of date message when a user should update their command line version.
// ╭───────────────────────────────────────────╮
// │                                           │
// │      Update available 1.1.1 → 1.1.3       │
// │                                           │
// │      Run bb update to update version      │
// │                                           │
// ╰───────────────────────────────────────────╯
fn update_message(old: &str, new: &str) -> String {
    let pipe = "│";
    let space_before_pipe = 17 - old.len() - new.len();

    let space_before_pipe = repeat(" ").take(space_before_pipe).collect::<String>();

    format!(r#"
{top}
{p}                                           {p}
{p}      Update available {old} → {new}{space}{p}
{p}                                           {p}
{p}      Run {bb_update} to update version      {p}
{p}                                           {p}
{bottom}
    "#, top = "╭───────────────────────────────────────────╮".yellow(),
        p = pipe.yellow(),
        old = old.red(),
        new = new.green(),
        space = space_before_pipe,
        bb_update = "bb update".blue(),
        bottom = "╰───────────────────────────────────────────╯".yellow()
    )
}
