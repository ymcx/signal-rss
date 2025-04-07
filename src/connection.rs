use dbus::blocking::LocalConnection;
use std::{error::Error, time::Duration};

pub struct SignalConnection {
    connection: LocalConnection,
}

impl SignalConnection {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let connection = LocalConnection::new_session()?;
        Ok(Self { connection })
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
