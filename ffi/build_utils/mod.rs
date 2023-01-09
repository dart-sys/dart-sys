//! functionally oriented programming utilities for building the library.

/// Functions related to git operations
pub mod git {
	/// Git wrapper that exposes git cli as a function.
	///
	/// Pass cli args to git as a `Vec<&str>`.
	///
	/// ## Arguments:
	/// * `args` - `Vec<&str>` of git cli args.
	pub fn git(args: Vec<&str>) {
		// TODO(@gutenfries)
	}

	/// Wrapper for git clone
	///
	/// ## Arguments:
	/// * `url` - `&str` git url.
	/// * `submodules` - `bool` include submodules.
	pub fn git_clone(url: &str, submodules: bool) {
		// TODO(@gutenfries)
	}
}

/// Functions related to cli based operations
pub mod cli_utils {
	/// Wrapper for the system shell's implimentation of `copy`.
	///
	/// Copys files and/or directories from `src` to `dst`.
	///
	/// ## Arguments:
	/// * `src` - `&str` source file path.
	/// * `dst` - `&str` destination file path.
	pub fn cp(src: &str, dst: &str) {
		// TODO(@gutenfries)
	}
}
