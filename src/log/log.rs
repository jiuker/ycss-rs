use lazy_static::lazy_static;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
// web_log!(LOGCH, "set config path is {}", path);
#[macro_export]
macro_rules! web_log {
    ($($data:expr),*) => {
        {
            LOGCH.lock().unwrap().send(format!($($data),*));
        }
    };
}

pub struct LogChDef {
    s: Sender<String>,
    r: Receiver<String>,
}

impl LogChDef {
    pub fn new() -> Self {
        let (s, r) = channel::<String>();
        LogChDef { s, r }
    }
    pub fn send(&mut self, data: String) {
        self.s.send(data).unwrap()
    }
    pub fn receive(&mut self) -> String {
        let data = match self.r.try_recv() {
            Ok(d) => d,
            Err(_) => "".to_string(),
        };
        data
    }
}
lazy_static! {
    pub static ref LOGCH: Arc<Mutex<LogChDef>> = Arc::new(Mutex::new(LogChDef::new()));
}
