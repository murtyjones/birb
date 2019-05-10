use crate::bash_completion::BashCompletionGenerator;
use crate::bb_filesystem::bb_cli_dir;
use crate::{run_str_in_bash, Subcommand};

/// Deploy different applications / services
#[derive(Debug, StructOpt)]
pub enum Push {
    #[structopt(name = "edgar")]
    Edgar(PushEdgar),
}

impl Subcommand for Push {
    fn run(&self) -> Result<(), failure::Error> {
        match self {
            Push::Edgar(push_edgar) => push_edgar.run(),
        }
    }
}

/// Push the Edgar Worker image to ECR
#[derive(Debug, StructOpt)]
pub struct PushEdgar {}

impl Subcommand for PushEdgar {
    fn run(&self) -> Result<(), failure::Error> {
        run_str_in_bash("./scripts/build_push_edgar_worker.sh").unwrap();
        Ok(())
    }
}
