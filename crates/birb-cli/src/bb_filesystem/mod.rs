//! Filesystem operations tailored to the layout and conventions of the Birb repository.
//!
//! Centralizing these operations makes them easy to tweak / enhance / generalize over time.
//! If our file system conventions change we have one place to make changes
//! (this module and/or it's submodules).

use lazy_static::lazy_static;
use std::fs;
use std::path::{Path, PathBuf};

lazy_static! {
    /// The Birb source code root directory.
    ///
    /// For different developers this will be a different folder on the file system.
    ///
    /// Since we don't know where that folder is, but we do know that it is two directories
    /// above the birb-cli directory, we just use a relative path and canonicalize it
    ///  (remove the dots, basically)
    static ref BB_ROOT_DIR: PathBuf = {
        let bb_root_dir = env!("CARGO_MANIFEST_DIR").to_owned() + "/../..";
        let bb_root_dir = PathBuf::from(bb_root_dir);
        let bb_root_dir = bb_root_dir.canonicalize().unwrap();
        bb_root_dir
    };
}

/// Get the path to the root directory of your Birb app repo.
///
/// ex: /Users/sally/code/birb/birb-project
pub fn bb_root_dir() -> &'static Path {
    BB_ROOT_DIR.as_ref()
}

/// Get the path to the birb-cli crate.
///
/// ex: /Users/sally/code/birb/crates/birb-cli
pub fn bb_cli_dir() -> &'static str {
    env!("CARGO_MANIFEST_DIR")
}

/// The `.bb` directory in your $HOME directory where the CLI stores information
pub fn bb_dot_dir() -> String {
    format!("{}/.bb", dirs::home_dir().unwrap().to_str().unwrap())
}

/// Get the version of the command line tools from the Cargo.toml file in the Birb CLI crate.
///
/// Note that this is not the version that your bb binary was compiled with.
/// This is just the version that is in crates/birb-cli/Cargo.toml, which might change as
/// you change branches.
pub fn cargo_toml_version() -> String {
    let cargo_toml = format!("{}/Cargo.toml", env!("CARGO_MANIFEST_DIR"));

    let contents = fs::read_to_string(cargo_toml).unwrap();

    for line in contents.lines() {
        if line.starts_with("version") {
            let mut split = line.split(r#"""#);

            split.next().unwrap();

            return split.next().unwrap().to_string();
        }
    }

    unreachable!("We will always have a version");
}
