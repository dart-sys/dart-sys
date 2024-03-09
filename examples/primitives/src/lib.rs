extern crate dart_sys;

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
