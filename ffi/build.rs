
mod api {
	use std::{cfg, env, path::PathBuf};

fn find_dart_sdk() -> Option<PathBuf> {
	if let Ok(path) = env::var("dart_sdk") {
		// Check for the dart SDK in the dart_sdk environment variable
		Some(path.into())
	} else {
		// Check for the dart SDK in the PATH variable
		let paths: Vec<PathBuf> =
			env::split_paths(&std::env::var("PATH").expect("Could not find $PATH variable.")).collect();

		for i in &paths {
			if i.components().any(|x| x.as_os_str() == "dart-sdk") {
				let mut path = i.clone();
				while !path.is_dir() || path.file_name().unwrap() != "dart-sdk" {
					path.pop();
				}
				if !path.as_os_str().is_empty() {
					return Some(path);
				}
			}
		}

		for i in &paths {
			if i.components().any(|x| x.as_os_str() == "flutter") {
				let mut path = i.clone();
				while !path.is_dir() || path.file_name().unwrap() != "flutter" {
					path.pop();
				}
				if !path.as_os_str().is_empty() {
					// println!("cargo:warning=INFO: Defaulting to Flutter-provided dart SDK.");
					return Some(path.join("bin/cache/dart-sdk"));
				}
			}
		}
		None
	}
}

fn emit_compiler_flags() {
	let target_os = env::var("CARGO_CFG_TARGET_OS");
	match target_os.as_ref().map(|x| &**x) {
		Ok("windows") => {
			let dart_path = match find_dart_sdk() {
				Some(x) => {
					// println!("cargo:warning=INFO: Found dart sdk at: {}", x.to_str().unwrap());
					x
				},
				None => {
					panic!(
						"Could not find a dart SDK.\nPlease install the Dart SDK or set the dart_sdk environment \
						 variable to the path of the Dart SDK.\nIf you have Flutter installed, the dart SDK will be \
						 found automatically."
					)
				},
			};
			let dart_path = dart_path;
			println!(
				r#"cargo:rustc-link-search=native={}"#,
				dart_path.join("bin").to_str().unwrap()
			);
			println!(r"cargo:rustc-link-lib=dart");
		},
		_ => println!("INFO: Target OS is not windows"),
	}

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

	pub fn build() {
		#[cfg(not(feature = "docs-only"))]
		emit_compiler_flags();
	}
}


/// Code generation for `dart_api_dl.h`.
mod api_dl {
	// Copyright 2021 Xayn AG
	//
	// Licensed under the Apache License, Version 2.0 (the "License");
	// you may not use this file except in compliance with the License.
	// You may obtain a copy of the License at
	//
	//     http://www.apache.org/licenses/LICENSE-2.0
	//
	// Unless required by applicable law or agreed to in writing, software
	// distributed under the License is distributed on an "AS IS" BASIS,
	// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
	// See the License for the specific language governing permissions and
	// limitations under the License.

	use std::{env, path::PathBuf};

	use bindgen::EnumVariation;

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

	static DL_ENABLED_VARS: &[&str] = &[
		"Dart_.+_DL",
		"DART_API_DL_MAJOR_VERSION",
		"DART_API_DL_MINOR_VERSION",
	];

	fn codegen() {
		print!("cargo:rerun-if-env-changed=CARGO_PKG_VERSION");
		let dart_src_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("dart-src");
	
		let dl_header_path = dart_src_dir.join("dart_api_dl.h");
		let dl_version_header_path = dart_src_dir.join("dart_version.h");
	
		let mut builder = bindgen::Builder::default()
			.header(dl_header_path.to_str().expect("non-utf8 path"))
			.header(dl_version_header_path.to_str().expect("non-utf8 path"))
			.parse_callbacks(Box::new(bindgen::CargoCallbacks))
			.default_enum_style(EnumVariation::NewType { is_bitfield: false });
	
		for function in DL_ENABLED_FUNCTIONS {
			builder = builder.allowlist_function(function);
		}
	
		for r#type in DL_ENABLED_TYPES {
			builder = builder.allowlist_type(r#type);
		}
	
		for var in DL_ENABLED_VARS {
			builder = builder.allowlist_var(var);
		}
	
		let bindings = builder
			.generate()
			.expect("Failed to generate dart_api_dl binding");
	
		let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("dart_api_dl_bindings.rs");
		bindings
			.write_to_file(out_path)
			.expect("Failed to write dat_api_dl bindings.");
	
		let dl_glue_path = dart_src_dir.join("dart_api_dl.c");
		cc::Build::new()
			.file(dl_glue_path)
			.include(dart_src_dir)
			.compile("dart_api_dl");
	}

	pub fn build() {
		codegen();
	}
}

fn main() {
    if cfg!(feature = "dart_api_dl") {
		api_dl::build();
    } else {
        api::build();
    }
}
