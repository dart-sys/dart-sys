extern crate dart_sys_fork;
/// defines a planaer cartesian coordinate
///
/// # Fields
///
/// * `latitude` - x coordinate
/// * `longitude` - y coordinate
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Coordinate {
	pub latitude: f64,
	pub longitude: f64,
}

/// defines a place in the world
///
/// # Fields
///
/// * `name` - name of the place
/// * `coordinate` - coordinate of the place
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Place {
	pub name: *const libc::c_char,
	pub coordinate: Coordinate,
}

/// creates a new coordinate
///
/// # Arguments
///
/// * `latitude` - x coordinate
/// * `longitude` - y coordinate
///
/// # Returns
///
/// * `Coordinate` - the new coordinate
#[no_mangle]
pub extern "C" fn create_coordinate(latitude: f64, longitude: f64) -> Coordinate {
	Coordinate { latitude, longitude }
}

/// creates a new place
///
/// # Arguments
///
/// * `name` - name of the place
/// * `latitude` - latitude of the place
/// * `longitude` - longitude of the place
///
/// # Returns
///
/// * `Place` - the new place
#[no_mangle]
pub extern "C" fn create_place(name: *const libc::c_char, latitude: f64, longitude: f64) -> Place {
	Place {
		name,
		coordinate: create_coordinate(latitude, longitude),
	}
}

/// Calculates the distance between two places
///
/// # Arguments
///
/// * `c1` - the first coordinate
/// * `c2` - the second coordinate
///
/// # Returns
///
/// * `f64` - the distance between the two coordinates
#[no_mangle]
pub extern "C" fn distance(c1: Coordinate, c2: Coordinate) -> f64 {
	let x = c1.latitude - c2.latitude;
	let y = c1.longitude - c2.longitude;
	(x * x + y * y).sqrt()
}
