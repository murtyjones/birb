use crate::bash_completion::BashCompletionGenerator;
use crate::bb_filesystem::bb_cli_dir;
use crate::{run_str_in_bash, Subcommand};

#[derive(Debug, StructOpt)]
pub enum Migrate {
    /// Perform "up" migrations
    #[structopt(name = "up")]
    Up(Up),
}

#[derive(Debug, StructOpt)]
pub struct Up {
    /// the environment to run against
    #[structopt(short = "-e", long = "--env")]
    env: String, // TODO only accept valid environment values (e.g. prod, local)
}

impl Subcommand for Migrate {
    fn run(&self) -> Result<(), failure::Error> {
        match self {
            Migrate::Up(up) => {
                let cmd = format!(
                    "./scripts/migrate.sh {} up", up.env
                );
                run_str_in_bash(cmd.as_str()).unwrap();
                Ok(())
            }
        }
    }
}
