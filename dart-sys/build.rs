use std::{
	env,
	env::VarError,
	fs::OpenOptions,
	io::Write,
	path::{Path, PathBuf},
};

use chrono::{DateTime, SecondsFormat, Utc};

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
	let mut file = OpenOptions::new()
		.append(true)
		.create(true)
		.open(PathBuf::from(env::var("OUT_DIR").unwrap()).join("build.log"))
		.unwrap();
	writeln!(file, "[{}] {}", now.to_rfc3339_opts(SecondsFormat::Secs, true), msg).unwrap();
}

/// Retuns the path to the preferred Dart SDK.
///
/// If an sdk CANNOT be found, it will panic and exit compilation instead of returning an error.
///
/// ## Arguments:
///
/// * `channel`: the channel to download the sdk from, if no sdk is found locally
fn get_dart_sdk() -> String {
	/// Attempts to find an installed Dart SDK.
	fn find_local_dart_sdk() -> Option<String> {
		log("INFO: searching for local Dart SDK");

		// firt and formost, check for the dart_sdk environment variable
		let dart_sdk: Result<String, VarError> = env::var("dart_sdk");
		if let Ok(dart_sdk) = dart_sdk {
			// if 'dart_sdk' is set, return Some(dart_sdk)
			Some(dart_sdk)
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
		log("INFO: searching for local flutter sdk");

		// firt and formost, check for the flutter_sdk environment variable
		let flutter_sdk: Result<String, VarError> = env::var("fluter_sdk");
		if let Ok(flutter_sdk) = flutter_sdk {
			// if 'flutter_sdk' is set, return Some(flutter_sdk)
			Some(flutter_sdk)
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

	// first, get path to Dart SDK
	log("INFO: searching for Dart SDK");
	// try to use a local dart-provided dart-sdk
	log("INFO: attempting to use local Dart SDK");
	let dart_sdk = match find_local_dart_sdk() {
		Some(x) => x,
		None => {
			log("INFO: failed to find local Dart SDK, trying to use local flutter sdk");
			// if that fails, try to use a local flutter-provided dart-sdk
			match find_local_flutter_sdk() {
				Some(x) => x,
				None => {
					// fall back to the sdk packaaged with this crate
					log("INFO: failed to find local flutter sdk, trying to use packaged Dart SDK");

					PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap())
						.join("dart-sdk")
						.to_str()
						.unwrap()
						.to_string()
				},
			}
		},
	};
	dart_sdk
}

#[cfg(not(feature = "dart_api_dl"))]
pub fn compile(_dart_sdk_path: &Path) {}

/// Generates bindings for the Dart API dynamic library.
#[cfg(not(feature = "dart_api_dl"))]
pub fn codegen(dart_sdk_path: &Path) {
	log("INFO: emitting compiler flags");

	// log location of Dart SDK
	log(&format!(
		"INFO: using Dart SDK at: \"{}\"",
		dart_sdk_path.to_str().unwrap()
	));

	// if target OS is windows, add extra compile flags nessecary for linking
	// AGAINST the Dart SDK binaries (gotta love windows)
	let target_os = env::var("CARGO_CFG_TARGET_OS");
	match target_os.as_ref().map(|x| &**x) {
		Ok("windows") => {
			log("INFO: target OS is windows, adding extra compile flags for linking against Dart SDK binaries");
			let dart_sdk_bin_path: PathBuf = dart_sdk_path.join("bin");
			let dart_sdk_lib_path = dart_sdk_path.join("bin").join("dart.lib");

			// Ensure that, on windows, the Dart SDK binaries are located at
			// `dart_sdk_path\bin\dart.exe` and `dart_sdk_path\bin\dart.lib`
			// and panic if they are not
			if !dart_sdk_lib_path.exists() {
				let error = &format!(
					"ERROR: Dart SDK binaries not found at \"{}\\.{{exe&lib}}\". Please ensure that the Dart SDK is \
					 installed correctly.",
					dart_sdk_bin_path.to_str().unwrap()
				);

				log(error);
				panic!("{}", error);
			}

			// log success location of Dart SDK binaries
			log(&format!(
				"INFO: successfully found Dart SDK binaries at: \"{}\"",
				dart_sdk_bin_path.to_str().unwrap()
			));

			// add extra compile flags for linking against Dart SDK binaries
			println!("cargo:rustc-link-search=native={}", dart_sdk_bin_path.to_str().unwrap());
			println!("cargo:rustc-link-lib=static=dart");
		},
		_ => log("INFO: target OS is not windows, skipping extra compile flags for linking against Dart SDK binaries"),
	}

	let dart_sdk_include_dir = dart_sdk_path.join("include");

	let bindings = bindgen::Builder::default()
		.header(
			dart_sdk_include_dir
				.join("dart_api.h")
				.to_str()
				.expect("ERROR: could not find path `dart_api_dl.h`"),
		)
		.header(
			dart_sdk_include_dir
				.join("dart_version.h")
				.to_str()
				.expect("ERROR: could not find path `dart_version.h`"),
		)
		.header(
			dart_sdk_include_dir
				.join("dart_native_api.h")
				.to_str()
				.expect("ERROR: could not find path `dart_native_api.h`"),
		)
		.header(
			dart_sdk_include_dir
				.join("dart_tools_api.h")
				.to_str()
				.expect("ERROR: could not find path `dart_tools_api.h`"),
		)
		.clang_arg("-DDART_SHARED_LIB")
		.generate()
		.expect("ERROR: bindgen failed to generate bindings");
	let out_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("ERROR: Could not find $CARGO_MANIFEST_DIR"));
	bindings
		.write_to_file(out_path.join("src/bindings/mod.rs"))
		.expect("ERROR: failed to write bindings to file");

	log("INFO: finished emitting compiler flags");
}

#[cfg(feature = "dart_api_dl")]
pub fn compile(dart_sdk_path: &Path) {
	let dart_sdk_include_dir = dart_sdk_path.join("include");
	let dart_dl_glue_path = dart_sdk_include_dir.join("dart_api_dl.c");
	cc::Build::new()
		.file(dart_dl_glue_path)
		.include(dart_sdk_include_dir)
		.compile("dart_api_dl");
}

/// Generates bindings for the Dart API dynamic library.
#[cfg(feature = "dart_api_dl")]
pub fn codegen(dart_sdk_path: &Path) {
	static DL_ENABLED_FUNCTIONS: &[&str] = &["Dart_InitializeApiDL"];

	static DL_ENABLED_TYPES: &[&str] = &[
		"Dart_.+_DL",
		"Dart_CObject",
		"Dart_Handle",
		"Dart_PersistentHandle",
		"Dart_WeakPersistentHandle",
		"Dart_HandleFinalizer",
		"Dart_FinalizableHandle",
		"Dart_CObject_Type",
		"Dart_TypedData_Type",
	];
	static DL_ENABLED_VARS: &[&str] = &["Dart_.+_DL", "DART_API_DL_MAJOR_VERSION", "DART_API_DL_MINOR_VERSION"];

	log("INFO: emitting compiler flags");

	// log location of Dart SDK
	log(&format!(
		"INFO: using Dart SDK at: \"{}\"",
		dart_sdk_path.to_str().unwrap()
	));

	// if target OS is windows, add extra compile flags nessecary for linking
	// AGAINST the Dart SDK binaries (gotta love windows)
	let target_os = env::var("CARGO_CFG_TARGET_OS");
	match target_os.as_ref().map(|x| &**x) {
		Ok("windows") => {
			log("INFO: target OS is windows, adding extra compile flags for linking against Dart SDK binaries");
			let dart_sdk_bin_path: PathBuf = dart_sdk_path.join("bin");
			let dart_sdk_lib_path = dart_sdk_path.join("bin").join("dart.lib");

			// Ensure that, on windows, the Dart SDK binaries are located at
			// `dart_sdk_path\bin\dart.exe` and `dart_sdk_path\bin\dart.lib`
			// and panic if they are not
			if !dart_sdk_lib_path.exists() {
				let error = &format!(
					"ERROR: Dart SDK binaries not found at \"{}\\.{{exe&lib}}\". Please ensure that the Dart SDK is \
					 installed correctly.",
					dart_sdk_bin_path.to_str().unwrap()
				);

				log(error);
				panic!("{}", error);
			}

			// log success location of Dart SDK binaries
			log(&format!(
				"INFO: successfully found Dart SDK binaries at: \"{}\"",
				dart_sdk_bin_path.to_str().unwrap()
			));

			// add extra compile flags for linking against Dart SDK binaries
			println!("cargo:rustc-link-search=native={}", dart_sdk_bin_path.to_str().unwrap());
			println!("cargo:rustc-link-lib=static=dart");
		},
		_ => log("INFO: target OS is not windows, skipping extra compile flags for linking against Dart SDK binaries"),
	}

	let dart_sdk_include_dir = dart_sdk_path.join("include");

	let mut builder = bindgen::Builder::default()
		.header(
			dart_sdk_include_dir
				.join("dart_api_dl.h")
				.to_str()
				.expect("ERROR: could not find path `dart_api_dl.h`"),
		)
		.header(
			dart_sdk_include_dir
				.join("dart_version.h")
				.to_str()
				.expect("ERROR: could not find path `dart_version.h`"),
		)
		.header(
			dart_sdk_include_dir
				.join("dart_native_api.h")
				.to_str()
				.expect("ERROR: could not find path `dart_native_api.h`"),
		)
		.header(
			dart_sdk_include_dir
				.join("dart_tools_api.h")
				.to_str()
				.expect("ERROR: could not find path `dart_tools_api.h`"),
		)
		.parse_callbacks(Box::new(bindgen::CargoCallbacks))
		.default_enum_style(bindgen::EnumVariation::NewType {
			is_bitfield: false,
			is_global: true,
		});

	for function_ in DL_ENABLED_FUNCTIONS {
		builder = builder.allowlist_function(function_);
	}

	for type_ in DL_ENABLED_TYPES {
		builder = builder.allowlist_type(type_);
	}

	for variable_ in DL_ENABLED_VARS {
		builder = builder.allowlist_var(variable_);
	}

	let bindings = builder
		.generate()
		.expect("ERROR: Failed to generate dart_api_dl binding");

	let out_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("ERROR: Could not find $CARGO_MANIFEST_DIR"));

	bindings
		.write_to_file(out_path.join("src/bindings_api_dl/mod.rs"))
		.expect("ERROR: failed to write dart_api_dl bindings to file");
}

fn main() {
	// emit cargo warning about where the build log file is located
	// do NOT emit this warning if the `ci` feature is enabled
	// because we run a strict CI, any warnings will cause the CI to fail
	#[cfg(not(feature = "ci"))]
	println!(
		"cargo:warning=INFO: build log is located at: `{}`",
		PathBuf::from(env::var("OUT_DIR").unwrap())
			.join("build.log")
			.to_str()
			.unwrap()
	);
	log("------------------------------");
	log("INFO: starting build script");

	#[cfg(not(feature = "docs_only"))]
	{
		// get Dart SDK path for linking
		let dart_sdk_path = PathBuf::from(&get_dart_sdk());

		// regen bindings
		if env::var("REGEN_DART_API").map_or(false, |var| var == "1") {
			codegen(&dart_sdk_path);
		}

		compile(&dart_sdk_path);
	}

	log("INFO: finished build script");
	log("------------------------------");
}
