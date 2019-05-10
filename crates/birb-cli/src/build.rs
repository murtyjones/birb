use crate::{run_str_in_bash, Subcommand};

/// Deploy different applications / services
#[derive(Debug, StructOpt)]
pub enum Build {
    /// Deploy the Edgar worker
    #[structopt(name = "api")]
    Api,
    /// Deploys whatever change is held by the "plan" file
    #[structopt(name = "edgar")]
    Edgar,
}

fn copy_artifacts(package: &str, package_crate: &str) {
    run_str_in_bash("rm -rf out").unwrap();
    run_str_in_bash("mkdir out").unwrap();
    run_str_in_bash(format!("cp ./crates/{}/Dockerfile-prod out", package_crate).as_str()).unwrap();
    run_str_in_bash(
        format!(
            "cp ./target/x86_64-unknown-linux-musl/release/{} out",
            package
        )
        .as_str(),
    )
    .unwrap();
}

fn build_binary(package: &str) {
    let build_command = format!(
        "
        # get base image
        docker pull clux/muslrust:nightly

        # build binary
        make build-release package={}
    ",
        package
    );
    run_str_in_bash(build_command.as_str()).unwrap();
}

impl Subcommand for Build {
    fn run(&self) -> Result<(), failure::Error> {
        match self {
            Build::Api => {
                build_binary("api");
                copy_artifacts("api_bin", "api");
                Ok(())
            }
            Build::Edgar => {
                build_binary("edgar-worker");
                copy_artifacts("edgar_worker_bin", "edgar-worker");
                Ok(())
            }
        }
    }
}