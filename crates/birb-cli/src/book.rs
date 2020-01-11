use crate::{run_str_in_bash, Subcommand};
use crate::bb_filesystem;
use crate::bb_filesystem::bb_root_dir;

#[derive(Debug, StructOpt)]
pub enum Book {
  /// Launches a local server with the book and watches for changes
  #[structopt(name = "watch")]
  Watch,
  /// Builds the production book site
  #[structopt(name = "build")]
  Build,
  /// Builds and deploys the production book site to docs.birb.io
  #[structopt(name = "deploy")]
  Deploy,
}

impl Subcommand for Book {
  fn run(&self) -> Result<(), failure::Error> {
    match self {
      Book::Watch => {
        // Open browser and watch for book changes
        run_str_in_bash("mdbook watch crates/book --open")?;
        Ok(())
      }
      Book::Build => {
        // Generates the book and outputs it to crates/book/book
        run_str_in_bash("mdbook build crates/book")?;
        Ok(())
      }
      Book::Deploy => {
        // Build the book
        run_str_in_bash("bb book deploy")?;
        // Not currently worrying about whether or not the deploy was successful
        run_str_in_bash(
          "AWS_SDK_LOAD_CONFIG=1   AWS_PROFILE=birb aws s3 cp crates/book/book s3://docs.birb.io --recursive",
        )?;
        Ok(())
      }
    }
  }
}
