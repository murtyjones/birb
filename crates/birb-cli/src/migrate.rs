use crate::{run_str_in_bash, Subcommand};
use crate::bb_filesystem::bb_root_dir;

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
                if up.env == "prod" {
                    let script_path = bb_root_dir().join("scripts/start_ssh_tunnel.sh command");
                    run_str_in_bash(script_path.to_str().unwrap()).unwrap();
                }
                let cmd = format!("./scripts/migrate.sh {} up", up.env);
                run_str_in_bash(cmd.as_str()).unwrap();
                Ok(())
            }
            Migrate::Down(down) => {
                if up.env == "prod" {
                    let script_path = bb_root_dir().join("scripts/start_ssh_tunnel.sh command");
                    run_str_in_bash(script_path.to_str().unwrap()).unwrap();
                }
                let cmd = format!("./scripts/migrate.sh {} down", down.env);
                run_str_in_bash(cmd.as_str()).unwrap();
                Ok(())
            }
        }
    }
}
