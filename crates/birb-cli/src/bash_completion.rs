use crate::bb_filesystem::bb_dot_dir;
use crate::Subcommand;
use crate::Bb;
use clap::Shell;
use std::fs::OpenOptions;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct BashCompletionGenerator {}

impl Subcommand for BashCompletionGenerator {
    /// Update the bash completions file with the latest bash completions for the Birb CLI
    fn run(&self) -> Result<(), failure::Error> {
        let mut completions_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&format!("{}/bb-bash-completion.sh", &bb_dot_dir(),))?;
        Bb::clap().gen_completions_to("bb", Shell::Bash, &mut completions_file);
        println!(
            r#"
# Paste this into your terminal to auto complete commands by pressing [Tab]
source $HOME/.bb/bb-bash-completion.sh
"#
        );

        Ok(())
    }
}
