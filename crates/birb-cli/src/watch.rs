use crate::{run_str_in_bash, Subcommand};

/// Deploy different applications / services
#[derive(Debug, StructOpt)]
pub struct Watch {}

impl Subcommand for Watch {
    fn run(&self) -> Result<(), failure::Error> {
        run_str_in_bash("bb docker restart").unwrap();
        run_str_in_bash("bb docker up-no-test").unwrap();
        run_str_in_bash("sleep 5s").unwrap();
        run_str_in_bash("cargo watch -x \"run -p api\"").unwrap();
        Ok(())
    }
}
