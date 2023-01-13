use std::{env, env::VarError, fs::OpenOptions, io::Write, path::PathBuf};

use chrono::prelude::*;

/// Logger for the build script.
///
/// Argument must be an `&str`.
///
/// # Example
///
/// ```rust
/// log("hello world");
/// ```
///
/// # Output
///
/// ```bash
/// $ cat build.log
/// ```
///
/// ```log
/// [2020-05-01T00:00:00Z] hello world
/// [2020-05-01T00:04:34Z] hello world 2.0!
/// ```
fn log(msg: &str) {
	let now: DateTime<Utc> = Utc::now();
	let mut file = OpenOptions::new().append(true).create(true).open("build.log").unwrap();
	writeln!(file, "[{}] {}", now.to_rfc3339(), msg).unwrap();
}

/// Attempts to find an installed dart sdk.
fn find_local_dart_sdk() -> Option<String> {
	log("INFO: searching for local dart sdk...");

	// firt and formost, check for the dart_sdk environment variable
	let dart_sdk: Result<String, VarError> = env::var("dart_sdk");
	if let Ok(dart_sdk) = dart_sdk {
		// if 'dart_sdk' is set, return Some(dart_sdk)
		return Some(dart_sdk);
	} else {
		// if 'dart_sdk' is not set, check `PATH`

		// get PATH
		let paths: Vec<PathBuf> =
			env::split_paths(&env::var("PATH").expect("Could not find $PATH variable.")).collect();

		for i in &paths {
			if i.components().any(|x| x.as_os_str() == "dart") {
				let mut path = i.clone();
				while !path.is_dir() || path.file_name().unwrap() != "dart-sdk" {
					path.pop();
				}
				if !path.as_os_str().is_empty() {
					return Some(path.to_str().unwrap().to_string());
				}
			}
		}

		None
	}
}

/// Attempts to find an installed flutter sdk.
fn find_local_flutter_sdk() -> Option<String> {
	log("INFO: searching for local flutter sdk...");

	// firt and formost, check for the flutter_sdk environment variable
	let flutter_sdk: Result<String, VarError> = env::var("fluter_sdk");
	if let Ok(flutter_sdk) = flutter_sdk {
		// if 'flutter_sdk' is set, return Some(flutter_sdk)
		return Some(flutter_sdk);
	} else {
		// if 'flutter_sdk' is not set, check `PATH`

		// get PATH
		let paths: Vec<PathBuf> =
			env::split_paths(&env::var("PATH").expect("Could not find $PATH variable.")).collect();

		for i in &paths {
			if i.components().any(|x| x.as_os_str() == "flutter") {
				let mut path = i.clone();
				while !path.is_dir() || path.file_name().unwrap() != "bin" {
					path.pop();
				}
				if !path.as_os_str().is_empty() {
					// return Some(path.to_str().unwrap().to_string());
					// append path prefix and return
					path.push("cache");
					path.push("dart-sdk");
					return Some(path.to_str().unwrap().to_string());
				}
			}
		}

		None
	}
}

/// returns path to the dart sdk packaged with `Dart-sys`
fn use_packaged_dart_sdk() -> Option<String> {
	log("INFO: attempting to use dart sdk packaged with dart-sys...");
	return Some(String::from(format!(
		"{}/dart-sdk/runtime/",
		env::var("CARGO_MANIFEST_DIR").unwrap()
	)));
}

/// Emits the compiler flags for `cargo build`
fn emit_compiler_flags() {
	// first, get path to dart sdk
	log("INFO: searching for dart sdk...");

	// try to use a local dart-provided dart-sdk
	log("INFO: attempting to use local dart sdk...");
	let dart_sdk = match find_local_dart_sdk() {
		Some(x) => x,
		None => {
			log("INFO: failed to find local dart sdk, trying to use local flutter sdk...");
			// if that fails, try to use a local flutter-provided dart-sdk
			match find_local_flutter_sdk() {
				Some(x) => x,
				None => {
					// if we can't find a local sdk, try to download one
					log("INFO: failed to find local flutter sdk, attempting to use dart sdk packaged with dart-sys...");
					match use_packaged_dart_sdk() {
						Some(x) => x,
						None => {
							// if we can't find a dart sdk, panic
							const ERROR: &str =
								"ERROR:\n  failed to find a dart sdk.\n  Please install the Dart SDK or set the \
								 dart_sdk environment variable to the path of the Dart SDK.\n  If you have Flutter \
								 installed, you can set the flutter_sdk environment variable to the path of the \
								 Flutter SDK.\n  If you have the Dart SDK installed, you can set the dart_sdk \
								 environment variable to the path of the Dart SDK.\n\n  If either flutter or dart are \
								 installed and available in your $PATH, they will be automatically found.\n\n  Common \
								 causes of this error are:\n    - The Dart SDK is not installed\n    - The Dart SDK \
								 is not in your $PATH\n    - You are using a forked version of `dart-sys` \
								 (unreccomended). ";
							log(ERROR);
							panic!("{}", ERROR);
						},
					}
				},
			}
		},
	};

	log(&format!("INFO: using dart sdk at: \"{}\"...", dart_sdk));
	// if target OS is windows, add extra compile flag	let target_os: Option<&'static str> =
	// let target_os = env::var("CARGO_CFG_TARGET_OS");
	// match target_os.as_ref().map(|x| &**x) {
	// 	Ok("windows") => {
	// 		let dart_path = match find_local_dart_sdk() {
	// 			Some(x) => x,
	// 			None => {
	// 				panic!(
	// 					"Could not find a dart SDK.\nPlease install the Dart SDK or set the dart_sdk environment \
	// 					 variable to the path of the Dart SDK.\nIf you have Flutter installed, the dart SDK will be \
	// 					 found automatically."
	// 				)
	// 			},
	// 		};
	// 		let dart_path = dart_path;
	// 		println!(
	// 			r#"cargo:rustc-link-search=native={}"#,
	// 			dart_path.as_str().join("bin").to_str().unwrap()
	// 		);
	// 		println!(r"cargo:rustc-link-lib=dart");
	// 	},
	// 	_ => (),
	// }
	// let bindings = bindgen::Builder::default()
	// 	.header("./bindgen/wrapper.h")
	// 	.clang_arg(format!(
	// 		"--include-directory={}",
	// 		dart_path.join("include").to_str().unwrap()
	// 	))
	// 	.clang_arg("-DDART_SHARED_LIB")
	// 	.generate()
	// 	.expect("Unable to generate bindings.");
	// let out_path = PathBuf::from(env::var("OUT_DIR").expect("Could not find OUT_DIR"));
	// bindings
	// 	.write_to_file(out_path.join("bindings.rs"))
	// 	.expect("Couldn't write bindings!");
	// panic!("OUT_DIR: {:?}", env::var("OUT_DIR").expect("Could not find OUT_DIR"));
}

fn main() {
	log("------------------------------");
	log("INFO: starting build script...");
	// #[cfg(not(feature = "docs-only"))]
	emit_compiler_flags();

	// let target_os = env::var("CARGO_CFG_TARGET_OS");
	// match target_os.as_ref().map(|x| &**x) {
	// 	Ok("windows") => {
	// 		// do windows things here
	// 	},
	// 	_ => {
	// 		// do normal things here
	// 	},
	// }
	log("INFO: finished build script...");
	log("------------------------------");
}
