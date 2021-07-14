//! A simple configurable logger implementation
//!
//! Supports logging to a specified file, as well as
//! setting and adjusting log message levels, and the
//! format of log messages

use log::*;
use std::fs::OpenOptions;
use std::io::Write;

/// Consists of name for path of file to log to, and string
/// which serves as a format string for log messages
pub struct Minilog {
	logfile_name: String,
	fmt_string: String,
}


impl Minilog {
	/// Initializes the logger, must be called before attempting
	/// to write log messages
	/// 
	/// # Examples
	/// 
	/// ```
	/// # use log::LevelFilter;
	/// # use minilog::Minilog;
	/// Minilog::init(LevelFilter::Info, "logs.txt", "{} - {}");
	/// ```
	pub fn init(
		loglevel: LevelFilter,
		logfile_name: &str,
		fmt_string: &str,
	) -> Result<(), SetLoggerError> {
		set_boxed_logger(Box::new(Minilog {
			logfile_name: logfile_name.to_owned(),
			fmt_string: fmt_string.to_owned(),
		}))
		.map(|()| set_max_level(loglevel))
	}
	///Sets the maximum level of log message to write
	/// 
	/// # Examples
	/// 
	/// ```
	/// # use log::LevelFilter;
	/// # use minilog::Minilog;
	/// Minilog::set_log_level(LevelFilter::Info);
	/// ```
	pub fn set_log_level(loglevel: LevelFilter) {
		set_max_level(loglevel);
	}
}

impl Log for Minilog {
	///returns whether logging is enabled for a given level
	fn enabled(&self, metadata: &Metadata) -> bool {
		metadata.level() <= max_level()
	}

	///logs a message to file, using the format string provided
	/// # Panics
	/// panics if it can't open the file or write to it
	fn log(&self, record: &Record) {
		if self.enabled(record.metadata()) {
			let log_msg = self
				.fmt_string
				.replacen("{}", &format!("{}", format_args!("{}", record.level())), 1)
				.replacen("{}", &format!("{}", format_args!("{}", record.args())), 1);
			if self.logfile_name == "stdout" {
				println!("{}", log_msg);
			}
			else {
				let mut file = OpenOptions::new()
					.read(true)
					.append(true)
					.create(true)
					.open(&self.logfile_name);
				match &mut file {
					Ok(file) => match writeln!(file, "{}", log_msg) {
						Ok(_) => {}
						Err(e) => panic!("{}: Write failed", e),
					},
					Err(e) => panic!("{}: Failed to write to logfile {}", e, &self.logfile_name),
				}
			}
		}
	}

	///preserved for trait implementation
	fn flush(&self) {}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;
	#[test]
	fn test() {
		match Minilog::init(LevelFilter::Info, "Minilog_test_main.txt", "{}: {}") {
			Ok(_) => {}
			Err(e) => panic!("{}: Could not set the logger!", e),
		}
		log!(Level::Error, "Test log!");
		error!("Test error!");
		warn!("Test warning!");
		trace!("Test trace! exluded");
		Minilog::set_log_level(LevelFilter::Trace);
		trace!("Test trace! not excluded");
		let file_contents =
			fs::read_to_string("Minilog_test_main.txt").expect("Was unable to read file.");
		fs::remove_file("Minilog_test_main.txt").expect("Unable to delete test file.");
		assert_eq!(
			file_contents,
			"ERROR: Test log!\nERROR: Test error!\nWARN: Test warning!\nTRACE: Test trace! not excluded\n"
		);
	}
}
