use std::{
	env,
	error::Error as StdError,
	fs::{self, File},
	io::{self, prelude::*, Error as IoError, ErrorKind as IoErrorKind},
	path::Path,
};

use sha2::{Digest, Sha256};

use crate::{
	log::LogLevel,
	utils::paths::{dart_sdk_path, temp_dir},
};

/// files to remove after downloading and unzipping the Dart SDK
const REMOVE_FILES: [&str; 1] = ["dartdoc_options.yaml"];

/// directories to remove after downloading and unzipping the Dart SDK
const REMOVE_DIRS: [&str; 3] = ["bin/snapshots", "bin/resources", "lib"];

/// Downloads the Dart SDK and unzips it to `dart_sdk_path()`
///
/// # Returns
///
/// * `Ok(())` if the Dart SDK was successfully downloaded and unzipped
/// * `Err(String)` if the Dart SDK failed to download or unzip
///
/// # Examples
///
/// ```
/// match update_dart_sdk::update_dart_sdk() {
///    Ok(_) => println!("Successfully updated Dart SDK"),
///   Err(e) => println!("Failed to update Dart SDK: {}", e),
/// }
/// ```
pub fn update_dart_sdk() -> Result<(), String> {
	log!("updating Dart SDK");
	let dart_sdk_zip_path = temp_dir().join("dart-sdk.zip");
	let dart_sdk_zip_path = dart_sdk_zip_path.to_str().unwrap();

	let dart_sdk_shasum_path = temp_dir().join("dart-sdk.zip.sha256sum");
	let dart_sdk_shasum_path = dart_sdk_shasum_path.to_str().unwrap();

	// get the current platform
	let platform = match env::consts::OS {
		"linux" => "linux",
		"macos" => "macos",
		"windows" => "windows",
		_ => {
			log!(LogLevel::Error, "ERROR: unknown/unsupported OS");
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
			log!(LogLevel::Error, "ERROR: unknown/unsupported CPU architecture");
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

	log!("Creating Dart SDK directory");

	if dart_sdk_path().exists() {
		log!(
			LogLevel::Info,
			format!(
				"Attempting to remove current Dart SDK: {:?}",
				dart_sdk_path().to_str().unwrap()
			)
		);
		match fs::remove_dir_all(&dart_sdk_path()) {
			Ok(_) => {
				log!(LogLevel::Success, "Successfully removed current Dart SDK");
				log!(format!(
					"Attempting to create Dart SDK directory: {:?}",
					dart_sdk_path().to_str().unwrap()
				));

				match fs::create_dir(&dart_sdk_path()) {
					Ok(_) => {
						log!(LogLevel::Success, "Successfully created Dart SDK directory");
					},
					Err(e) => {
						log!(LogLevel::Error, format!("Failed to create Dart SDK directory: {:?}", e));
						panic!("ERROR: Failed to create Dart SDK directory");
					},
				};
			},
			Err(e) => {
				log!(LogLevel::Error, format!("Failed to remove current Dart SDK: {:?}", e));
				panic!("ERROR: Failed to remove current Dart SDK");
			},
		};
	} else {
		match fs::create_dir(&dart_sdk_path()) {
			Ok(_) => {
				log!(
					LogLevel::Success,
					format!(
						"Successfully created Dart SDK directory: {:?}",
						dart_sdk_path().to_str().unwrap()
					)
				);
			},
			Err(e) => {
				log!(LogLevel::Error, format!("Failed to create Dart SDK directory: {:?}", e));
				panic!("ERROR: Failed to create Dart SDK directory");
			},
		};
	}

	log!("Creating temp directory");

	if temp_dir().exists() {
		log!(
			LogLevel::Info,
			format!(
				"Attempting to existing remove temp directory: {:?}",
				temp_dir().to_str().unwrap()
			)
		);
		match fs::remove_dir_all(&temp_dir()) {
			Ok(_) => {
				log!(LogLevel::Success, "Found old temp directory, removed it");

				log!(format!(
					"Attempting to create temp directory: {:?}",
					temp_dir().to_str().unwrap()
				));

				match fs::create_dir(&temp_dir()) {
					Ok(_) => {
						log!(LogLevel::Success, "Successfully created temp directory");
					},
					Err(e) => {
						log!(LogLevel::Error, format!("Failed to create temp directory: {:?}", e));
						panic!("ERROR: Failed to create temp directory");
					},
				};
			},
			Err(e) => {
				log!(LogLevel::Error, format!("Failed to remove temp directory: {:?}", e));
				panic!("ERROR: Failed to remove temp directory");
			},
		};
	} else {
		match fs::create_dir(&temp_dir()) {
			Ok(_) => {
				log!(LogLevel::Success, "Successfully created temp directory");
			},
			Err(e) => {
				log!(LogLevel::Error, format!("Failed to create temp directory: {:?}", e));
				panic!("ERROR: Failed to create temp directory");
			},
		};
	}

	log!("Downloading Dart SDK zip archive");

	match download(dart_sdk_download_url, dart_sdk_zip_path) {
		Ok(_) => log!(LogLevel::Success, "Successfully downloaded Dart SDK zip archive"),
		Err(e) => {
			log!(
				LogLevel::Error,
				format!("Failed to download Dart SDK zip archive: {}", e)
			);
			panic!("ERROR: Failed to download Dart SDK zip archive: {}", e);
		},
	}

	log!("Downloading Dart SDK SHA-256 hash");

	match download(dart_sdk_shasum_download_url, dart_sdk_shasum_path) {
		Ok(_) => log!(LogLevel::Success, "Successfully downloaded Dart SDK SHA-256 hash"),
		Err(e) => {
			log!(
				LogLevel::Error,
				format!("Failed to download Dart SDK SHA-256 hash: {}", e)
			);
			panic!("ERROR: Failed to download Dart SDK SHA-256 hash: {}", e);
		},
	}

	log!("Checking integrity of Dart SDK zip archive");

	match check_sha256_checksum(dart_sdk_zip_path, dart_sdk_shasum_path) {
		Ok(_) => {
			log!(
				LogLevel::Success,
				"Successfully checked integrity of Dart SDK zip archive"
			)
		},
		Err(e) => {
			log!(
				LogLevel::Error,
				format!("Failed to check integrity of Dart SDK zip archive: {}", e)
			);
			panic!("ERROR: Failed to check integrity of Dart SDK zip archive: {}", e)
		},
	}

	log!("Unzipping Dart SDK zip archive");

	match unzip_file(dart_sdk_zip_path, dart_sdk_path().parent().unwrap().to_str().unwrap()) {
		Ok(_) => log!(LogLevel::Success, "Successfully unzipped Dart SDK zip archive"),
		Err(e) => {
			log!(LogLevel::Error, format!("Failed to unzip Dart SDK zip archive: {}", e));
			panic!("ERROR: Failed to unzip Dart SDK zip archive: {}", e);
		},
	}

	log!("Removing unused Dart SDK files");

	for entry in REMOVE_FILES {
		let path = dart_sdk_path().join(entry);
		if path.exists() {
			log!(
				LogLevel::Info,
				format!("Attempting to remove unused Dart SDK file: {:?}", path)
			);
			match fs::remove_file(&path) {
				Ok(_) => log!(LogLevel::Success, "Successfully removed file"),
				Err(e) => {
					log!(LogLevel::Error, format!("Failed to remove file: {}", e));
					panic!("ERROR: Failed to remove file: {}", e)
				},
			};
		}
	}

	for entry in REMOVE_DIRS {
		let path = dart_sdk_path().join(entry);
		if path.exists() {
			log!(
				LogLevel::Info,
				format!("Attempting to remove unused Dart SDK directory: {:?}", path)
			);
			match fs::remove_dir_all(&path) {
				Ok(_) => log!(LogLevel::Success, "Successfully removed directory"),
				Err(e) => {
					log!(LogLevel::Error, format!("Failed to remove directory: {}", e));
					panic!("ERROR: Failed to remove directory: {}", e)
				},
			};
		}
	}

	log!(LogLevel::Success, "Successfully removed unused Dart SDK files");

	log!("Removing temporary files");

	match fs::remove_dir_all(temp_dir()) {
		Ok(_) => log!("Successfully removed temporary files"),
		Err(e) => {
			log!(LogLevel::Error, format!("Failed to remove temporary files: {}", e));
			panic!("ERROR: Failed to remove temporary files: {}", e);
		},
	};
	log!(LogLevel::Success, "Successfully updated Dart SDK");

	Ok(())
}

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
	log!(format!(
		"Downloading url: \"{}\" (this may take a while, this is normal.)",
		url.as_str()
	));
	// Download the file
	let mut resp = reqwest::blocking::get(url)?;

	log!(LogLevel::Success, "Downloaded successfully");

	// Create a buffer to read the file into
	let mut buffer = Vec::new();
	// Read the response into the buffer
	resp.copy_to(&mut buffer)?;

	log!(format!("Writing file to: \"{}\"", dest));

	// Create a file to write the buffer to
	let mut file = File::create(dest)?;
	// Write the buffer to the file
	file.write_all(&buffer)?;

	log!(LogLevel::Success, format!("Successfully wrote file to: \"{}\"", dest));

	Ok(())
}

/// Checks the integrity of a downloaded file using the SHA-256 hash algorithm
///
/// ## Arguments:
///
/// * `file_path`: path to the the file to check the integrity of
/// * `hash_path`: the path to the SHA-256 hash file to check the integrity of the file against
fn check_sha256_checksum(file_path: &str, hash_path: &str) -> Result<(), Box<dyn StdError>> {
	log!(format!("checking integrity of \"{}\"", file_path));
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
		log!("integrity check successful");
		Ok(())
	} else {
		let error = format!(
			"integrity check failed. Expected hash: `{}`, Actual hash: `{}`",
			expected_hash, acutal_hash
		);
		log!(LogLevel::Error, error.as_str());
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
	log!(format!(
		"attempting to unzip file \"{}\" to \"{}\"",
		file_path, destination
	));
	let file = match File::open(file_path) {
		Ok(file) => file,
		Err(e) => {
			log!(LogLevel::Error, format!("Failed to open zip file: {}", e));
			panic!("ERROR: Failed to open zip file: {}", e);
		},
	};
	let mut archive = match zip::ZipArchive::new(file) {
		Ok(archive) => archive,
		Err(e) => {
			log!(LogLevel::Error, format!("Failed to read zip file: {}", e));
			panic!("ERROR: Failed to read zip file: {}", e);
		},
	};
	for i in 0..archive.len() {
		let mut file = match archive.by_index(i) {
			Ok(file) => file,
			Err(e) => {
				log!(LogLevel::Error, format!("Failed to read file: {}", e));
				panic!("ERROR: Failed to read file: {}", e);
			},
		};
		let outpath = Path::new(destination).join(file.name());
		if file.name().ends_with('/') {
			match fs::create_dir_all(&outpath) {
				Ok(_) => (),
				Err(e) => {
					log!(LogLevel::Error, format!("Failed to create directory: {}", e));
					panic!("ERROR: Failed to create directory: {}", e);
				},
			}
		} else {
			if let Some(p) = outpath.parent() {
				if !p.exists() {
					match fs::create_dir_all(p) {
						Ok(_) => (),
						Err(e) => {
							log!(LogLevel::Error, format!("Failed to create directory: {}", e));
							panic!("ERROR: Failed to create directory: {}", e);
						},
					}
				}
			}
			if outpath.exists() {
				match fs::remove_file(&outpath) {
					Ok(_) => (),
					Err(e) => {
						log!(LogLevel::Error, format!("Failed to remove file: {}", e));
						panic!("ERROR: Failed to remove file: {}", e);
					},
				}
			}
			let mut outfile = match File::create(&outpath) {
				Ok(f) => f,
				Err(e) => {
					log!(LogLevel::Error, format!("Failed to create file: {}", e));
					panic!("ERROR: Failed to create file: {}", e);
				},
			};

			match io::copy(&mut file, &mut outfile) {
				Ok(_) => (),
				Err(e) => {
					log!(LogLevel::Error, format!("Failed to copy file: {}", e));
					panic!("ERROR: Failed to copy file: {}", e);
				},
			}
		}
	}
	Ok(())
}
