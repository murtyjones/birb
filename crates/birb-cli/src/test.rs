use crate::{run_str_in_bash, Subcommand};

#[derive(Debug, StructOpt)]
pub enum Test {
    /// Test everything
    #[structopt(name = "all")]
    All,
    /// Test the filing-parser crate
    #[structopt(name = "parser")]
    FilingParser,
}

impl Subcommand for Test {
    fn run(&self) -> Result<(), failure::Error> {
        match self {
            Test::All => {
                run_str_in_bash(
                    r#"
                    docker-compose down
                    docker-compose up -d
                    docker-compose run --rm test bash -c "cargo test --all"
                "#,
                )
                .unwrap();
                Ok(())
            }
            Test::FilingParser => {
                run_str_in_bash(
                    r#"
                    cargo run -p filing-parser --bin download_examples
                    cargo test -p filing-parser
                "#,
                )
                .unwrap();
                Ok(())
            }
        }
    }
}
