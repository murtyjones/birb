use crate::bb_filesystem::bb_root_dir;
use crate::{run_str_in_bash, Subcommand};
use std::path::PathBuf;

/// SSH into DB via Bastion
#[derive(Debug, StructOpt)]
pub struct Ssh {}

impl Subcommand for Ssh {
    fn run(&self) -> Result<(), failure::Error> {
        let script_path = bb_root_dir().join("scripts/start_ssh_tunnel.sh");
        run_str_in_bash(script_path.to_str().unwrap()).unwrap();
        Ok(())
    }
}
