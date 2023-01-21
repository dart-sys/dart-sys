#![allow(
	non_upper_case_globals,
	non_camel_case_types,
	non_snake_case,
	unused_variables,
	dead_code
)]

#[cfg(not(feature = "dart_api_dl"))]
pub mod bindings;

#[cfg(not(feature = "dart_api_dl"))]
pub use bindings::*;

#[cfg(feature = "dart_api_dl")]
pub mod bindings_api_dl;

#[cfg(feature = "dart_api_dl")]
pub use bindings_api_dl::*;
