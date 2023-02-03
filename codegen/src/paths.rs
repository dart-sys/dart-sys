/// returns the path to the repository root
///
/// ## Panics
///
/// Panics if the `CARGO_MANIFEST_DIR` environment variable is not set.
pub fn repo_root() -> std::path::PathBuf {
	std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
		.parent()
		.unwrap()
		.to_path_buf()
}

/// returns the path to the `dart-sdk` directory
///
/// ## Panics
///
/// Panics if the `CARGO_MANIFEST_DIR` environment variable is not set.
pub fn dart_sdk_path() -> std::path::PathBuf {
	dart_sys_crate_path().join("dart-sdk")
}

/// returns the path to the prefered temp directory to use
/// for the code generator
///
/// ## Panics
///
/// Panics if the `CARGO_MANIFEST_DIR` environment variable is not set.
pub fn temp_dir() -> std::path::PathBuf {
	repo_root().join("codegen").join("temp")
}

/// returns the path to the log file
///
/// ## Panics
///
/// Panics if the `CARGO_MANIFEST_DIR` environment variable is not set.
pub fn log_file_path() -> std::path::PathBuf {
	repo_root().join("codegen").join("build.log")
}

/// returns the path to the dart-sdk bin directory
///
/// ## Panics
///
/// Panics if the `CARGO_MANIFEST_DIR` environment variable is not set.
pub fn dart_sdk_bin_path() -> std::path::PathBuf {
	dart_sdk_path().join("bin")
}

/// returns the path to the dart-sdk include directory
///
/// ## Panics
///
/// Panics if the `CARGO_MANIFEST_DIR` environment variable is not set.
pub fn dart_sdk_include_path() -> std::path::PathBuf {
	dart_sdk_path().join("include")
}

/// returns the path to the dart-sys crate
///
/// ## Panics
///
/// Panics if the `CARGO_MANIFEST_DIR` environment variable is not set.
pub fn dart_sys_crate_path() -> std::path::PathBuf {
	repo_root().join("dart-sys")
}
