pub use once_cell::sync::OnceCell;

pub use logger::{json_guard, term_guard, GuardLogger, slog_macro};
pub use logger::macros::*;

pub type GuardLoggerCell = OnceCell<GuardLogger>;

mod logger;
