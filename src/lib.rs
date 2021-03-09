pub use once_cell::sync::OnceCell;

pub use logger::macros::*;
pub use logger::{json_guard, slog_macro, term_guard, GuardLogger};

pub type GuardLoggerCell = OnceCell<GuardLogger>;

mod logger;
