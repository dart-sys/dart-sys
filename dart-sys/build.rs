#[cfg(not(feature = "download_dart_sdk"))]
use std::env::VarError;
use std::{
	env,
	// "Error" is a very generic name and oftehn conflicts with other crates
	error::Error as StdError,
	fs::{self, File, OpenOptions},
	io::{self, Error as IoError, ErrorKind as IoErrorKind, Read, Write},
	path::{Path, PathBuf},
	time::Duration,
};

use chrono::{DateTime, SecondsFormat, Utc};
use reqwest::StatusCode;
use sha2::{Digest, Sha256};
use zip::ZipArchive;

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

#[cfg(not(feature = "download_dart_sdk"))]
/// Attempts to find an installed Dart SDK.
fn find_local_dart_sdk() -> Option<String> {
	log("INFO: searching for local Dart SDK");

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

#[cfg(not(feature = "download_dart_sdk"))]
/// Attempts to find an installed flutter sdk.
fn find_local_flutter_sdk() -> Option<String> {
	log("INFO: searching for local flutter sdk");

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

/// Dart SDK channel
/// Options are `stable`, `beta`, and `dev`.
enum DartSdkChannel {
	Stable,
	Beta,
	Dev,
}

impl DartSdkChannel {
	fn to_string(&self) -> String {
		match self {
			DartSdkChannel::Stable => "stable".to_string(),
			DartSdkChannel::Beta => "beta".to_string(),
			DartSdkChannel::Dev => "dev".to_string(),
		}
	}

	#[allow(dead_code)]
	fn from_string(s: &str) -> Option<DartSdkChannel> {
		match s {
			"stable" => Some(DartSdkChannel::Stable),
			"beta" => Some(DartSdkChannel::Beta),
			"dev" => Some(DartSdkChannel::Dev),
			_ => None,
		}
	}
}

/// Downloads a Dart SDK.
/// Returns `None` if the download fails.
/// Returns `Some(path)` if the download succeeds.
///
/// ## Argsuments:
/// * `channel`: the channel to download the sdk from. Defaults to `stable`. Options are `stable`,
///   `beta`, and `dev`.
fn download_dart_sdk(channel: DartSdkChannel) -> Result<String, Box<dyn StdError>> {
	log("INFO: attempting to download dart-sdk");
	// get the current platform name
	let platform = match env::consts::OS {
		"linux" => "linux",
		"macos" => "macos",
		"windows" => "windows",
		_ => {
			const ERROR: &str = "ERROR: unknown/unsupported OS";
			log(ERROR);
			return Err(Box::new(IoError::new(IoErrorKind::Unsupported, ERROR)));
		},
	};

	// get the current platform architecture
	let arch = match env::consts::ARCH {
		"x86_64" => "x64",
		"arm" => "arm",
		"arm64" => "arm64",
		"ia32" => "ia32",
		_ => {
			const ERROR: &str = "ERROR: unknown/unsupported CPU architecture";
			log(ERROR);
			return Err(Box::new(IoError::new(IoErrorKind::Unsupported, ERROR)));
		},
	};

	// attempt to download the sdk from the official dart mirror

	// Official dart mirror url with platform and architecture
	let dart_sdk_download_url: String = format!(
		"https://storage.googleapis.com/dart-archive/channels/{_channel}/release/latest/sdk/dartsdk-{_platform}-{_arch}-release.zip",
		_channel = channel.to_string(),
		_platform = platform,
		_arch = arch,
	);

	// SHA256 hash to check integrity of the sdk url
	let dart_sdk_shasum_download_url: String = format!(
		"https://storage.googleapis.com/dart-archive/channels/{_channel}/release/latest/sdk/dartsdk-{_platform}-{_arch}-release.zip.sha256sum",
		_channel = channel.to_string(),
		_platform = platform,
		_arch = arch,
	);

	/// use reqwest to download the sdk and the shasum synchronously
	fn download<T>(url: T) -> Result<(), Box<dyn StdError>>
	where T: reqwest::IntoUrl {
		log(&format!("INFO: downloading \"{}\"", url.as_str()));
		log("INFO: Download will take a while. This is normal.");

		let is_shasum = url.as_str().contains(".sha256sum");

		// attempt to download the sdk synchronously from the official dart mirror
		// We have to use a custom synchronous client so that the request time out

		// create a synchronous client
		let client = reqwest::blocking::Client::builder()
		// timeout after 3 minutes
			.timeout(Duration::from_secs(180))
			.build()?;

		// send a get request to the url
		let response = client.get(url).send()?;

		// check if the response is successful
		match response.status() {
			StatusCode::OK => {
				let cargo_home = env::var("CARGO_HOME").expect("Could not find $CARGO_HOME variable.");

				// path to write the resource to
				let file_path = if is_shasum {
					format!("{}/dart-sdk.zip.sha256sum", cargo_home)
				} else {
					format!("{}/dart-sdk.zip", cargo_home)
				};

				// create a file to write the response to, if it doesn't exist
				// if the path does exist, emit a warning and delete the file/directory.
				let mut file = match File::create(&file_path) {
					Ok(file) => file,
					Err(e) => {
						// if the path exists, delete it
						if e.kind() == IoErrorKind::AlreadyExists {
							log(&format!("WARNING: \"{}\" already exists. Deleting", &file_path));
							// do NOT emit this warning if the `ci` feature is enabled
							// because we run a strict CI, any warnings will cause the CI to fail
							#[cfg(not(feature = "ci"))]
							println!("cargo:warning=\"{}\" already exists. Deleting", &file_path);

							// delete the file/directory
							fs::remove_file(&file_path)?;

							// try to create the file again
							File::create(&file_path)?
						} else {
							return Err(Box::new(e));
						}
					},
				};

				// write the response to the file
				file.write_all(&response.bytes()?)?;

				Ok(log("INFO: Successfully downloaded resource"))
			},
			// If response is not successful, return the respective error
			_ => {
				let http_error = format!("ERROR: HTTP error: {}", response.status());
				let error = format!("ERROR: could not download: {{{}}}", http_error);

				log(&error);
				Err(Box::new(IoError::new(IoErrorKind::Other, error)))
			},
		}
	}

	/// Checks the integrity of the downloaded sdk using SHA-256 hash
	///
	/// ## Arguments:
	/// * `file_path`: path to the the file to check the integrity of
	/// * `hash_path`: the path to the SHA-256 hash file to check the integrity of the file against
	fn check_sha256_checksum(file_path: &str, hash_path: &str) -> Result<(), Box<dyn StdError>> {
		log(&format!("INFO: checking integrity of \"{}\"", file_path));

		// Open file to check the integrity of
		let mut file = File::open(file_path)?;
		// Open file containing the SHA-256 hash
		let mut hash_file = File::open(hash_path)?;

		// Create a buffer to read the file into
		let mut file_buffer = Vec::new();
		// Create a buffer to read the hash file into
		let mut hash_file_buffer = Vec::new();

		// Read the file contents into the empty buffer
		file.read_to_end(&mut file_buffer)?;
		// Read the hash file contents into the empty buffer
		hash_file.read_to_end(&mut hash_file_buffer)?;

		// Convert the expected hash to a string
		let expected_hash = String::from_utf8(hash_file_buffer)?;

		// Create a new Sha256 object
		let mut hasher = Sha256::new();

		// Write the file buuffer to the Sha256 object
		hasher.update(&file_buffer);

		// get the final hash of the file
		let actual_hash = hasher.finalize();

		// convert the hash to a string
		let acutal_hash = format!("{:x}", actual_hash);

		// check if the expected hash includes the actual hash
		// ? the Dart SDK hash file includes the file name, so we have to check if the actual hash is
		// ? included in the expected hash
		if expected_hash.contains(&acutal_hash) {
			Ok(log("INFO: integrity check successful"))
		} else {
			let error = format!(
				"ERROR: integrity check failed. Expected hash: `{}`, Actual hash: `{}`",
				expected_hash, acutal_hash
			);
			log(&error);
			Err(Box::new(IoError::new(IoErrorKind::Other, error)))
		}
	}

	/// Unzips a file using the zip algorithm
	///
	/// (does NOT work with tar.gz/.tar/.gz/.tar.7z/.7z files)
	///
	/// ## Arguments:
	/// * `file_path`: path to the file to unzip
	/// * `destination`: path to the destination to unzip the file/directory to
	fn unzip_file(file_path: &str, destination: &str) -> Result<(), Box<dyn StdError>> {
		log("INFO: attempting to unzip Dart SDK");
		let file = File::open(file_path)?;
		let mut archive = ZipArchive::new(file)?;

		for i in 0..archive.len() {
			let mut file = archive.by_index(i)?;
			let outpath = Path::new(destination).join(file.name());

			if file.name().ends_with('/') {
				fs::create_dir_all(&outpath)?;
			} else {
				if let Some(p) = outpath.parent() {
					if !p.exists() {
						fs::create_dir_all(&p)?;
					}
				}
				if outpath.exists() {
					fs::remove_file(&outpath)?;
				}
				let mut outfile = File::create(&outpath)?;
				io::copy(&mut file, &mut outfile)?;
			}
		}

		Ok(log("INFO: successfully unzipped Dart SDK"))
	}

	let cargo_home = env::var("CARGO_HOME").expect("Could not find $CARGO_HOME variable.");

	// attempt to download the sdk and shasum
	let dart_sdk_download_res = download(dart_sdk_download_url);
	let dart_sdk_shasum_download_res = download(dart_sdk_shasum_download_url);

	// check if the sdk and shasum were downloaded successfully
	if dart_sdk_download_res.is_ok() {
		// if the sdk was downloaded successfully, check if the shasum was downloaded successfully
		if dart_sdk_shasum_download_res.is_ok() {
			// if the shasum was downloaded successfully, check if the shasum is valid
			if check_sha256_checksum(
				&format!("{}/dart-sdk.zip", cargo_home),
				&format!("{}/dart-sdk.zip.sha256sum", cargo_home),
			)
			.is_ok()
			{
				// if the shasum is valid, return the path to the sdk
				log("INFO: successfully downloaded Dart SDK");
				if unzip_file(
					&format!("{}/dart-sdk.zip", cargo_home),
					&format!("{}/dart-sdk", cargo_home),
				)
				.is_ok()
				{
					log("INFO: successfully unzipped Dart SDK");
					return Ok(format!("{}/dart-sdk/dart-sdk", cargo_home));
				} else {
					// return the respective error
					let error = unzip_file(
						&format!("{}/dart-sdk.zip", cargo_home),
						&format!("{}/dart-sdk", cargo_home),
					)
					.unwrap_err();
					log(&format!("ERROR: failed to unzip Dart SDK: {{{}}}", error));
					return Err(error);
				}
			} else {
				// return the respective error
				let error = check_sha256_checksum(
					&format!("{}/dart-sdk.zip", cargo_home),
					&format!("{}/dart-sdk.zip.sha256sum", cargo_home),
				)
				.unwrap_err();
				log(&format!("ERROR: failed to check shasum: {{{}}}", error));
				return Err(error);
			}
		} else {
			// return the respective error
			let error = dart_sdk_shasum_download_res.unwrap_err();
			log(&format!("ERROR: failed to download Dart SDK shasum: {{{}}}", error));
			return Err(error);
		}
	} else {
		// return the respective error
		let error = dart_sdk_download_res.unwrap_err();
		log(&format!("ERROR: failed to download Dart SDK: {{{}}}", error));
		return Err(error);
	}
}

/// Retuns the path to the preferred Dart SDK.
///
/// If no sdk is found locally, it will download the sdk from the specified channel.
///
/// If an sdk CANNOT be found, it will panic and exit compilation instead of returning an error.
///
/// ## Arguments:
///
/// * `channel`: the channel to download the sdk from, if no sdk is found locally
#[cfg(not(feature = "download_dart_sdk"))]
fn get_dart_sdk(channel: DartSdkChannel) -> String {
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
					// if we can't find a local sdk, try to download one
					log("INFO: failed to find local flutter sdk, attempting to download the official dart-sdk");
					match download_dart_sdk(channel) {
						Ok(x) => x,
						Err(e) => {
							// log error and panic
							log(&format!("{}", e));
							panic!("{}", e);
						},
					}
				},
			}
		},
	};
	dart_sdk
}

