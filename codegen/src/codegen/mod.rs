use std::env;

use bindgen::Bindings;

use crate::{
	log::LogLevel,
	utils::paths::{dart_sdk_bin_path, dart_sdk_include_path},
};

pub fn generate_bindings() -> Result<Bindings, String> {
	log!(LogLevel::Info, "Generating rust bindings to Dart API...");

	// if target OS is windows, add extra compile flags nessecary for linking
	// AGAINST the Dart SDK binaries (gotta love windows)
	match env::consts::OS {
		"windows" => {
			log!(
				LogLevel::Info,
				"target OS is windows, adding extra compile flags for linking against Dart SDK binaries"
			);

			// add extra compile flags for linking against Dart SDK binaries
			println!(
				"cargo:rustc-link-search=native={}",
				dart_sdk_bin_path().to_str().unwrap()
			);
			println!("cargo:rustc-link-lib=static=dart");
		},
		_ => {
			log!(
				LogLevel::Info,
				"target OS is not windows, skipping extra compile flags for linking against Dart SDK binaries"
			)
		},
	}

	let builder = bindgen::Builder::default()
		.header(
			dart_sdk_include_path()
				.join("dart_api_dl.h")
				.to_str()
				.expect("ERROR: could not find path `dart_api_dl.h`"),
		)
		.header(
			dart_sdk_include_path()
				.join("dart_api.h")
				.to_str()
				.expect("ERROR: could not find path `dart_api_dl.h`"),
		)
		.header(
			dart_sdk_include_path()
				.join("dart_version.h")
				.to_str()
				.expect("ERROR: could not find path `dart_version.h`"),
		)
		.header(
			dart_sdk_include_path()
				.join("dart_native_api.h")
				.to_str()
				.expect("ERROR: could not find path `dart_native_api.h`"),
		)
		.header(
			dart_sdk_include_path()
				.join("dart_tools_api.h")
				.to_str()
				.expect("ERROR: could not find path `dart_tools_api.h`"),
		)
		.parse_callbacks(Box::new(bindgen::CargoCallbacks))
		.default_enum_style(bindgen::EnumVariation::NewType {
			is_bitfield: false,
			is_global: true,
		})
		.use_core()
		.layout_tests(false)
		.rustfmt_bindings(false)
		.sort_semantically(true);

	log!(LogLevel::Info, "Generating bindings...");
	let bindings = builder.generate().expect("ERROR: Failed to generate dart_api binding");
	log!(LogLevel::Success, "Successfully generated bindings!");

	Ok(bindings)
}
