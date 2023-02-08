//! Generates bindings for the Dart API dynamic library.

use crate::{
	log::LogLevel,
	paths::{dart_sys_crate_path, log_file_path, repo_root},
	symbols::ByteBuffer,
};

#[macro_use]
mod log;
mod codegen;

mod partials;
mod paths;
mod symbols;
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
	// generate bindings
	let mut bindings: ByteBuffer = match codegen::generate_bindings() {
		Ok(b) => ByteBuffer::from_bindgen_bindings(b),
		Err(e) => {
			log!(LogLevel::Error, format!("{}", e));
			panic!("ERROR: {}", e);
		},
	};

	let symbols = symbols::parse_symbols(bindings.clone()).unwrap();
	let features = symbols::generate_features(symbols);

	log!("generating features...");

	for feature in features.iter() {
		log!(format!(
			"generating feature: \"{}\" ({} of {})",
			feature,
			features.iter().position(|f| f == feature).unwrap() + 1,
			features.len()
		));
		match symbols::add_feature_macro(&mut bindings, feature.to_string()) {
			Ok(_) => (),
			Err(e) => {
				log!(LogLevel::Error, format!("{}", e));
				panic!("ERROR: {}", e);
			},
		}
	}

	log!(LogLevel::Success, "successfully generated all features");

	log!("writing files...");

	match std::fs::write(
		dart_sys_crate_path().join("Cargo.toml"),
		format!(
			"{}{} = []\nall = [\n\t\"{}\"\n]",
			partials::CARGO_TOML_HEADER_PARTIAL,
			features.join(" = []\n"),
			features.join("\",\n\t\"")
		),
	) {
		Ok(_) => (),
		Err(e) => {
			log!(LogLevel::Error, format!("{}", e));
			panic!("ERROR: {}", e);
		},
	}

	// write header and bindings to file
	match std::fs::write(
		dart_sys_crate_path().join("src").join("lib.rs"),
		format!("{}{}", partials::LIB_RS_HEADER_PARTIAL, bindings),
	) {
		Ok(_) => (),
		Err(e) => {
			log!(LogLevel::Error, format!("{}", e));
			panic!("ERROR: {}", e);
		},
	}

	// write build.rs to file
	match std::fs::write(dart_sys_crate_path().join("build.rs"), partials::BUILD_RS_PARTIAL) {
		Ok(_) => (),
		Err(e) => {
			log!(LogLevel::Error, format!("{}", e));
			panic!("ERROR: {}", e);
		},
	}

	log!(LogLevel::Success, "successfully wrote files");

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
		.output()
	{
		Ok(_) => (),
		Err(e) => {
			log!(LogLevel::Error, format!("{}", e));
			panic!("ERROR: {}", e);
		},
	}

	log!(LogLevel::Success, "build successful");
	log!("------------------------------");
}
