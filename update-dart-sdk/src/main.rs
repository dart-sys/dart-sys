//! A small rust script for updating the Dart SDK.

use std::{
	env,
	error::Error as StdError,
	fs::{self, File},
	io::{self, prelude::*, Error as IoError, ErrorKind as IoErrorKind},
	path::{Path, PathBuf},
};

use chrono::{DateTime, SecondsFormat, Utc};
use sha2::{Digest, Sha256};

/// Logger macro for the build script.
/// # Example
///
/// ```rust
///
/// log!("hello world");
/// log!("hi, {}", "there");
/// log!("{}", &error);
/// ```
///
/// ```bash
///
/// > cat logfile.log
/// ```
/// ```console
///
/// INFO: hello world
/// INFO: hi, there
/// ERROR: <error message>
/// ```
#[macro_export]
macro_rules! log {
	($($arg:tt)*) => ({
		let msg = format!($($arg)*);
		let now: DateTime<Utc> = Utc::now();
		let now = now.to_rfc3339_opts(SecondsFormat::Secs, true);
		let mut file = fs::OpenOptions::new()
			.append(true)
			.create(true)
			.open(PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("update-dart-sdk.log"))
			.unwrap();
		let message = format!("[{}]: {}\n", now, msg);
		file.write_all(message.as_bytes()).unwrap();
		print!("{}", message);
	});
}

