/*
 * © Copyright Dan Dumont 2024. All rights reserved. Use of this source code is
 * governed by the Apache-2.0 license that can be found in the LICENSE file.
 */

use std::fs::File;
use std::io::Write;

use clap::{Parser, Subcommand, ValueEnum};
use semver::{Version, Prerelease};
use toml_edit::{Document, Item, Formatted, Value};

use crate::CARGO_TOML;

#[cfg(test)]
#[path = "../test/cli_test.rs"]
mod cli_test;


#[derive(Debug, Parser)]
#[clap(version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, ValueEnum, Clone)]
enum Bump {
	#[clap(help = "Bump the major digit of the version \n  1.0.0   -> 2.0.0\n  1.1.0   -> 2.0.0\n  2.0.0-1 -> 2.0.0")]
	Major,
	#[clap(help = "Bump the minor digit of the version \n  1.0.0   -> 1.1.0\n  1.0.1   -> 1.1.0\n  1.1.0-1 -> 1.1.0")]
	Minor,
	#[clap(help = "Bump the patch digit of the version \n  1.0.0   -> 1.0.1\n  1.0.0-1 -> 1.0.0")]
	Patch,
	#[clap(help = "Bump the major digit of the version as a prerelease\n  1.0.0   -> 2.0.0-0\n  2.0.0-0 -> 2.0.0-1")]
	Premajor,
	#[clap(help = "Bump the minor digit of the version as a prerelease\n  1.0.0   -> 1.1.0-0\n  1.1.0-0 -> 1.1.0-1")]
	Preminor,
	#[clap(help = "Bump the patch digit of the version as a prerelease\n  1.0.0   -> 1.0.1-0\n  1.0.1-0 -> 1.0.1-1")]
	Prepatch,
	#[clap(help = "Bump the prelease version, or behaves like prepatch\n  1.1.0   -> 1.1.1-0\n  1.1.1-0 -> 1.1.1-1\n  1.1.0-5 -> 1.1.0-6")]
	Prerelease,
	#[clap(help = "Bump the version from a prerelease, or behaves like patch\n  1.1.0   -> 1.1.1\n  1.1.1-5 -> 1.1.1")]
	Release,
}

#[derive(Debug, Subcommand)]
enum Commands {
	#[command(about = "Return the project's current version")]
	Get,
	#[clap(about = "Bump the project's version", value_enum)]
	Bump {
		#[clap(default_value = "patch")]
		bump: Option<Bump>,
		#[arg(default_value = "false", short = 'd', long = "dry-run", help = "Display only, do not write the file")]
		dry_run: bool
	},
}

pub(crate) fn exec(mut contents: Document) {
	let version = {
		if let Some(pkg) = contents.get("package") {
			pkg.get("version")
				.expect("Error: no version found in package table.")
				.as_str()
				.expect("Error: package.version value could not be parsed.")
		} else if let Some(lib) = contents.get("lib") {
			lib.get("version")
				.expect("Error: no version found in lib table.")
				.as_str()
				.expect("Error: lib.version value could not be parsed.")
		} else {
			panic!("Error: could not find version container in Cargo.toml file.")
		}
	};

	let args: Vec<String> = std::env::args()
		.enumerate()
		// If we are called from cargo, remove this arg
		.filter_map(|(idx, arg)| match (idx, arg.as_str()) { (1, "semver") => None, _ => Some(arg) })
		.collect();

	match Cli::parse_from(args).command {
		Commands::Get => println!("{version}"),
		Commands::Bump { bump, dry_run } => {
			let new_version = do_bump(bump.unwrap_or(Bump::Patch), &version);
			if !dry_run {
				let new_version = Item::Value(Value::String(Formatted::new(new_version.clone())));

				// We got this before...
				let package = contents.get_mut("package").unwrap().as_table_mut().unwrap();
				package.insert("version", new_version);

				let mut file = File::options().write(true).truncate(true).open(CARGO_TOML).expect("Error: failed to open cargo file for write.");
				write!(file, "{contents}").expect("Error: failed to write cargo file.");
			}
			println!("{new_version}");
		}
	}
}

