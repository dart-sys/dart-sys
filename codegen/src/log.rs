/// log level
///
/// # Variants
///
/// * `Error` - Error log level
/// * `Warn` - Warning log level
/// * `Info` - Information log level
/// * `Success` - Success log level
#[allow(dead_code)]
pub enum LogLevel {
	Error,
	Warn,
	Info,
	Success,
}

/// Logger macro for the Dart FFI code generator.
///
/// Prints pretty messages to STDOUT, as well as to a logfile
///
/// # Arguments
///
/// * `level` - Log level
/// * `message` - Log message
///
/// # Panics
///
/// Panics if the `CARGO_MANIFEST_DIR` environment variable is not set.
///
/// # Example
///
/// ```rust
/// log!(LogLevel::Info, "hello world");
/// log!(LogLevel::Success, "hello world, this is a success message");
/// log!(LogLevel::Error, "this is an error message");
/// log!(LogLevel::Warn, "10");
/// ```
///
/// # Output
///
/// ```console
/// INFO:    hello world
/// SUCCESS: hello world, this is a success message
/// ERROR:   this is an error message
/// WARN:    10
/// ```
/// ```bash
/// 
/// cat build.log
/// ```
///
/// ```log
/// [2020-05-01T00:00:00Z] INFO:    hello world
/// [2020-05-01T00:04:34Z] SUCCESS: hello world, this is a success message
/// [2020-05-01T00:04:34Z] ERROR:   this is an error message
/// [2020-05-01T00:04:34Z] WARN:    10
/// ```
#[macro_export]
macro_rules! log {
	($message:expr) => {
		$crate::log::log($crate::log::LogLevel::Info, $message)
	};
	($level:expr, $message:expr) => {
		$crate::log::log($level, $message)
	};
}

/// log worker for log! macro
///
/// # Arguments
///
/// * `level` - Log level
/// * `message` - Log message
/// * `arg` -  log argument
///
/// # Panics
///
/// Panics if the `CARGO_MANIFEST_DIR` environment variable is not set.
pub fn log<T>(level: LogLevel, message: T)
where T: std::fmt::Display {
	use std::io::Write;
	let now = chrono::Utc::now();
	let log_level = match level {
		LogLevel::Error => "ERROR:  ",
		LogLevel::Warn => "WARN:   ",
		LogLevel::Info => "INFO:   ",
		LogLevel::Success => "SUCCESS:",
	};
	// color the log level
	let log_level_c = format!(
		"\x1b[{}m{}\x1b[0m",
		match level {
			// error bright red
			LogLevel::Error => "31;1",
			// warn bright yellow
			LogLevel::Warn => "33;1",
			// info cyan
			LogLevel::Info => "36;1",
			// success bright green
			LogLevel::Success => "32;1",
		},
		log_level
	);
	let log_message_c = format!("{} {}", log_level_c, message);
	let log_message = format!("[{}] {} {}", now.format("%Y-%m-%dT%H:%M:%SZ"), log_level, message);
	println!("    {}", log_message_c);
	let log_file = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("build.log");
	let mut log_file = std::fs::OpenOptions::new()
		.append(true)
		.create(true)
		.open(log_file)
		.unwrap();
	writeln!(log_file, "{}", log_message).unwrap();
}
