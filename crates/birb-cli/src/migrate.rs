use crate::bb_filesystem::bb_root_dir;
use crate::{run_str_in_bash, Subcommand};

#[derive(Debug, StructOpt)]
pub enum Migrate {
    /// Perform "up" migrations
    #[structopt(name = "up")]
    Up(Up),
    /// Perform "down" migrations
    #[structopt(name = "down")]
    Down(Down),
    /// Revert (undo) last migration
    #[structopt(name = "revert")]
    Revert(Revert),
    /// Redo last migration
    #[structopt(name = "redo")]
    Redo(Redo),
    /// Create a new migration
    #[structopt(name = "create")]
    Create(Create),
    /// Get the migration status
    #[structopt(name = "status")]
    Status(Status),
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

#[derive(Debug, StructOpt)]
pub struct Revert {
    /// the environment to run against
    #[structopt(short = "-e", long = "--env")]
    env: String, // TODO only accept valid environment values (e.g. prod, local)
}

#[derive(Debug, StructOpt)]
pub struct Redo {
    /// the environment to run against
    #[structopt(short = "-e", long = "--env")]
    env: String, // TODO only accept valid environment values (e.g. prod, local)
}

#[derive(Debug, StructOpt)]
pub struct Create {
    /// the name of the migration to create
    #[structopt(short = "-s", long = "--slug")]
    slug: String,
}

#[derive(Debug, StructOpt)]
pub struct Status {
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
                if down.env == "prod" {
                    let script_path = bb_root_dir().join("scripts/start_ssh_tunnel.sh command");
                    run_str_in_bash(script_path.to_str().unwrap()).unwrap();
                }
                let cmd = format!("./scripts/migrate.sh {} down", down.env);
                run_str_in_bash(cmd.as_str()).unwrap();
                Ok(())
            }
            Migrate::Revert(revert) => {
                if revert.env == "prod" {
                    let script_path = bb_root_dir().join("scripts/start_ssh_tunnel.sh command");
                    run_str_in_bash(script_path.to_str().unwrap()).unwrap();
                }
                let cmd = format!("./scripts/migrate.sh {} revert", revert.env);
                run_str_in_bash(cmd.as_str()).unwrap();
                Ok(())
            }
            Migrate::Redo(redo) => {
                if redo.env == "prod" {
                    let script_path = bb_root_dir().join("scripts/start_ssh_tunnel.sh command");
                    run_str_in_bash(script_path.to_str().unwrap()).unwrap();
                }
                let cmd = format!("./scripts/migrate.sh {} redo", redo.env);
                run_str_in_bash(cmd.as_str()).unwrap();
                Ok(())
            }
            Migrate::Create(create) => {
                let cmd = format!("dbmigrate create --slug {} s--path ./db/migrations", create.slug);
                run_str_in_bash(cmd.as_str()).unwrap();
                Ok(())
            }
            Migrate::Status(status) => {
                if status.env == "prod" {
                    let script_path = bb_root_dir().join("scripts/start_ssh_tunnel.sh command");
                    run_str_in_bash(script_path.to_str().unwrap()).unwrap();
                }
                let cmd = format!("./scripts/migrate.sh {} status", status.env);
                run_str_in_bash(cmd.as_str()).unwrap();
                Ok(())
            }
        }
    }
}
