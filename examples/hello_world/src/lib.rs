extern crate dart_sys;
extern crate rand;

pub mod ffi_utils {
	use std::{ffi::CStr, mem::MaybeUninit, sync::Mutex};

	use rand::{
		rngs::{OsRng, StdRng},
		Rng, RngCore, SeedableRng,
	};

	lazy_static::lazy_static! {
		static ref RNG: Mutex<Option<Box<dyn RngCore + Send + Sync>>> = Mutex::new(None);
	}

	#[allow(non_snake_case)]
	#[no_mangle]
	/// Initializer function for the library.
	///
	/// This function is called by the Dart VM when the library is loaded.
	pub unsafe extern "C" fn hello_world_Init(parent_library: dart_sys::Dart_Handle) -> dart_sys::Dart_Handle {
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
	/// Helper funtion.
	///
	/// Helps resolve the name of the native function.
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

	#[no_mangle]
	/// Helper function.
	///
	/// Helps handle errors.
	unsafe fn handle_error(handle: dart_sys::Dart_Handle) -> dart_sys::Dart_Handle {
		if dart_sys::Dart_IsError(handle) {
			dart_sys::Dart_PropagateError(handle);
		}
		handle
	}

	#[no_mangle]
	unsafe extern "C" fn system_rand(arguments: dart_sys::Dart_NativeArguments) {
		let integer = if let Some(x) = &mut *RNG.lock().unwrap() {
			x.gen::<i64>()
		} else {
			let mut rng = Box::new(OsRng) as Box<dyn RngCore+Send+Sync>;
			let num = rng.gen::<i64>();
			let rng = Some(rng);
			*RNG.lock().unwrap() = rng;
			num
		};
		let result = handle_error(dart_sys::Dart_NewInteger(integer));
		dart_sys::Dart_SetReturnValue(arguments, result);
	}

	#[no_mangle]
	unsafe extern "C" fn system_s_rand(arguments: dart_sys::Dart_NativeArguments) {
		let mut success = false;
		let seed_object = handle_error(dart_sys::Dart_GetNativeArgument(arguments, 0));
		if dart_sys::Dart_IsInteger(seed_object) {
			let mut fits = false;
			handle_error(dart_sys::Dart_IntegerFitsIntoInt64(seed_object, &mut fits));
			if fits {
				let mut seed = 0;
				handle_error(dart_sys::Dart_IntegerToInt64(seed_object, &mut seed));
				*RNG.lock().unwrap() = Some(Box::new(StdRng::seed_from_u64(seed as u64)));
				success = true;
			}
		}
		dart_sys::Dart_SetReturnValue(arguments, handle_error(dart_sys::Dart_NewBoolean(success)));
	}
}

pub use ffi_utils::hello_world_Init;

#[no_mangle]
/// Prints "Hello, world!" to the standard output.
pub extern "C" fn hello_world() {
	println!("Hello, World!");
}
