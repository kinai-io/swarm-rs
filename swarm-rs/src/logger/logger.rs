use std::{
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::{file_io, time::today_with_format};

#[derive(Serialize, Deserialize)]
struct LogEntry {
    level: String,
    event_type: String,
    data: Value,
    timestamp: String,
}

pub struct Logger {
    file_prefix: String,
    base_dir: PathBuf,
    file_name: String,
    file: Arc<Mutex<std::fs::File>>

}

impl Logger {

    pub fn new<T: AsRef<Path>>(base_dir: T, file_prefix: &str) -> Self {
        let file_name = Logger::current_file_name(file_prefix);
        
        let file_path = Path::new(base_dir.as_ref()).join(&file_name);

        file_io::create_parent_dirs(&file_path);

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)
            .expect("Failed to open or create log file");

        Self {
            file_prefix: file_prefix.to_string(),
            base_dir: PathBuf::from(base_dir.as_ref()),
            file_name,
            file: Arc::new(Mutex::new(file)),
        }
    }

    fn current_file_name(file_prefix: &str) -> String {
        let date = today_with_format("%Y%m%d");
        format!("{}-{}.log", file_prefix, date)
    }

    pub fn log<T: Serialize>(&self, level: &str, event_type: &str, data: &T) {
        let json_data = serde_json::to_string(data).expect("Failed to serialize log data");

        let current_file = Self::current_file_name(&self.file_prefix);
        let mut file = if &current_file == &self.file_name {
            let file = self.file.lock().expect("Failed to lock log file");
            file
        }else {
            let mut file = self.file.lock().expect("Failed to lock log file");
            let file_path = Path::new(&self.base_dir).join(&current_file);
            file_io::create_parent_dirs(&file_path);
            let new_file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(file_path)
                .expect("Failed to open or create log file");

            *file = new_file;
            file
        };

        let timestamp = chrono::Local::now().to_rfc3339();
        let pid = std::process::id();
        writeln!(file, "[{}] {} - {} - {} - {}", pid, timestamp, level, event_type, json_data)
            .expect("Failed to write to log file");
    }

    pub fn trace<T: Serialize>(&self, event_type: &str, data: &T) {
        self.log("trace", event_type, data);
    }

    pub fn debug<T: Serialize>(&self, event_type: &str, data: &T) {
        self.log("debug", event_type, data);
    }

    pub fn info<T: Serialize>(&self, event_type: &str, data: &T) {
        self.log("info",event_type, data);
    }

    pub fn warn<T: Serialize>(&self, event_type: &str, data: &T) {
        self.log("warn", event_type, data);
    }

    pub fn error<T: Serialize>(&self, event_type: &str, data: &T) {
        self.log("error",event_type, data);
    }

}
