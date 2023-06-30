/*
 * © Copyright Dan Dumont 2023. All rights reserved. Use of this source code is
 * governed by the Apache-2.0 license that can be found in the LICENSE file.
 */

#[cfg(test)]
use crate::cli::*;

#[test]
fn test_major_bump() {
	assert_eq!(do_bump(Bump::Major, "1.0.0"),   "2.0.0");
	assert_eq!(do_bump(Bump::Major, "1.0.0-0"), "1.0.0");
	assert_eq!(do_bump(Bump::Major, "1.1.0-0"), "2.0.0");
	assert_eq!(do_bump(Bump::Major, "1.1.1-0"), "2.0.0");
}

#[test]
fn test_premajor_bump() {
	assert_eq!(do_bump(Bump::Premajor, "1.0.0"),   "2.0.0-0");
	assert_eq!(do_bump(Bump::Premajor, "1.0.0-0"), "1.0.0-1");
	assert_eq!(do_bump(Bump::Premajor, "1.1.0-0"), "2.0.0-0");
	assert_eq!(do_bump(Bump::Premajor, "1.1.1-0"), "2.0.0-0");
}

#[test]
fn test_minor_bump() {
	assert_eq!(do_bump(Bump::Minor, "1.0.0"),   "1.1.0");
	assert_eq!(do_bump(Bump::Minor, "1.0.0-0"), "1.1.0");
	assert_eq!(do_bump(Bump::Minor, "1.1.0-0"), "1.1.0");
	assert_eq!(do_bump(Bump::Minor, "1.1.1-0"), "1.2.0");
}

#[test]
fn test_preminor_bump() {
	assert_eq!(do_bump(Bump::Preminor, "1.0.0"),   "1.1.0-0");
	assert_eq!(do_bump(Bump::Preminor, "1.0.0-0"), "1.1.0-0");
	assert_eq!(do_bump(Bump::Preminor, "1.1.0-0"), "1.1.0-1");
	assert_eq!(do_bump(Bump::Preminor, "1.1.1-0"), "1.2.0-0");
}

#[test]
fn test_patch_bump() {
	assert_eq!(do_bump(Bump::Patch, "1.0.0"),   "1.0.1");
	assert_eq!(do_bump(Bump::Patch, "1.0.0-0"), "1.0.0");
	assert_eq!(do_bump(Bump::Patch, "1.1.0-0"), "1.1.0");
	assert_eq!(do_bump(Bump::Patch, "1.1.1-0"), "1.1.1");
}

#[test]
fn test_prepatch_bump() {
	assert_eq!(do_bump(Bump::Prepatch, "1.0.0"),   "1.0.1-0");
	assert_eq!(do_bump(Bump::Prepatch, "1.0.0-0"), "1.0.1-0");
	assert_eq!(do_bump(Bump::Prepatch, "1.1.0-0"), "1.1.1-0");
	assert_eq!(do_bump(Bump::Prepatch, "1.1.1-0"), "1.1.1-1");
}

#[test]
fn test_prerelease_bump() {
	assert_eq!(do_bump(Bump::Prerelease, "1.0.0"),   "1.0.1-0");
	assert_eq!(do_bump(Bump::Prerelease, "1.0.0-0"), "1.0.0-1");
	assert_eq!(do_bump(Bump::Prerelease, "1.1.0-0"), "1.1.0-1");
	assert_eq!(do_bump(Bump::Prerelease, "1.1.1-0"), "1.1.1-1");
}

#[test]
fn test_release_bump() {
	assert_eq!(do_bump(Bump::Release, "1.0.0"),   "1.0.1");
	assert_eq!(do_bump(Bump::Release, "1.0.0-0"), "1.0.0");
	assert_eq!(do_bump(Bump::Release, "1.1.0-0"), "1.1.0");
	assert_eq!(do_bump(Bump::Release, "1.1.1-0"), "1.1.1");
}
