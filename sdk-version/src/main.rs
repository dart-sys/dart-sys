/// prints out the git hash of the current SDK version to stdout
fn main() {
	// sdk version is a git hash, located in a plain text file at
	// {project_root}/dart-sys/dart-sdk/revision
	let sdk_version = std::fs::read_to_string("dart-sys/dart-sdk/revision").unwrap();

	// print out the sdk version
	println!("{}", sdk_version);

	// exit with success
	std::process::exit(0);
}
