//! A small rust script for updating the Dart SDK.

use std::{
	env,
	error::Error as StdError,
	fs::{self, File},
	io::{self, prelude::*, Error as IoError, ErrorKind as IoErrorKind},
	path::Path,
};

use sha2::{Digest, Sha256};

mod fs_utils {
	use std::{fs, path::Path};

	/// Copies all files and directories from one directory to another.
	/// If the destination directory does not exist, it will be created.
	/// If the source directory does not exist, this function will panic.
	/// If a file fails to copy, this function will panic.
	///
	/// ## Arguments:
	/// * `src_dir` - The source directory to copy from.
	/// * `dest_dir` - The destination directory to copy to.
	pub fn copy_dir(src_dir: &Path, dest_dir: &Path) {
		for src_entry in src_dir
			.read_dir()
			.unwrap_or_else(|e| panic!("Copying files failed: {}\n{}", src_dir.display(), e))
		{
			let src_entry = src_entry.unwrap();
			let src_type = src_entry.file_type().unwrap();
			let src_path = &src_entry.path();
			let dest_path = &dest_dir.join(src_entry.file_name());
			if src_type.is_dir() {
				create_dir(dest_path);
				copy_dir(src_path, dest_path);
			} else if src_type.is_file() {
				match fs::copy(src_path, dest_path) {
					Ok(_) => {},
					Err(e) => {
						panic!("ERROR: Failed to copy file: \"{}\"{{{}}}", src_path.display(), e);
					},
				};
			}
		}
	}

	/// Creates a directory if it does not already exist.
	/// If the directory already exists, this function will do nothing.
	///
	/// ## Arguments:
	/// * `name` - The name of the directory to create.
	pub fn create_dir(name: &Path) {
		if !name.is_dir() {
			fs::create_dir(name).unwrap_or_else(|e| panic!("ERROR: Failed to create dir: {}\n{}", name.display(), e));
		}
	}

	/// Moves a directory from one location to another, along with all of its contents.
	/// If the destination directory already exists, it will be deleted and recreated.
	/// If the source directory does not exist, this function will panic.
	///
	/// ## Arguments:
	/// * `src_dir` - The source directory to move.
	/// * `dest_dir` - The destination directory to move to.
	pub fn move_dir(src_dir: &Path, dest_dir: &Path) {
		if src_dir.is_dir() {
			if dest_dir.exists() {
				fs::remove_dir_all(dest_dir)
					.unwrap_or_else(|e| panic!("ERROR: Failed to remove dir: {}\n{}", dest_dir.display(), e));
			}
			fs::rename(src_dir, dest_dir)
				.unwrap_or_else(|e| panic!("ERROR: Failed to move dir: {}\n{}", src_dir.display(), e));
		} else {
			panic!("ERROR: Source directory does not exist: {}", src_dir.display());
		}
	}
}