/// returns the selected channel to download the Dart SDK from.
///
/// If one is not specified, defaults to the stable channel
#[allow(unreachable_code)]
fn get_dart_sdk_channel() -> DartSdkChannel {
	// if all of the download_dart_sdk_* features are enabled (caused by `--all-features`), log error
	// and return stable
	#[cfg(all(
		feature = "download_dart_sdk_stable",
		feature = "download_dart_sdk_beta",
		feature = "download_dart_sdk_dev"
	))]
	{
		const WARNING: &str = "WARNING: more than one `download_dart_sdk_*` feature is enabled, defaulting to stable";
		log(WARNING);
		// do NOT emit this warning if the `ci` feature is enabled
		// because we run a strict CI, any warnings will cause the CI to fail
		#[cfg(not(feature = "ci"))]
		println!("cargo:warning={}", WARNING);
		return DartSdkChannel::Stable;
	}

	#[cfg(feature = "download_dart_sdk_stable")]
	return DartSdkChannel::Stable;

	#[cfg(feature = "download_dart_sdk_beta")]
	return DartSdkChannel::Beta;

	#[cfg(feature = "download_dart_sdk_dev")]
	return DartSdkChannel::Dev;

	#[cfg(not(any(
		feature = "download_dart_sdk_stable",
		feature = "download_dart_sdk_beta",
		feature = "download_dart_sdk_dev"
	)))]
	return DartSdkChannel::Stable;
}

