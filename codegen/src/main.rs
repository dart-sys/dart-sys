//! Generates bindings for the Dart API dynamic library.

use crate::{
	log::LogLevel,
	utils::{
		file_stubs,
		paths::{dart_sys_crate_path, log_file_path},
	},
};

#[macro_use]
mod log;

mod codegen;
mod update_dart_sdk;
mod utils;

fn main() {
	log!("------------------------------");
	log!(
		LogLevel::Info,
		format!("build log is located at: \"{}\"", log_file_path().to_str().unwrap())
	);

	// update Dart SDK
	match update_dart_sdk::update_dart_sdk() {
		Ok(_) => (),
		Err(e) => {
			log!(LogLevel::Error, format!("{}", e));
			panic!("ERROR: {}", e);
		},
	}

	// generate bindings for Dart API and write to buffer

	// Buffer to writing bindings to
	let mut buffer = Vec::new();

	// generate bindings
	match codegen::generate_bindings() {
		Ok(bindings) => {
			// write bindings to buffer
			match bindings.write(Box::new(&mut buffer)) {
				Ok(_) => (),
				Err(e) => {
					log!(LogLevel::Error, format!("{}", e));
					panic!("ERROR: {}", e);
				},
			}
		},
		Err(e) => {
			log!(LogLevel::Error, format!("{}", e));
			panic!("ERROR: {}", e);
		},
	}

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
		format!(
			"{}{}",
			file_stubs::LIB_RS_HEADER_STUB,
			String::from_utf8(buffer).unwrap()
		),
	) {
		Ok(_) => (),
		Err(e) => {
			log!(LogLevel::Error, format!("{}", e));
			panic!("ERROR: {}", e);
		},
	}

	// write build.rs to file
	match std::fs::write(dart_sys_crate_path().join("build.rs"), file_stubs::BUILD_RS_STUB) {
		Ok(_) => (),
		Err(e) => {
			log!(LogLevel::Error, format!("{}", e));
			panic!("ERROR: {}", e);
		},
	}

	log!("formatting bindings crate");
	// format bindings crate with rustfmt
	match std::process::Command::new("rustfmt")
		.arg(dart_sys_crate_path().join("src").join("lib.rs"))
		.output()
	{
		Ok(_) => {
			log!(LogLevel::Success, "Successfully formatted bindings crate");
		},
		Err(e) => {
			log!(LogLevel::Error, format!("{}", e));
			panic!("ERROR: {}", e);
		},
	}

	// write cargo.toml to file
	match std::fs::write(
		dart_sys_crate_path().join("Cargo.toml"),
		file_stubs::CARGO_TOML_HEADER_STUB,
	) {
		Ok(_) => (),
		Err(e) => {
			log!(LogLevel::Error, format!("{}", e));
			panic!("ERROR: {}", e);
		},
	}

	log!("------------------------------");
}