fn main() {
	println!("INFO: Running update-dart-sdk script");

	let dart_sdk_dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("dart-sdk");
	let dart_sdk_temp_dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("temp");

	let dart_sdk_zip_path = dart_sdk_temp_dir.join("dart-sdk.zip");
	let dart_sdk_zip_path = dart_sdk_zip_path.to_str().unwrap();

	let dart_sdk_shasum_path = dart_sdk_temp_dir.join("dart-sdk.zip.sha256sum");
	let dart_sdk_shasum_path = dart_sdk_shasum_path.to_str().unwrap();

	println!("INFO:Creating Dart SDK directory");

	if dart_sdk_dir.exists() {
		println!("INFO: Attempting to remove current Dart SDK: {:?}", dart_sdk_dir);
		match fs::remove_dir_all(&dart_sdk_dir) {
			Ok(_) => {
				println!("INFO: Found old Dart SDK, removed it");

				println!("INFO: Attempting to create Dart SDK directory: {:?}", dart_sdk_dir);

				match fs::create_dir(&dart_sdk_dir) {
					Ok(_) => {
						println!("INFO: Successfully created Dart SDK directory");
					},
					Err(e) => {
						println!("ERROR: Failed to create Dart SDK directory: {:?}", e);
						panic!("ERROR: Failed to create Dart SDK directory");
					},
				};
			},
			Err(e) => {
				println!("ERROR: Failed to remove current Dart SDK: {:?}", e);
				panic!("ERROR: Failed to remove current Dart SDK");
			},
		};
		println!("INFO: Successfully removed current Dart SDK");
	} else {
		match fs::create_dir(&dart_sdk_dir) {
			Ok(_) => {
				println!("INFO: Successfully created Dart SDK directory");
			},
			Err(e) => {
				println!("ERROR: Failed to create Dart SDK directory: {:?}", e);
				panic!("ERROR: Failed to create Dart SDK directory");
			},
		};
	}

	println!("INFO: Creating temp directory");

	if dart_sdk_temp_dir.exists() {
		println!("INFO: Attempting to remove temp directory: {:?}", dart_sdk_temp_dir);
		match fs::remove_dir_all(&dart_sdk_temp_dir) {
			Ok(_) => {
				println!("INFO: Found old temp directory, removed it");

				println!("INFO: Attempting to create temp directory: {:?}", dart_sdk_temp_dir);

				match fs::create_dir(&dart_sdk_temp_dir) {
					Ok(_) => {
						println!("INFO: Successfully created temp directory");
					},
					Err(e) => {
						println!("ERROR: Failed to create temp directory: {:?}", e);
						panic!("ERROR: Failed to create temp directory");
					},
				};
			},
			Err(e) => {
				println!("ERROR: Failed to remove temp directory: {:?}", e);
				panic!("ERROR: Failed to remove temp directory");
			},
		};
	} else {
		match fs::create_dir(&dart_sdk_temp_dir) {
			Ok(_) => {
				println!("INFO: Successfully created temp directory");
			},
			Err(e) => {
				println!("ERROR: Failed to create temp directory: {:?}", e);
				panic!("ERROR: Failed to create temp directory");
			},
		};
	}

	println!("INFO: attempting to download Dart SDK (this may take a while)");

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
		println!(
			"INFO: Downloading url: \"{}\" (this may take a while, this is normal.)",
			url.as_str()
		);
		// Download the file
		let mut resp = reqwest::blocking::get(url)?;

		println!("INFO: Downloaded successfully");

		// Create a buffer to read the file into
		let mut buffer = Vec::new();
		// Read the response into the buffer
		resp.copy_to(&mut buffer)?;

		println!("INFO: Writing file to: \"{}\"", dest);

		// Create a file to write the buffer to
		let mut file = File::create(dest)?;
		// Write the buffer to the file
		file.write_all(&buffer)?;

		println!("INFO: Successfully wrote file to: \"{}\"", dest);

		Ok(())
	}

	/// Checks the integrity of a downloaded file using the SHA-256 hash algorithm
	///
	/// ## Arguments:
	///
	/// * `file_path`: path to the the file to check the integrity of
	/// * `hash_path`: the path to the SHA-256 hash file to check the integrity of the file against
	fn check_sha256_checksum(file_path: &str, hash_path: &str) -> Result<(), Box<dyn StdError>> {
		println!("INFO: checking integrity of \"{}\"", file_path);
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
			Ok(println!("INFO: integrity check successful"))
		} else {
			let error = format!(
				"ERROR: integrity check failed. Expected hash: `{}`, Actual hash: `{}`",
				expected_hash, acutal_hash
			);
			println!("{}", error);
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
		println!(
			"INFO: attempting to unzip file \"{}\" to \"{}\"",
			file_path, destination
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
		println!("INFO: successfully unzipped Dart SDK");
		Ok(())
	}

	println!("INFO: Downloading Dart SDK zip archive");

	match download(dart_sdk_download_url, dart_sdk_zip_path) {
		Ok(_) => println!("INFO: Successfully downloaded Dart SDK zip archive"),
		Err(e) => panic!("ERROR: Failed to download Dart SDK zip archive: {}", e),
	}

	println!("INFO: Downloading Dart SDK SHA-256 hash");

	match download(dart_sdk_shasum_download_url, dart_sdk_shasum_path) {
		Ok(_) => println!("INFO: Successfully downloaded Dart SDK SHA-256 hash"),
		Err(e) => panic!("ERROR: Failed to download Dart SDK SHA-256 hash: {}", e),
	}

	println!("INFO: Checking integrity of Dart SDK zip archive");

	match check_sha256_checksum(dart_sdk_zip_path, dart_sdk_shasum_path) {
		Ok(_) => println!("INFO: Successfully checked integrity of Dart SDK zip archive"),
		Err(e) => panic!("ERROR: Failed to check integrity of Dart SDK zip archive: {}", e),
	}

	println!("INFO: Unzipping Dart SDK zip archive");

	match unzip_file(
		dart_sdk_temp_dir.join("dart-sdk.zip").to_str().unwrap(),
		dart_sdk_temp_dir.to_str().unwrap(),
	) {
		Ok(_) => println!("INFO: Successfully unzipped Dart SDK zip archive"),
		Err(e) => panic!("ERROR: Failed to unzip Dart SDK zip archive: {}", e),
	}

	println!("INFO: Copying files");

	// copy all files from /temp/dart-sdk to /dart-sdk

	fs_utils::copy_dir(&dart_sdk_temp_dir.join("dart-sdk"), &dart_sdk_dir);

	println!("INFO: Successfully copied files");

	println!("INFO: Removing temporary files");

	// remove /temp/dart-sdk

	match fs::remove_dir_all(dart_sdk_temp_dir) {
		Ok(_) => println!("INFO: Successfully removed temporary files"),
		Err(e) => panic!("ERROR: Failed to remove temporary files: {}", e),
	};

	println!("INFO: Successfully removed temporary files");

	println!("INFO: Moving Dart SDK to repository root");

	fs_utils::move_dir(&dart_sdk_dir, &Path::new("..").join("dart-sdk"));

	println!("INFO: Successfully moved Dart SDK to repository root");

	println!("INFO: Removing unused Dart SDK files");

	match fs::remove_dir_all(Path::new("..").join("dart-sdk").join("bin")) {
		Ok(_) => println!("INFO: Successfully removed unused Dart SDK files"),
		Err(e) => panic!("ERROR: Failed to remove unused Dart SDK files: {}", e),
	};

	match fs::remove_dir_all(Path::new("..").join("dart-sdk").join("lib")) {
		Ok(_) => println!("INFO: Successfully removed unused Dart SDK files"),
		Err(e) => panic!("ERROR: Failed to remove unused Dart SDK files: {}", e),
	};

	println!("INFO: Successfully removed unused Dart SDK files");

	println!("INFO: Finished update-dart-sdk script");
}
