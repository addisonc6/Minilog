# Minilog

A simple Rust logginger implementing the [log crate](https://docs.rs/log/0.4.14/log/).<br>
Writes all logs to a specified file with user provided format.

# Usage
```
//initialize the logger
Minilog::init(LevelFilter::Info, "logs.txt", "{} - {}");

//log messages to file
log!(Level::Error, "Oh no! There was an error with something.");
warn!("Warning! Something could go wrong here.");

//change log level
Minilog::set_log_level(LevelFilter::Warn);
```
