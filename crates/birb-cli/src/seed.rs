use crate::{run_str_in_bash, Subcommand};

#[derive(Debug, StructOpt)]
pub enum Seed {
    /// Perform seeds
    #[structopt(name = "up")]
    Up(Up),
}

#[derive(Debug, StructOpt)]
pub struct Up {
    /// the environment to run against
    #[structopt(short = "-e", long = "--env")]
    env: String, // TODO only accept valid environment values (e.g. prod, local)
}

impl Subcommand for Seed {
    fn run(&self) -> Result<(), failure::Error> {
        match self {
            Seed::Up(up) => {
                let cmd = format!("./scripts/seed.sh {} up", up.env);
                run_str_in_bash(cmd.as_str()).unwrap();
                // if seeding locally, use the dbslicer
                if up.env == "local" {
                    run_str_in_bash("cargo run -p dbslicer").expect("Couldn't run dbslicer");
                }
                Ok(())
            }
        }
    }
}
