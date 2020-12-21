pub use logger::json_guard;
pub use logger::slog_macro;
pub use logger::term_guard;
pub use logger::GuardLogger;
pub use once_cell::sync::OnceCell;

pub type GuardLoggerCell = OnceCell<GuardLogger>;

mod logger;
