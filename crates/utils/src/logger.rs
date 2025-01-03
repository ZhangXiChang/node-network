use anyhow::Result;
use flexi_logger::LogSpecification;

trait TextPaint {
    fn paint(&self, text: impl ToString) -> String;
}
impl TextPaint for log::Level {
    fn paint(&self, text: impl ToString) -> String {
        let text = text.to_string();
        match self {
            log::Level::Trace => nu_ansi_term::Color::DarkGray.paint(text),
            log::Level::Debug => nu_ansi_term::Color::Purple.paint(text),
            log::Level::Info => nu_ansi_term::Color::Green.paint(text),
            log::Level::Warn => nu_ansi_term::Color::Yellow.paint(text),
            log::Level::Error => nu_ansi_term::Color::Red.paint(text),
        }
        .to_string()
    }
}

pub struct Logger {
    logger: flexi_logger::Logger,
}
impl Logger {
    fn fmt_str(now_time: impl ToString, level: impl ToString, msg: impl ToString) -> String {
        format!(
            "[{}][{}]:{}",
            now_time.to_string(),
            level.to_string(),
            msg.to_string()
        )
    }
    pub fn new() -> Self {
        let logger = flexi_logger::Logger::with(
            LogSpecification::builder()
                .default(log::LevelFilter::Info)
                .build(),
        )
        .format_for_stderr(
            |write: &mut dyn std::io::Write,
             now_time: &mut flexi_logger::DeferredNow,
             record: &log::Record|
             -> std::io::Result<()> {
                write.write_all(
                    Self::fmt_str(
                        now_time.format_rfc3339(),
                        record.level().paint(record.level()),
                        record.args(),
                    )
                    .as_bytes(),
                )
            },
        );
        Self { logger }
    }
    pub fn enable_file(self) -> Self {
        let logger = self
            .logger
            .log_to_file(flexi_logger::FileSpec::default().directory("./log/"))
            .duplicate_to_stderr(flexi_logger::Duplicate::All)
            .format_for_files(
                |write: &mut dyn std::io::Write,
                 now_time: &mut flexi_logger::DeferredNow,
                 record: &log::Record|
                 -> std::io::Result<()> {
                    write.write_all(
                        Self::fmt_str(now_time.format_rfc3339(), record.level(), record.args())
                            .as_bytes(),
                    )
                },
            );
        Self { logger }
    }
    pub fn start(self) -> Result<()> {
        self.logger.start()?;
        Ok(())
    }
}
