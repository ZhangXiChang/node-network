use std::path::PathBuf;

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

pub trait LoggerBuilder {
    fn fmt_str(now_time: impl ToString, level: impl ToString, msg: impl ToString) -> String {
        format!(
            "[{}][{}]:{}",
            now_time.to_string(),
            level.to_string(),
            msg.to_string()
        )
    }
    fn builder(log_level: log::LevelFilter) -> Self;
    fn log_file_dir(self, log_file_dir: impl Into<PathBuf>) -> Self;
}

impl LoggerBuilder for flexi_logger::Logger {
    fn builder(log_level: log::LevelFilter) -> Self {
        Self::with(log_level).format_for_stderr(
            |write, deferred_now, record| -> std::io::Result<()> {
                write.write_all(
                    Self::fmt_str(
                        deferred_now.format_rfc3339(),
                        record.level().paint(record.level()),
                        record.args(),
                    )
                    .as_bytes(),
                )
            },
        )
    }
    fn log_file_dir(self, log_file_dir: impl Into<PathBuf>) -> Self {
        self.log_to_file(flexi_logger::FileSpec::default().directory(log_file_dir))
            .duplicate_to_stderr(flexi_logger::Duplicate::All)
            .format_for_files(|write, deferred_now, record| -> std::io::Result<()> {
                write.write_all(
                    Self::fmt_str(deferred_now.format_rfc3339(), record.level(), record.args())
                        .as_bytes(),
                )
            })
    }
}

impl LoggerBuilder for tauri_plugin_log::Builder {
    fn builder(log_level: log::LevelFilter) -> Self {
        Self::new()
            .clear_targets()
            .level(log_level)
            .target(tauri_plugin_log::Target::new(
                tauri_plugin_log::TargetKind::Stderr,
            ))
            .format(|format_callback, arguments, record| {
                format_callback.finish(format_args!("[{}]:{}", record.level(), arguments))
            })
    }
    fn log_file_dir(self, log_file_dir: impl Into<PathBuf>) -> Self {
        self.target(tauri_plugin_log::Target::new(
            tauri_plugin_log::TargetKind::Folder {
                path: log_file_dir.into(),
                file_name: None,
            },
        ))
    }
}
