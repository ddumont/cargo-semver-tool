/*
 * © Copyright Dan Dumont 2023. All rights reserved. Use of this source code is
 * governed by the Apache-2.0 license that can be found in the LICENSE file.
 */

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
