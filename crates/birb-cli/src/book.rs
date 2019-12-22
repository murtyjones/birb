use crate::{run_str_in_bash, Subcommand};
use crate::bb_filesystem;
use crate::bb_filesystem::bb_root_dir;

#[derive(Debug, StructOpt)]
pub enum Book {
  /// Launches a local server with the book and watches for changes
  #[structopt(name = "watch")]
  Watch,
}

impl Subcommand for Book {
  fn run(&self) -> Result<(), failure::Error> {
    match self {
      Book::Watch => {
        // Open browser and watch for book changes
        run_str_in_bash("mdbook watch crates/book --open")?;
        Ok(())
      }
    }
  }
}
