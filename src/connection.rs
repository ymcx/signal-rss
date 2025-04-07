use dbus::blocking::LocalConnection;
use std::{error::Error, time::Duration};

pub struct SignalConnection {
    connection: LocalConnection,
}

impl SignalConnection {
    pub fn new() -> Self {
        match LocalConnection::new_session() {
            Ok(connection) => Self { connection },
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }
    }

    pub fn send(&self, message: &str, group: &[u8]) -> Result<(), Box<dyn Error>> {
        self.connection
            .with_proxy(
                "org.asamk.Signal",
                "/org/asamk/Signal",
                Duration::from_secs(5),
            )
            .method_call::<(i64,), (&str, &[&str], &[u8]), &str, &str>(
                "org.asamk.Signal",
                "sendGroupMessage",
                (message, &[], group),
            )?;

        Ok(())
    }
}
