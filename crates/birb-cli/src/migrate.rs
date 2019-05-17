use crate::{run_str_in_bash, Subcommand};

#[derive(Debug, StructOpt)]
pub enum Migrate {
    /// Perform "up" migrations
    #[structopt(name = "up")]
    Up(Up),
    /// Perform "down" migrations
    #[structopt(name = "down")]
    Down(Down),
}

#[derive(Debug, StructOpt)]
pub struct Up {
    /// the environment to run against
    #[structopt(short = "-e", long = "--env")]
    env: String, // TODO only accept valid environment values (e.g. prod, local)
}

#[derive(Debug, StructOpt)]
pub struct Down {
    /// the environment to run against
    #[structopt(short = "-e", long = "--env")]
    env: String, // TODO only accept valid environment values (e.g. prod, local)
}

impl Subcommand for Migrate {
    fn run(&self) -> Result<(), failure::Error> {
        match self {
            Migrate::Up(up) => {
                let cmd = format!("./scripts/migrate.sh {} up", up.env);
                run_str_in_bash(cmd.as_str()).unwrap();
                Ok(())
            }
            Migrate::Down(down) => {
                let cmd = format!("./scripts/migrate.sh {} down", down.env);
                run_str_in_bash(cmd.as_str()).unwrap();
                Ok(())
            }
        }
    }
}
