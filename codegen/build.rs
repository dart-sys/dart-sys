use std::{env, path::PathBuf};

fn main() {
	cc::Build::new()
		.file(
			PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
				.parent()
				.unwrap()
				.join("dart-sdk")
				.join("include")
				.join("dart_api_dl.c"),
		)
		.include(
			PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
				.parent()
				.unwrap()
				.join("dart-sdk")
				.join("include"),
		)
		.compile("dart_api_dl");
}
