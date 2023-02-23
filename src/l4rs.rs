//! It's a simple wrapper initializer for log4rs
//!
//! ```rust
//! [dependencies]
//! log4rs = "1.2.0+"
//! log = "0.4.17+"
//! ```
//!
//! ```rust
//! use bsc;
//! use log;
//!
//! let logfile = "config/log4rs.yaml";
//!
//! if bsc::tool::l4rs::init_file(logfile) == None {
//!     panic!("Init log file {:?} error.", logfile);
//! )
//!
//! info!("hello");
//! ```

use log4rs;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::{Config, init_config};
use log4rs::config::{Appender, Logger, Root};
use log::LevelFilter;

const DEFAULT_LOG_FILE: &str = "./default.log";

/// We should provide the config `file` in yaml format.
pub fn init_file(file: &str) -> Option<()> {
    let r = log4rs::init_file(file, Default::default());
    if r.is_err() {
        println!("[ERROR] {:?}", r.unwrap_err().to_string());
        return None;
    }

    return Some(r.unwrap());
}

/// Use the default configuration when the `file` is not exist.
pub fn init_file_or(file: &str) -> Option<()> {
    if let Some(v) = init_file(file) {
        return Some(v);
    }

    return init_default();
}

/// Make default configuration and initialize the logger
pub fn init_default() -> Option<()> {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("[{d}] [{l}] - {m}{n}")))
        .build();

    let defaultFile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("[{d}] [{l}] - {m}{n}")))
        .build(DEFAULT_LOG_FILE)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(defaultFile)))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("file")
                .build(LevelFilter::Info)
        ).unwrap();

    let r = init_config(config);
    if r.is_err() {
        println!("[ERROR] {:?}", r.unwrap_err().to_string());
        return None;
    }

    return Some(());
}
