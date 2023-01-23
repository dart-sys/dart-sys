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
	pub unsafe extern "C" fn primitives_Init(parent_library: dart_sys::Dart_Handle) -> dart_sys::Dart_Handle {
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

pub use ffi_utils::primitives_Init;

/// Returns a random number between 0 and 255 as an unsigned 8-bit integer by using an external
/// crate.
#[no_mangle]
pub extern "C" fn random_number() -> u8 {
	rand::random::<u8>()
}

/// Adds two integers with wrapping support by using the rust stdlib.
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
	a.wrapping_add(b)
}

/// multiplies two integers by unsafe pointer arithmetic
///
/// ```c
/// // C example
///
/// int *multiply(int a, int b) {
///     // Allocates native memory in C.
///     int *mult = (int *)malloc(sizeof(int));
///     *mult = a * b;
///     return mult;
/// }
/// ```
#[no_mangle]
pub unsafe extern "C" fn multiply(a: i32, b: i32) -> *mut i32 {
	// Allocate memory in rust
	let mult = std::alloc::alloc(std::alloc::Layout::new::<i32>()) as *mut i32;
	// Write to memory
	*mult = a * b;
	mult
}

/// Frees an `i32` allocated pointer from memory.
#[no_mangle]
pub unsafe extern "C" fn free_pointer(ptr: *mut i32) {
	std::ptr::drop_in_place(ptr);
}

/// subtracts two integers by pointer dereferencing
#[no_mangle]
pub unsafe extern "C" fn subtract(a: *mut i32, b: i32) -> i32 {
	*a - b
}
