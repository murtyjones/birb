use crate::bb_filesystem::bb_root_dir;
use crate::{run_str_in_bash, Subcommand};
use dbslicer;

/// SSH into DB via Bastion
#[derive(Debug, StructOpt)]
pub struct DbSlicer {}

impl Subcommand for DbSlicer {
    fn run(&self) -> Result<(), failure::Error> {
        let script_path = bb_root_dir().join("scripts/start_ssh_tunnel.sh command");
        run_str_in_bash(script_path.to_str().unwrap()).unwrap();

        dbslicer::run();

        Ok(())
    }
}
