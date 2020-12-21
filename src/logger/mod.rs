mod json;

#[macro_use]
mod macros;
mod message;
mod terminal;

pub use macros::*;

use slog::{o, Drain, Logger as SLogger};
use slog_scope::GlobalLoggerGuard;
use std::sync::Arc;

/// `GuardLogger` embed `GlobalLoggerGuard` of slog.
///
/// On drop it will reset global logger to `slog::Discard`.
/// This will `drop` any existing global logger.

#[must_use]
pub struct GuardLogger {
    pub guard: GlobalLoggerGuard,
}

impl std::fmt::Debug for GuardLogger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GuardLogger").finish()
    }
}

pub mod slog_macro {
    pub use slog::{debug, error, info, trace, warn};
    pub use slog_scope::with_logger;
}

pub fn json_guard(app_name: &str) -> GuardLogger {
    with_async(json_async_drain(app_name))
}

pub fn term_guard(app_name: &str) -> GuardLogger {
    with_async(term_async_drain(app_name))
}

fn with_async(drain: slog::Fuse<slog_async::Async>) -> GuardLogger {
    let logger = SLogger::root(Arc::new(drain).fuse(), o!());

    let scope_guard = slog_scope::set_global_logger(logger);

    //this forwards the logging from other crates ex. warp

    slog_stdlog::init().expect("Failed to init slog_stdlog compatibility layer");
    // slog_stdlog::init_with_level(log::Level::Info).unwrap();

    GuardLogger { guard: scope_guard }
}

fn term_async_drain(app_name: &str) -> slog::Fuse<slog_async::Async> {
    let decorator = slog_term::TermDecorator::new().build();
    let custom_format = terminal::CustomFormat::new(app_name, decorator);
    let env = slog_envlogger::new(custom_format).fuse();
    slog_async::Async::new(env).build().fuse()
}

fn json_async_drain(app_name: &str) -> slog::Fuse<slog_async::Async> {
    let json = json::JsonDrain::new(app_name, std::io::stdout());
    let env = slog_envlogger::new(json).fuse();
    slog_async::Async::new(env).build().fuse()
}
