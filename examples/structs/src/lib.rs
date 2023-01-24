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
	/// ## Safety
	/// ...
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

/// Uses pointer arithmetic to reverse a string.
///
/// # Arguments
///
/// * `string` - A pointer to an `&str` string.
/// * `length` - The `usize` length of the string.
///
/// ## Safety
///
/// This function is unsafe because it dereferences raw pointers.
/// The caller must ensure that the pointer is valid.
///
///
/// ## way to reverse a string by pointer manipulation in C:
/// ```c
/// char *reverse_string(char *string, int length) {
///     // Allocates native memory in C.
///     char *reversed_str = (char *)malloc((length + 1) * sizeof(char));
///     for (int i = 0; i < length; i++) {
///         reversed_str[length - i - 1] = string[i];
///     }
///     reversed_str[length] = '\0';
///     return reversed_str;
/// }
/// ```
#[no_mangle]
pub unsafe extern "C" fn reverse_string(string: *const &str) -> *mut &str {
	let string = &*string;
	let length = string.len();
	// opt to use String type for buffer allocation
	let mut buf = String::with_capacity(length);
	for i in 0..length {
		buf.push(string.chars().nth(length - i - 1).unwrap());
	}

	let reversed_str = buf.as_str();
	let reversed_str = Box::new(&reversed_str);
	let reversed_str = Box::into_raw(reversed_str);

	reversed_str as *mut &str
}

/// Returns 'Hello, World!' by pointer.
///
/// ## Safety
/// This function is unsafe because it dereferences raw pointers.
/// The caller must ensure that the pointer is valid.
///
/// ## C equivilant:
/// ```c
/// char *hello_world() {
///     return "Hello World";
/// }
/// ```
#[no_mangle]
pub unsafe extern "C" fn hello_world() -> *mut &'static str {
	let hello_world = "Hello, World!";
	let hello_world = Box::new(&hello_world);
	let hello_world = Box::into_raw(hello_world);

	hello_world as *mut &str
}

/// Frees a string from memory.
///
/// ## Arguments
///
/// * `str` - A pointer to a `&str` string.
///
/// ## Safety
///
/// This function is unsafe because it dereferences raw pointers.
/// The caller must ensure that the pointer is valid.
///
/// ## C equivilant:
/// ```c
/// void free_string(char *str) {
///    // Free native memory in C which was allocated in C.
///   free(str);
/// }
/// ```
#[no_mangle]
pub unsafe extern "C" fn free_str(str: *mut &str) {
	// drop the string from memory
	drop(Box::from_raw(str));
}

/// Struct representation of a planar cartesian coordinate.
///
/// ## Fields
///
/// * `latitude` - The `f64` latitude of the coordinate.
/// * `longitude` - The `f64` longitude of the coordinate.
///
/// ## C equivilant:
/// ```c
/// struct Coordinate {
///     double latitude;
///     double longitude;
/// };
/// ```
#[repr(C)]
pub struct Coordinate {
	pub latitude: f64,
	pub longitude: f64,
}

/// Creates a coordinate `Coordinate` on a cartesian plane.
///
/// ## Arguments
///
/// * `latitude` - The `f64` latitude of the coordinate.
/// * `longitude` - The `f64` longitude of the coordinate.

/// ##C equivilant:
/// ```c
/// struct Coordinate create_coordinate(double latitude, double longitude) {
///     struct Coordinate coordinate;
///     coordinate.latitude = latitude;
///     coordinate.longitude = longitude;
///     return coordinate;
/// }
/// ```
#[no_mangle]
pub extern "C" fn create_coordinate(latitude: f64, longitude: f64) -> Coordinate {
	Coordinate { latitude, longitude }
}

/// Struct representation of a place.
///
/// ## Fields
///
/// * `name` - The `&str` name of the place.
/// * `coordinate` - The `Coordinate` coordinate of the place.
///
/// ## C equivilant:
/// ```c
/// struct Place {
///     char *name;
///     struct Coordinate coordinate;
/// };
/// ```
#[repr(C)]
pub struct Place<'a> {
	pub name: *const &'a str,
	pub coordinate: Coordinate,
}

/// Creates a place `Place` on a cartesian plane.
///
/// ## Arguments
///
/// * `name` - The `&str` name of the place.
/// * `latitude` - The `f64` latitude of the coordinate.
/// * `longitude` - The `f64` longitude of the coordinate.
///
///
/// ## C equivilant:
/// ```c
/// struct Place create_place(char *name, double latitude, double longitude) {
///     struct Place place;
///     place.name = name;
///     place.coordinate = create_coordinate(latitude, longitude);
///     return place;
/// }
/// ```
#[no_mangle]
pub extern "C" fn create_place(name: *const &str, latitude: f64, longitude: f64) -> Place {
	Place {
		name,
		coordinate: create_coordinate(latitude, longitude),
	}
}

/// Calculates the distance between two coordinates.
///
/// ## Arguments
///
/// * `c1` - The `Coordinate` coordinate of the first place.
/// * `c2` - The `Coordinate` coordinate of the second place.

/// ## C equivilant:
/// ```c
/// double distance(struct Coordinate c1, struct Coordinate c2) {
///     double xd = c2.latitude - c1.latitude;
///     double yd = c2.longitude - c1.longitude;
///     return sqrt(xd*xd + yd*yd);
/// }
/// ```
#[no_mangle]
pub extern "C" fn distance(c1: Coordinate, c2: Coordinate) -> f64 {
	let xd = c2.latitude - c1.latitude;
	let yd = c2.longitude - c1.longitude;
	(xd * xd + yd * yd).sqrt()
}
