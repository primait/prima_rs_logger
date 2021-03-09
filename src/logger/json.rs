use std::cell::RefCell;
use std::{collections::HashMap, io};

use slog::{Drain, OwnedKVList, Record, KV};

use super::message::{AdditionalFields, Fields, Message};

pub struct JsonDrain<W: io::Write> {
    app: String,
    output: RefCell<W>,
}

impl<W: io::Write> JsonDrain<W> {
    pub fn new<T>(app: T, d: W) -> Self
    where
        T: Into<String>,
    {
        Self {
            app: app.into(),
            output: RefCell::new(d),
        }
    }
}

impl<W: io::Write> Drain for JsonDrain<W> {
    type Ok = ();
    type Err = io::Error;
    fn log(&self, record: &Record, values: &OwnedKVList) -> Result<Self::Ok, Self::Err> {
        let mut additional = AdditionalFields(HashMap::with_capacity(16));
        record.kv().serialize(record, &mut additional)?;
        values.serialize(record, &mut additional)?;

        let target: &str = if record.tag().is_empty() {
            record.location().module
        } else {
            record.tag()
        };

        let metadata = Fields {
            target,
            file: record.location().file,
            line: record.location().line,
            additional: additional.0,
        };

        let level: String = record.level().as_str().to_lowercase();

        let message = Message {
            timestamp: chrono::Utc::now(),
            app: self.app.as_str(),
            level: level.as_str(),
            message: record.msg().to_string(),
            metadata,
        };

        let serialized: String = serde_json::to_string(&message)?;

        let mut output = self.output.borrow_mut();

        writeln!(&mut output, "{}", serialized)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use slog::{info, o, Drain};

    use crate::logger::message::Message;

    use super::JsonDrain;

    struct MockWriter {
        entries: Arc<Mutex<Vec<Vec<u8>>>>,
    }

    impl std::io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let mut entries = self.entries.lock().unwrap();
            entries.push(buf.to_vec());
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    fn setup_test_logger(name: &str) -> (slog::Logger, Arc<Mutex<Vec<Vec<u8>>>>) {
        let entries = Arc::new(Mutex::new(vec![]));
        let writer = MockWriter {
            entries: entries.clone(),
        };

        let term_drain = Mutex::new(JsonDrain::new(name, writer)).map(slog::Fuse);

        let logger = slog::Logger::root(term_drain.fuse(), o!());

        (logger, entries)
    }

    // without metadata
    #[test]
    fn json_logger_test_without_metadata() {
        let (logger, entries) = setup_test_logger("json_logger_test_without_metadata");

        info!(logger, "Logger Test");

        let events = entries.lock().unwrap();

        let last = events.get(0).expect("Index at 0 is not present");

        let message: Message = serde_json::from_slice(last).unwrap();

        assert_eq!("json_logger_test_without_metadata", message.app);
        assert_eq!("Logger Test", message.message);
        assert_eq!(None, message.metadata.additional.get("build"));
    }
    // without metadata
    #[test]
    fn json_logger_test_with_metadata() {
        let (logger, entries) = setup_test_logger("json_logger_test_with_metadata");

        info!(logger, "Logger Test"; "build_id" => "unknown");

        let events = entries.lock().unwrap();

        let last = events.get(0).expect("Index at 0 is not present");

        let message: Message = serde_json::from_slice(last).unwrap();

        assert_eq!("json_logger_test_with_metadata", message.app);
        assert_eq!("Logger Test", message.message);
        assert_eq!(
            Some("unknown"),
            message
                .metadata
                .additional
                .get("build_id")
                .map(|s| s.as_ref())
        );
    }
}
