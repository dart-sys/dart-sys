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

//! Bindings for `dart_api_dl.h`.
//!
//! The dart_api_dl library is for allowing code which is embedded/
//! loaded into dart to interact with the dart VM. In combination
//! with darts `ffi` package it is about to/did replace the previous
//! way to extends dart through bindings to `dart_api.h`. It is independent
//! of flutter but it also can be used in combination with flutter and by
//! flutter plugins, _but is a different API than the one provided by flutter
//! for flutter plugins_.
//!
//! This library provides just the auto generated bindings and statically
//! links in the necessary C glue code. **It's strongly recommended to
//! use the `dart-api-dl` library, which provides a still low-level but
//! slightly nicer and safer to use interface**.
//!
//! # Supported Dart Versions
//!
//! Any Dart VM with a `dart_api_dl.h` version >=2.0 and
//! <3.0 are supported. This means the minimal supported
//! dart version is 2.12. Known compatible versions
//! include 2.13, 2.14 and 2.15 (though 2.15 adds
//! a new CObject variant we do not yet support).
//!
//! # Dart Functions
//!
//! Except [`Dart_InitializeApiDL`] all functions are provided through
//! global variables containing function pointers which are set when
//! [`Dart_InitializeApiDL`] is called successfully.
//!
//! Accessing any of this global variable before [`Dart_InitializeApiDL`]
//! completed should be treated as unsound, even if you do null pointer
//! checks.
//!
//! ## Dart API DL Version Handling
//!
//! The dart API DL is separately versioned from dart. Calling
//! [`Dart_InitializeApiDL`] will fail if the major version doesn't
//! match. **It won't fail if the minor version doesn't match.**
//!
//! Using bindings with a lower minor version (e.g. 2.0) than
//! that of the Dart VM (e.g. 2.1) is not a problem at all
//! and no special care must be taking in that case.
//!
//! But if using dart bindings with a higher minor version with
//! a Dart VM having a lower minor version you need to consider
//! following:
//!
//! - Some function pointer might be null even after [`Dart_InitializeApiDL`] was called. Doing "is
//!   null" checks *after* (and only after) [`Dart_InitializeApiDL`] was called required for any
//!   function added after version 2.0 and is sound ("is null" checks before initialization are not
//!   sound!).
//!
//! - You must not use variants of [`Dart_CObject_Type`]/[`Dart_CObject`] which didn't exist in the
//!   dart VM's API version.
//!
//! The const [`DART_API_DL_MAJOR_VERSION`] and [`DART_API_DL_MINOR_VERSION`]
//! represent the version of this bindings.
//!
//! The version of the Dart VM's API DL **can not be looked up, the functionality
//! is missing.** Currently we are at API DL version 2.0 so it doesn't matter.
// FIXME: But Dart 2.15 will bump it to 2.1 (hopefully) in a (potentially) non-detectable way.
//        If it's not fixed by dart we could somewhat work around it, but I hope we don't need to.
//        Also versions are accessible from dart, so higher level bindings
//        can handle it, somewhat, not very nicely.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// This is triggered by auto generated alignment *tests*, which unsafely
// turn a nullptr into a reference.
#![cfg_attr(test, allow(deref_nullptr))]

include!(concat!(env!("OUT_DIR"), "/dart_api_dl_bindings.rs"));

pub const ILLEGAL_PORT: Dart_Port_DL = 0;

#[cfg(test)]
mod tests {
	#![deny(deref_nullptr)]
	use static_assertions::assert_type_eq_all;

	use super::*;

	#[test]
	fn dart_port_is_dart_port_dl() {
		assert_type_eq_all!(Dart_Port, Dart_Port_DL);
	}
}