fn main() {
	log!("--------------------------------------------");
	log!("INFO: Running update-dart-sdk script");

	let dart_sdk_dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
		.join("..")
		.join("dart-sdk");
	let temp_dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("temp");

	let dart_sdk_zip_path = temp_dir.join("dart-sdk.zip");
	let dart_sdk_zip_path = dart_sdk_zip_path.to_str().unwrap();

	let dart_sdk_shasum_path = temp_dir.join("dart-sdk.zip.sha256sum");
	let dart_sdk_shasum_path = dart_sdk_shasum_path.to_str().unwrap();

	println!("parent_dart_sdk_dir: {}", dart_sdk_dir.display());
	println!("dart_sdk_dir: {}", dart_sdk_dir.display());

	let platform = match env::consts::OS {
		"linux" => "linux",
		"macos" => "macos",
		"windows" => "windows",
		_ => {
			panic!("ERROR: unknown/unsupported OS");
		},
	};

	// get the current platform architecture
	let arch = match env::consts::ARCH {
		"x86_64" => "x64",
		"arm" => "arm",
		"arm64" => "arm64",
		"ia32" => "ia32",
		_ => {
			panic!("ERROR: unknown/unsupported CPU architecture");
		},
	};

	// Official dart mirror url with platform and architecture
	let dart_sdk_download_url: String = format!(
		"https://storage.googleapis.com/dart-archive/channels/stable/release/latest/sdk/dartsdk-{_platform}-{_arch}-release.zip",
		_platform = platform,
		_arch = arch,
	);

	// SHA256 hash to check integrity of the sdk url
	let dart_sdk_shasum_download_url: String = format!("{}.sha256sum", dart_sdk_download_url);

	/// Uses reqwest to download a file from the given url and writes it to the given path.
	///
	/// ## Arguments
	///
	/// * `url` - The url to download the file from.
	/// * `dest` - the destination path to write the file to.
	///
	/// ## Note:
	///
	/// This function is synchronous and will block the current thread until the download is
	/// complete.
	///
	/// This function is also only designed for downloading zip archives and sha256sum files.
	/// Using it for other file types may result in unexpected behavior.
	fn download<T, U>(url: T, dest: &U) -> Result<(), Box<dyn StdError>>
	where
		T: reqwest::IntoUrl,
		U: AsRef<Path>+std::fmt::Display+?Sized, {
		log!(
			"INFO: Downloading url: \"{}\" (this may take a while, this is normal.)",
			url.as_str(),
		);
		// Download the file
		let mut resp = reqwest::blocking::get(url)?;

		log!("INFO: Downloaded successfully");

		// Create a buffer to read the file into
		let mut buffer = Vec::new();
		// Read the response into the buffer
		resp.copy_to(&mut buffer)?;

		log!("INFO: Writing file to: \"{}\"", dest);

		// Create a file to write the buffer to
		let mut file = File::create(dest)?;
		// Write the buffer to the file
		file.write_all(&buffer)?;

		log!("INFO: Successfully wrote file to: \"{}\"", dest);

		Ok(())
	}

	/// Checks the integrity of a downloaded file using the SHA-256 hash algorithm
	///
	/// ## Arguments:
	///
	/// * `file_path`: path to the the file to check the integrity of
	/// * `hash_path`: the path to the SHA-256 hash file to check the integrity of the file against
	fn check_sha256_checksum(file_path: &str, hash_path: &str) -> Result<(), Box<dyn StdError>> {
		log!("INFO: checking integrity of \"{}\"", file_path);
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
			Ok(log!("INFO: integrity check successful"))
		} else {
			let error = format!(
				"ERROR: integrity check failed. Expected hash: `{}`, Actual hash: `{}`",
				expected_hash, acutal_hash
			);
			log!("{}", error);
			Err(Box::new(IoError::new(IoErrorKind::Other, error)))
		}
	}

	/// Unzips a file using the zip algorithm
	///
	/// (does NOT work with tar.gz/.tar/.gz/.tar.7z/.7z files)
	///
	/// ## Arguments:
	///
	/// * `file_path`: path to the file to unzip
	/// * `destination`: path to the destination to unzip the file/directory to
	fn unzip_file(file_path: &str, destination: &str) -> Result<(), Box<dyn StdError>> {
		log!(
			"INFO: attempting to unzip file \"{}\" to \"{}\"",
			file_path,
			destination,
		);
		let file = File::open(file_path)?;
		let mut archive = zip::ZipArchive::new(file)?;
		for i in 0..archive.len() {
			let mut file = archive.by_index(i)?;
			let outpath = Path::new(destination).join(file.name());
			if file.name().ends_with('/') {
				fs::create_dir_all(&outpath)?;
			} else {
				if let Some(p) = outpath.parent() {
					if !p.exists() {
						fs::create_dir_all(p)?;
					}
				}
				if outpath.exists() {
					fs::remove_file(&outpath)?;
				}
				let mut outfile = File::create(&outpath)?;
				io::copy(&mut file, &mut outfile)?;
			}
		}
		Ok(())
	}

	log!("INFO: Creating Dart SDK directory");

	if dart_sdk_dir.exists() {
		log!("INFO: Attempting to remove current Dart SDK: {:?}", dart_sdk_dir);
		match fs::remove_dir_all(&dart_sdk_dir) {
			Ok(_) => {
				log!("INFO: Found old Dart SDK, removed it");

				log!("INFO: Attempting to create Dart SDK directory: {:?}", &dart_sdk_dir,);

				match fs::create_dir(&dart_sdk_dir) {
					Ok(_) => {
						log!("INFO: Successfully created Dart SDK directory");
					},
					Err(e) => {
						log!("ERROR: Failed to create Dart SDK directory: {:?}", e);
						panic!("ERROR: Failed to create Dart SDK directory");
					},
				};
			},
			Err(e) => {
				log!("ERROR: Failed to remove current Dart SDK: {:?}", e);
				panic!("ERROR: Failed to remove current Dart SDK");
			},
		};
		log!("INFO: Successfully removed current Dart SDK");
	} else {
		match fs::create_dir(&dart_sdk_dir) {
			Ok(_) => {
				log!("INFO: Successfully created Dart SDK directory");
			},
			Err(e) => {
				log!("ERROR: Failed to create Dart SDK directory: {:?}", e);
				panic!("ERROR: Failed to create Dart SDK directory");
			},
		};
	}

	log!("INFO: Creating temp directory");

	if temp_dir.exists() {
		log!("INFO: Attempting to remove temp directory: {:?}", temp_dir);
		match fs::remove_dir_all(&temp_dir) {
			Ok(_) => {
				log!("INFO: Found old temp directory, removed it");

				log!("INFO: Attempting to create temp directory: {:?}", temp_dir);

				match fs::create_dir(&temp_dir) {
					Ok(_) => {
						log!("INFO: Successfully created temp directory");
					},
					Err(e) => {
						log!("ERROR: Failed to create temp directory: {:?}", e);
						panic!("ERROR: Failed to create temp directory");
					},
				};
			},
			Err(e) => {
				log!("ERROR: Failed to remove temp directory: {:?}", e);
				panic!("ERROR: Failed to remove temp directory");
			},
		};
	} else {
		match fs::create_dir(&temp_dir) {
			Ok(_) => {
				log!("INFO: Successfully created temp directory");
			},
			Err(e) => {
				log!("ERROR: Failed to create temp directory: {:?}", e);
				panic!("ERROR: Failed to create temp directory");
			},
		};
	}

	log!("INFO: attempting to download Dart SDK (this may take a while)");

	log!("INFO: Downloading Dart SDK zip archive");

	match download(dart_sdk_download_url, dart_sdk_zip_path) {
		Ok(_) => log!("INFO: Successfully downloaded Dart SDK zip archive"),
		Err(e) => panic!("ERROR: Failed to download Dart SDK zip archive: {}", e),
	}

	log!("INFO: Downloading Dart SDK SHA-256 hash");

	match download(dart_sdk_shasum_download_url, dart_sdk_shasum_path) {
		Ok(_) => log!("INFO: Successfully downloaded Dart SDK SHA-256 hash"),
		Err(e) => panic!("ERROR: Failed to download Dart SDK SHA-256 hash: {}", e),
	}

	log!("INFO: Checking integrity of Dart SDK zip archive");

	match check_sha256_checksum(dart_sdk_zip_path, dart_sdk_shasum_path) {
		Ok(_) => log!("INFO: Successfully checked integrity of Dart SDK zip archive"),
		Err(e) => panic!("ERROR: Failed to check integrity of Dart SDK zip archive: {}", e),
	}

	log!("INFO: Unzipping Dart SDK zip archive");

	match unzip_file(
		temp_dir.join("dart-sdk.zip").to_str().unwrap(),
		dart_sdk_dir.join("..").to_str().unwrap(),
	) {
		Ok(_) => log!("INFO: Successfully unzipped Dart SDK zip archive"),
		Err(e) => panic!("ERROR: Failed to unzip Dart SDK zip archive: {}", e),
	}

	log!("INFO: Moving Dart SDK to repository root");

	log!("INFO: Removing unused Dart SDK files");

	match fs::remove_dir_all(Path::new("..").join("dart-sdk").join("bin")) {
		Ok(_) => log!("INFO: Successfully removed unused Dart SDK files"),
		Err(e) => panic!("ERROR: Failed to remove unused Dart SDK files: {}", e),
	};

	match fs::remove_dir_all(Path::new("..").join("dart-sdk").join("lib")) {
		Ok(_) => log!("INFO: Successfully removed unused Dart SDK files"),
		Err(e) => panic!("ERROR: Failed to remove unused Dart SDK files: {}", e),
	};

	match fs::remove_file(Path::new("..").join("dart-sdk").join("dartdoc_options.yaml")) {
		Ok(_) => log!("INFO: Successfully removed unused Dart SDK files"),
		Err(e) => panic!("ERROR: Failed to remove unused Dart SDK files: {}", e),
	};

	log!("INFO: Removing temporary files");

	match fs::remove_dir_all(temp_dir) {
		Ok(_) => log!("INFO: Successfully removed temporary files"),
		Err(e) => panic!("ERROR: Failed to remove temporary files: {}", e),
	};
	log!("INFO: Finished update-dart-sdk script");
	log!("--------------------------------------------");
}
