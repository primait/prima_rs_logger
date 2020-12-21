/// # Example info! macro
///
/// ```rust,no_run
/// paperboy::logger::info!("Info {}", "param"; "meta" => "data" );
/// ```

#[macro_export(local_inner_macros)]
macro_rules! info {
        ( $($args:tt)* ) => {
            $crate::slog_macro::with_logger(|l| {
                $crate::slog_macro::info!(l, $($args)* )
            });
        };
        (#$tag:expr, $($args:tt)* ) => {
            $crate::slog_macro::with_logger(|l| {
                $crate::slog_macro::info!(l, #$tag, $($args)* )
            });
        };
    }

/// # Example warn! macro
///
/// ```rust,no_run
/// paperboy::logger::warn!("Warn {}", "param"; "meta" => "data" );
/// ```

// This macro is internally called `v_warn` since `pub use warn` is forbidden.
// but exposed as `warn`
#[macro_export(local_inner_macros)]
macro_rules! v_warn {
        ( $($args:tt)* ) => {
            $crate::slog_macro::with_logger(|l| {
                $crate::slog_macro::warn!(l, $($args)* )
            });
        };
        (#$tag:expr, $($args:tt)* ) => {
            $crate::slog_macro::with_logger(|l| {
                $crate::slog_macro::warn!(l, #$tag, $($args)* )
            });
        };
    }

/// # Example debug! macro
///
/// ```rust,no_run
/// paperboy::logger::debug!("Debug {}", "param"; "meta" => "data" );
/// ```

#[macro_export(local_inner_macros)]
macro_rules! debug {
        ( $($args:tt)* ) => {
            $crate::slog_macro::with_logger(|l| {
                $crate::slog_macro::debug!(l, $($args)* )
            });
        };
        (#$tag:expr, $($args:tt)* ) => {
            $crate::slog_macro::with_logger(|l| {
                $crate::slog_macro::debug!(l, #$tag, $($args)* )
            });
        };
    }

/// # Example trace! macro
///
/// ```rust,no_run
/// paperboy::logger::trace!("Trace {}", "param"; "meta" => "data" );
/// ```

#[macro_export(local_inner_macros)]
macro_rules! trace {
        ( $($args:tt)* ) => {
            $crate::slog_macro::with_logger(|l| {
                $crate::slog_macro::trace!(l, $($args)* )
            });
        };
        (#$tag:expr, $($args:tt)* ) => {
            $crate::slog_macro::with_logger(|l| {
                $crate::slog_macro::trace!(l, #$tag, $($args)* )
            });
        };
    }

/// # Example error! macro
///
/// ```rust,no_run
/// paperboy::logger::error!("Error {}", "param"; "meta" => "data" );
/// ```

#[macro_export(local_inner_macros)]
macro_rules! error {
        ( $($args:tt)* ) => {
            $crate::slog_macro::with_logger(|l| {
                $crate::slog_macro::error!(l, $($args)* )
            });
        };
        (#$tag:expr, $($args:tt)* ) => {
            $crate::slog_macro::with_logger(|l| {
                $crate::slog_macro::slog_error!(l, #$tag, $($args)* )
            });
        };
    }

pub use debug;
pub use error;
pub use info;
pub use trace;
pub use v_warn as warn;
