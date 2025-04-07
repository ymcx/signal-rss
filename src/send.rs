use dbus::blocking::{LocalConnection, Proxy};
use std::{sync::Arc, time::Duration};

#[allow(dead_code)]
pub struct DBUS {
    connection: Proxy<'static, Arc<LocalConnection>>,
}

impl DBUS {
    pub fn new() -> Self {
        let conn = Arc::new(LocalConnection::new_session().unwrap());
        let connection = Proxy::new(
            "org.asamk.Signal",
            "/org/asamk/Signal",
            Duration::from_secs(5),
            conn.clone(),
        );

        Self { connection }
    }

    pub fn send(&self, _message: &str, _group: &Vec<u8>) {
        // println!("sending {}", message);
        // let _: Result<(Vec<String>,), dbus::Error> = self.connection.method_call(
        //     "org.asamk.Signal",
        //     "sendGroupMessage",
        //     (message, Vec::<&str>::new(), group),
        // );
    }
}
