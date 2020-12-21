use slog::{Drain, OwnedKVList, Record, KV};
use slog_term::{CountingWriter, Decorator, RecordDecorator, Serializer, ThreadSafeTimestampFn};
use std::io::Write;
use std::{io, result};

const TIMESTAMP_FORMAT: &str = "%FT%T%.3f";

/// Terminal-output formatting `Drain`
///
/// It's a clone of `slog_term::FullFormat`
///
/// **Note**: logging to `CustomFormat` drain is thread-safe, since every
/// line of output is formatted indecently. However, the underlying
/// IO, needs to be synchronized.
pub struct CustomFormat<D>
where
    D: Decorator,
{
    app: String,
    decorator: D,
    fn_timestamp: Box<dyn ThreadSafeTimestampFn<Output = io::Result<()>>>,
    use_original_order: bool,
}

impl<D> Drain for CustomFormat<D>
where
    D: Decorator,
{
    type Ok = ();
    type Err = io::Error;

    fn log(&self, record: &Record, values: &OwnedKVList) -> result::Result<Self::Ok, Self::Err> {
        self.format_full(record, values)
    }
}

impl<D> CustomFormat<D>
where
    D: Decorator,
{
    pub fn new<T>(app: T, d: D) -> Self
    where
        T: Into<String>,
    {
        Self {
            app: app.into(),
            fn_timestamp: Box::new(timestamp_local),
            decorator: d,
            use_original_order: false,
        }
    }

    fn format_full(&self, record: &Record, values: &OwnedKVList) -> io::Result<()> {
        self.decorator.with_record(record, values, |decorator| {
            let comma_needed = print_msg_header(&self.app, &*self.fn_timestamp, decorator, record)?;
            {
                let mut serializer =
                    Serializer::new(decorator, comma_needed, self.use_original_order);

                record.kv().serialize(record, &mut serializer)?;

                values.serialize(record, &mut serializer)?;

                serializer.finish()?;
            }

            decorator.start_whitespace()?;
            writeln!(decorator)?;

            decorator.flush()?;

            Ok(())
        })
    }
}

/// Default local timezone timestamp function
///
/// The exact format used, is still subject to change.
pub fn timestamp_local(io: &mut dyn io::Write) -> io::Result<()> {
    write!(io, "{}", chrono::Local::now().format(TIMESTAMP_FORMAT))
}

/// Returns `true` if message was not empty
/// this defines how the message will be printed on the terminal
pub fn print_msg_header(
    _app: &str,
    fn_timestamp: &dyn ThreadSafeTimestampFn<Output = io::Result<()>>,
    mut rd: &mut dyn RecordDecorator,
    record: &Record,
) -> io::Result<bool> {
    rd.start_timestamp()?;
    fn_timestamp(&mut rd)?;

    rd.start_whitespace()?;
    write!(rd, " ")?;

    let target = if record.tag().is_empty() {
        record.location().module
    } else {
        record.tag()
    };

    write!(rd, "[{}:{}]", target, record.location().line)?;

    rd.start_level()?;
    write!(rd, "[{}]", record.level().as_str())?;

    rd.start_whitespace()?;
    write!(rd, " ")?;

    rd.start_msg()?;
    let mut count_rd = CountingWriter::new(&mut rd);
    write!(count_rd, "{}", record.msg())?;
    Ok(count_rd.count() != 0)
}
