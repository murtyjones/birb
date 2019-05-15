//! The Birb CLI
//!
//! ```ignore
//! cargo install --path crates/birb-cli --force
//! bb --help
//! ```

#![deny(missing_docs)]

#[macro_use]
extern crate structopt;
extern crate failure;

mod aws;
mod bash_completion;
mod bb_filesystem;
mod build;
mod docker;
mod migrate;
mod plan;
mod push;
mod seed;
mod ssh;
mod test;
mod update;
mod watch;

use crate::aws::Aws;
use crate::bash_completion::BashCompletionGenerator;
use crate::bb_filesystem::{bb_dot_dir, cargo_toml_version};
use crate::build::Build;
use crate::docker::Docker;
use crate::migrate::Migrate;
use crate::plan::Plan;
use crate::push::Push;
use crate::seed::Seed;
use crate::ssh::Ssh;
use crate::test::Test;
use crate::update::Update;
use crate::watch::Watch;
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
    /// Used to plan infrastructure changes
    #[structopt(name = "plan")]
    Plan(Plan),
    /// Used to deploy different applications or services
    #[structopt(name = "aws")]
    Aws(Aws),
    /// Used to watch for changes while developing
    #[structopt(name = "watch")]
    Watch(Watch),
    /// Used to build application binaries
    #[structopt(name = "build")]
    Build(Build),
    /// Generate the file that powers autocompleting the `bb` command in
    /// your bash shell.
    #[structopt(name = "generate-bash-completions")]
    GenerateBashCompletions(BashCompletionGenerator),
    /// Update your Birb CLI to the latest version
    #[structopt(name = "update")]
    Update(Update),
    /// Used to SSH into the Bastion for RDS access
    #[structopt(name = "ssh")]
    Ssh(Ssh),
    /// Apply migrations
    #[structopt(name = "migrate")]
    Migrate(Migrate),
    /// Seed DB
    #[structopt(name = "seed")]
    Seed(Seed),
    /// Push images to ECR
    #[structopt(name = "push")]
    Push(Push),
    /// Run tests
    #[structopt(name = "test")]
    Test(Test),
}

/// Used to create a Birb CLI subcommand
///
/// ```sh,ignore
/// # An example of running a subcommand in your terminal
///
/// bb help # bb [subcommand].. in this case `help` is the subcommand.
///
/// bb aws plan # `deploy` and `plan` are both subcommands (subcommands can nest)
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
        Bb::Build(build) => boxed_cmd(build),
        Bb::Docker(docker) => boxed_cmd(docker),
        Bb::Plan(plan) => boxed_cmd(plan),
        Bb::Aws(aws) => boxed_cmd(aws),
        Bb::Watch(watch) => boxed_cmd(watch),
        Bb::Ssh(ssh) => boxed_cmd(ssh),
        Bb::Migrate(migrate) => boxed_cmd(migrate),
        Bb::Seed(seed) => boxed_cmd(seed),
        Bb::Push(push) => boxed_cmd(push),
        Bb::Test(test) => boxed_cmd(test),
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
        .expect(format!("{} command failed to start", bash_str).as_str())
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

    format!(
        r#"
{top}
{p}                                           {p}
{p}      Update available {old} → {new}{space}{p}
{p}                                           {p}
{p}      Run {bb_update} to update version      {p}
{p}                                           {p}
{bottom}
    "#,
        top = "╭───────────────────────────────────────────╮"
            .yellow(),
        p = pipe.yellow(),
        old = old.red(),
        new = new.green(),
        space = space_before_pipe,
        bb_update = "bb update".blue(),
        bottom = "╰───────────────────────────────────────────╯"
            .yellow()
    )
}
