use crate::{run_str_in_bash, Subcommand};

/// Deploy different applications / services
#[derive(Debug, StructOpt)]
pub struct Watch {}

impl Subcommand for Watch {
    fn run(&self) -> Result<(), failure::Error> {
        // Make a vector to hold the children which are spawned.
        let mut children = vec![];

        children.push(std::thread::spawn(move || {
            run_str_in_bash("cargo watch -x \"run -p server\"").unwrap();
        }));

        children.push(std::thread::spawn(move || {
            run_str_in_bash("npm start").unwrap();
        }));

        for child in children {
            // Wait for the thread to finish. Returns a result.
            let _ = child.join();
        }

        Ok(())
    }
}
