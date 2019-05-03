use crate::{run_str_in_bash, Subcommand};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Docker {
    /// A single command to start/restart Birb's docker setup in the background.
    /// Stops the docker containers that are running, runs docker up, and then runs
    /// docker init
    #[structopt(name = "rebuild")]
    Rebuild,
    /// Starts Birb's docker setup in the background.
    #[structopt(name = "up")]
    Up,
    /// Stops the docker containers that are running.
    #[structopt(name = "down")]
    Down,
    /// Stops the docker containers that are running, then starts Birb's docker
    /// containers in the background.
    #[structopt(name = "restart")]
    Restart,
    /// Removes all Docker artifacts locally.
    #[structopt(name = "prune")]
    Prune,
    /// Follow logs.
    #[structopt(name = "logs")]
    Logs,
}

impl Subcommand for Docker {
    /// Process the docker subcommand in our CLI.
    ///
    /// More info: `bb docker --help`
    fn run(&self) -> Result<(), failure::Error> {
        match self {
            Docker::Rebuild => {
                run_str_in_bash("
                    docker-compose down
                    docker-compose build
                    docker-compose up -d").unwrap();
            }
            Docker::Up => {
                run_str_in_bash("docker-compose up -d").unwrap();
            }
            Docker::Down => {
                run_str_in_bash("docker-compose down").unwrap();
            }
            Docker::Restart => {
                run_str_in_bash("bb docker down && bb docker up").unwrap();
            }
            Docker::Prune => {
                run_str_in_bash("docker system prune -f && docker volume prune -f").unwrap();
            }
            Docker::Logs => {
                run_str_in_bash("docker-compose logs -f").unwrap();
            }
        };

        Ok(())
    }
}
