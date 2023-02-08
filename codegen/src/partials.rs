/// Header stub for Cargo.toml
pub const CARGO_TOML_HEADER_PARTIAL: &str = "[package]
name = \"dart-sys\"
description = \"Statically generated, Opt-in style bindings to the Dart SDK\"
version.workspace = true
edition.workspace = true
license.workspace = true
authors.workspace = true
repository.workspace = true
keywords.workspace = true
categories.workspace = true
readme = \"../README.md\"

[build-dependencies]
cc = \"^1.0.67\"

[features]
";

/// Header stub for lib.rs
pub const LIB_RS_HEADER_PARTIAL: &str = "//! Opt-in style bindings to the Dart SDK
//!
//! This crate provides bindings to \
                                         the Dart SDK. It is generated using
//! [bindgen](https://crates.io/crates/bindgen) \
                                         and the official Dart SDK.
//!
//! Bindings are generated statically, meaning that the Dart SDK headers are
//! included in the crate and no external dependencies are required.
#![no_std]
#![allow(
	non_upper_case_globals,
	non_camel_case_types,
	non_snake_case,
	unused_variables,
	dead_code
)]
";

/// build.rs stub
pub const BUILD_RS_PARTIAL: &str = "use std::{env, path::PathBuf};
fn main() {
	cc::Build::new()
		.file(
			PathBuf::from(env::var(\"CARGO_MANIFEST_DIR\").unwrap())
				.join(\"dart-sdk\")
				.join(\"include\")
				.join(\"dart_api_dl.c\"),
		)
		.include(
			PathBuf::from(env::var(\"CARGO_MANIFEST_DIR\").unwrap())
				.join(\"dart-sdk\")
				.join(\"include\"),
		)
		.compile(\"dart_api_dl\");
}
";
