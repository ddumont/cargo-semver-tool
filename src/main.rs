/*
 * © Copyright Dan Dumont 2023. All rights reserved. Use of this source code is
 * governed by the Apache-2.0 license that can be found in the LICENSE file.
 */

//! Update the version of your rust project!
//!
//! # Getting Started
//! ```console
//! $ cargo install cargo-semver-tool
//! ```
//! ## Usage
//! ```console
//! $ cargo semver --help
//! A cargo command to help you version your rust project similar to npm-version
//!
//! Usage: cargo-semver <COMMAND>
//!
//! Commands:
//!   get   Return the project's current version
//!   bump  Bump the project's version
//!   help  Print this message or the help of the given subcommand(s)
//!
//! Options:
//!   -h, --help     Print help
//!   -V, --version  Print version
//! ```
//! ## Available Subcommands
//!
//! ### `cargo semver get`
//!
//! #### Usage
//! ```console
//! $ cargo semver get --help
//! Return the project's current version
//!
//! Usage: cargo-semver get
//!
//! Options:
//!   -h, --help  Print help
//! ```
//!
//! ### `cargo semver bump --help`
//!
//! #### Usage
//! ```console
//! Bump the project's version
//!
//! Usage: cargo-semver bump [OPTIONS] [BUMP]
//!
//! Arguments:
//!   [BUMP]
//!           [default: patch]
//!
//!           Possible values:
//!           - major:      Bump the major digit of the version
//!               1.0.0   -> 2.0.0
//!               1.1.0   -> 2.0.0
//!               2.0.0-1 -> 2.0.0
//!           - minor:      Bump the minor digit of the version
//!               1.0.0   -> 1.1.0
//!               1.0.1   -> 1.1.0
//!               1.1.0-1 -> 1.1.0
//!           - patch:      Bump the patch digit of the version
//!               1.0.0   -> 1.0.1
//!               1.0.0-1 -> 1.0.0
//!           - premajor:   Bump the major digit of the version as a prerelease
//!               1.0.0   -> 2.0.0-0
//!               2.0.0-0 -> 2.0.0-1
//!           - preminor:   Bump the minor digit of the version as a prerelease
//!               1.0.0   -> 1.1.0-0
//!               1.1.0-0 -> 1.1.0-1
//!           - prepatch:   Bump the patch digit of the version as a prerelease
//!               1.0.0   -> 1.0.1-0
//!               1.0.1-0 -> 1.0.1-1
//!           - prerelease: Bump the prelease version, or behaves like prepatch
//!               1.1.0   -> 1.1.1-0
//!               1.1.1-0 -> 1.1.1-1
//!               1.1.0-5 -> 1.1.0-6
//!           - release:    Bump the version from a prerelease, or behaves like patch
//!               1.1.0   -> 1.1.1
//!               1.1.1-5 -> 1.1.1
//!
//! Options:
//!   -d, --dry-run
//!           Display only, do not write the file
//!
//!   -h, --help
//!           Print help (see a summary with '-h')
//! ```

mod cli;
use cli::exec;

use std::fs;

use toml_edit::Document;

const CARGO_TOML: &'static str = "./Cargo.toml";

fn main() {
	let cargo = fs::read_to_string(CARGO_TOML)
		.expect("Error: could not read Cargo.toml file.");
	let contents = cargo.parse::<Document>()
		.expect("Error: malformed Cargo.toml file.");
    exec(contents);
}