fn do_bump(kind: Bump, version: &str) -> String {
	let mut nver = Version::parse(version)
		.expect(format!("Error: version ({version}) is not valid semver").as_str());

	match kind {
		Bump::Major => match nver.pre.as_str() {
			"" => { nver.major(); }
			 _ => match (nver.major, nver.minor, nver.patch) {
				(0, 0, 0) => { nver.major(); }
				(_, 0, 0) => { nver.empty_pre(); }
				(_, _, _) => { nver.major(); }
			}
		}
		Bump::Premajor => match nver.pre.as_str() {
			"" => { nver.major().zero_pre(); }
			 _ => match (nver.major, nver.minor, nver.patch) {
				(_, 0, 0) => { nver.pre(); }
				(_, _, _) => { nver.major().zero_pre(); }
			}
		}
		Bump::Minor => match nver.pre.as_str() {
			"" => { nver.minor(); }
			 _ => match (nver.minor, nver.patch) {
				(0, 0) => { nver.minor(); }
				(_, 0) => { nver.empty_pre(); }
				(_, _) => { nver.minor(); }
			}
		}
		Bump::Preminor => match nver.pre.as_str() {
			"" => { nver.minor().zero_pre(); }
			 _ => match (nver.minor, nver.patch) {
				(0, 0) => { nver.minor().zero_pre(); }
				(_, 0) => { nver.pre(); }
				(_, _) => { nver.minor().zero_pre(); }
			}
		}
		Bump::Patch => match nver.pre.as_str() {
			"" => { nver.patch(); }
			 _ => { nver.empty_pre(); }
		}
		Bump::Prepatch => match nver.pre.as_str() {
			"" => { nver.patch().zero_pre(); }
			 _ => match nver.patch {
				0 => { nver.patch().zero_pre(); }
				_ => { nver.pre(); }
			 }
		}
		Bump::Prerelease => match nver.pre.as_str() {
			"" => { nver.patch().zero_pre(); }
			 _ => { nver.pre(); }
		}
		Bump::Release => match nver.pre.as_str() {
			"" => { nver.patch(); }
			 _ => { nver.empty_pre(); }
		}
	}

	nver.to_string()
}

trait Increment {
	fn major(&mut self) -> &mut Self;
	fn minor(&mut self) -> &mut Self;
	fn patch(&mut self) -> &mut Self;
	fn pre(&mut self) -> &mut Self;
	fn zero_pre(&mut self) -> &mut Self;
	fn empty_pre(&mut self) -> &mut Self;
}
impl Increment for Version {
    fn major(&mut self) -> &mut Self {
		self.major = self.major + 1;
		self.minor = 0;
		self.patch = 0;
		self.pre = Prerelease::EMPTY;

		self
    }

	fn minor(&mut self) -> &mut Self {
		self.minor = self.minor + 1;
		self.patch = 0;
		self.pre = Prerelease::EMPTY;

		self
    }

	fn patch(&mut self) -> &mut Self {
        self.patch = self.patch + 1;
		self.pre = Prerelease::EMPTY;

		self
    }

	fn pre(&mut self) -> &mut Self {
		let bumped = match self.pre.as_str() {
			"" => 0,
			 _ => self.pre.parse::<usize>().unwrap()
		} + 1;

        self.pre = Version::parse(format!("0.0.0-{}", bumped).as_str()).unwrap().pre;

		self
    }

	fn zero_pre(&mut self) -> &mut Self {
        self.pre = Version::parse("0.0.0-0").unwrap().pre;

		self
    }

	fn empty_pre(&mut self) -> &mut Self {
        self.pre = Prerelease::EMPTY;

		self
    }
}
