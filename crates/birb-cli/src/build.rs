use crate::{run_str_in_bash, Subcommand};

/// Deploy different applications / services
#[derive(Debug, StructOpt)]
pub enum Build {
    /// Build the server
    #[structopt(name = "server")]
    Server,
    /// Deploys whatever change is held by the "plan" file
    #[structopt(name = "edgar")]
    Edgar,
}

fn build_binary(package: &str) {
    let build_command = format!(
        "
        # get base image
        docker pull clux/muslrust:nightly

        # build binary
        scripts/build_binary.sh {}
    ",
        package
    );
    let _result = run_str_in_bash(build_command.as_str());
}

impl Subcommand for Build {
    fn run(&self) -> Result<(), failure::Error> {
        match self {
            Build::Server => {
                build_binary("api");
                Ok(())
            }
            Build::Edgar => {
                build_binary("edgar-worker");
                Ok(())
            }
        }
    }
}
