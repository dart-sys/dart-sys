//! Generates bindings for the Dart API dynamic library.

use crate::{
	log::LogLevel,
	paths::{dart_sys_crate_path, log_file_path, repo_root},
};

#[macro_use]
mod log;

mod codegen;
mod partials;
mod paths;
mod update;

fn main() {
	log!("------------------------------");
	log!(
		LogLevel::Info,
		format!("build log is located at: \"{}\"", log_file_path().to_str().unwrap())
	);

	// update Dart SDK
	match update::update_dart_sdk() {
		Ok(_) => (),
		Err(e) => {
			log!(LogLevel::Error, format!("{}", e));
			panic!("ERROR: {}", e);
		},
	}

	// generate bindings
	let bindings = match codegen::generate_bindings() {
		Ok(bindings) => bindings,
		Err(e) => {
			log!(LogLevel::Error, format!("{}", e));
			panic!("ERROR: {}", e);
		},
	};

	// remove  Cargo.toml, build.rs, and src/ directory in bindings crate, keeping dart-sdk/ directory,
	// if they exist
	match std::fs::remove_file(dart_sys_crate_path().join("build.rs")) {
		Ok(_) => (),
		Err(e) => {
			log!(LogLevel::Error, format!("{}", e));
			panic!("ERROR: {}", e);
		},
	}

	match std::fs::remove_file(dart_sys_crate_path().join("Cargo.toml")) {
		Ok(_) => (),
		Err(e) => {
			log!(LogLevel::Error, format!("{}", e));
			panic!("ERROR: {}", e);
		},
	}

	// write header and bindings to file
	match std::fs::write(
		dart_sys_crate_path().join("src").join("lib.rs"),
		format!("{}{}", partials::LIB_RS_HEADER_STUB, bindings),
	) {
		Ok(_) => (),
		Err(e) => {
			log!(LogLevel::Error, format!("{}", e));
			panic!("ERROR: {}", e);
		},
	}

	// write build.rs to file
	match std::fs::write(dart_sys_crate_path().join("build.rs"), partials::BUILD_RS_STUB) {
		Ok(_) => (),
		Err(e) => {
			log!(LogLevel::Error, format!("{}", e));
			panic!("ERROR: {}", e);
		},
	}

	// write cargo.toml to file
	match std::fs::write(
		dart_sys_crate_path().join("Cargo.toml"),
		partials::CARGO_TOML_HEADER_STUB,
	) {
		Ok(_) => (),
		Err(e) => {
			log!(LogLevel::Error, format!("{}", e));
			panic!("ERROR: {}", e);
		},
	}

	// format generated code
	log!("formatting generated code...");
	match std::process::Command::new("rustup")
		.arg("run")
		.arg("nightly")
		.arg("cargo")
		.arg("fmt")
		.arg("--package")
		.arg("dart-sys")
		.current_dir(repo_root())
		.status()
	{
		Ok(_) => log!(LogLevel::Success, "successfully formatted bindings"),
		Err(e) => {
			log!(LogLevel::Error, format!("{}", e));
			panic!("ERROR: {}", e);
		},
	}

	log!("------------------------------");
}