/// Emits the compiler flags for `cargo build`
fn emit_compiler_flags() {
	log("INFO: emitting compiler flags");
	// get Dart SDK path for linking
	#[cfg(not(feature = "download_dart_sdk"))]
	let _dart_sdk_path = get_dart_sdk(get_dart_sdk_channel());
	#[cfg(feature = "download_dart_sdk")]
	let _dart_sdk_path = download_dart_sdk(get_dart_sdk_channel()).unwrap();
	// convert dart_sdk_path to PathBuf
	let dart_sdk_path = PathBuf::from(&_dart_sdk_path);

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

	let dart_sdk_header_wrapper = PathBuf::from("./bindgen/dart_sdk_wrapper.h");

	// Ensure that the Dart SDK header wrapper exists
	if !dart_sdk_header_wrapper.exists() {
		let error = &format!(
			"ERROR: Dart SDK header wrapper not found at \"{}\". Please ensure that Dart-sys is not corrupt. Proceed \
			 with caution.",
			dart_sdk_header_wrapper.to_str().unwrap()
		);

		log(error);
		panic!("{}", error);
	}

	let bindings = bindgen::Builder::default()
		.header("./bindgen/dart_sdk_wrapper.h")
		.clang_arg(format!(
			"--include-directory={}",
			dart_sdk_path.join("include").to_str().unwrap()
		))
		.clang_arg("-DDART_SHARED_LIB")
		.generate()
		.expect("ERROR: bindgen failed to generate bindings");
	let out_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("ERROR: Could not find $CARGO_MANIFEST_DIR"));
	bindings
		.write_to_file(out_path.join("src/bindings/mod.rs"))
		.expect("ERROR: failed to write bindings to file");

	log("INFO: finished emitting compiler flags");
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

	// if the `docs_only` feature is not enabled, emit compiler flags
	// if `--all-features` is passed, run anyway
	// if `docs_only` is combined with any other feature, run anyway
	if !cfg!(feature = "docs_only") ||
		cfg!(all(feature = "docs_only", feature = "download_dart_sdk")) ||
		cfg!(all(feature = "docs_only", feature = "download_dart_sdk_stable")) ||
		cfg!(all(feature = "docs_only", feature = "download_dart_sdk_beta")) ||
		cfg!(all(feature = "docs_only", feature = "download_dart_sdk_dev"))
	{
		emit_compiler_flags();
	}

	log("INFO: finished build script");
	log("------------------------------");
}
