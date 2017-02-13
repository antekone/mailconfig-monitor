use log;
use log::{LogRecord, LogLevel, LogMetadata, SetLoggerError, Log, LogLevelFilter};
use ansi_term::Colour::{Yellow, Red, White};

struct DefaultLogger;

impl Log for DefaultLogger {
    fn enabled(&self, _: &LogMetadata) -> bool { true }

    fn log(&self, record: &LogRecord) {
        // if self.enabled() ...
        let banner = match record.level() {
            LogLevel::Error => format!("{}", Red.paint("error")),
            LogLevel::Warn  => format!("{}", Yellow.paint("warn")),
            LogLevel::Info  => format!("{}", White.paint("info")),
            LogLevel::Debug => "debug".to_string(),
            LogLevel::Trace => "trace".to_string(),
        };

        println!("[{}] {}", banner, record.args());
    }
}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(|max_log_level| {
        max_log_level.set(LogLevelFilter::Trace);
        Box::new(DefaultLogger)
    })
}
