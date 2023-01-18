extern crate dart_sys;

use std::{ffi::CStr, mem::MaybeUninit, sync::Mutex};

mod ffi_utils {

	#[no_mangle]
	unsafe extern "C" fn resolve_name(
		name: dart_sys::Dart_Handle, _argc: std::os::raw::c_int, _auto_setup_scope: *mut bool,
	) -> dart_sys::Dart_NativeFunction {
		if !dart_sys::Dart_IsString(name) {
			return None;
		}
		let mut result: dart_sys::Dart_NativeFunction = None;
		let mut cname = MaybeUninit::<*const libc::c_char>::uninit();
		handle_error(dart_sys::Dart_StringToCString(name, cname.as_mut_ptr()));

		let cname = CStr::from_ptr(cname.assume_init());
		if cname.to_bytes() == b"SystemRand" {
			result = Some(system_rand);
		} else if cname.to_bytes() == b"SystemSrand" {
			result = Some(system_s_rand)
		}
		result
	}

	#[allow(non_snake_case)]
	#[no_mangle]
	pub unsafe extern "C" fn dart_rust_dart_sys_Init(parent_library: dart_sys::Dart_Handle) -> dart_sys::Dart_Handle {
		if dart_sys::Dart_IsError(parent_library) {
			return parent_library;
		}

		let result_code = dart_sys::Dart_SetNativeResolver(parent_library, Some(resolve_name), None);
		if dart_sys::Dart_IsError(result_code) {
			result_code
		} else {
			dart_sys::Dart_Null()
		}
	}

	#[no_mangle]
	/// Error handler for Dart FFI functions.
	unsafe fn handle_error(handle: dart_sys::Dart_Handle) -> dart_sys::Dart_Handle {
		if dart_sys::Dart_IsError(handle) {
			dart_sys::Dart_PropagateError(handle);
		}
		handle
	}
}
