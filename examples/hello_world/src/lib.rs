extern crate dart_sys_fork;

#[no_mangle]
/// Prints "Hello, world!" to the standard output.
pub extern "C" fn hello_world() {
	println!("Hello, World!");
}
