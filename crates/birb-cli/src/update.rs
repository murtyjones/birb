use crate::bash_completion::BashCompletionGenerator;
use crate::bb_filesystem::bb_cli_dir;
use crate::{run_str_in_bash, Subcommand};

#[derive(Debug, StructOpt)]
pub struct Update {}

impl Subcommand for Update {
    fn run(&self) -> Result<(), failure::Error> {
        let reinstall_cli = format!("cargo install -f --path {}", bb_cli_dir());
        run_str_in_bash(&reinstall_cli).expect("Did not cargo install the CLI");

        // Not that we've updated the CLI, update our bash completions
        BashCompletionGenerator {}.run()
    }
}
