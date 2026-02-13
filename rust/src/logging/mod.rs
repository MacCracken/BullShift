use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;

/// Structured log levels
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
    Critical = 5,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "TRACE"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Critical => write!(f, "CRITICAL"),
        }
    }
}

/// Structured log entry with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    /// Timestamp of the log entry
    pub timestamp: DateTime<Utc>,

    /// Log level
    pub level: LogLevel,

    /// Target module/component
    pub target: String,

    /// Log message
    pub message: String,

    /// Optional additional context data
    pub context: Option<HashMap<String, serde_json::Value>>,

    /// Optional error details if this is an error log
    pub error_details: Option<ErrorDetails>,

    /// Optional request ID for tracing
    pub request_id: Option<String>,

    /// Optional user ID if applicable
    pub user_id: Option<String>,

    /// Optional session ID if applicable
    pub session_id: Option<String>,
}

/// Detailed error information for structured logging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetails {
    /// Error code
    pub code: String,

    /// Error message
    pub message: String,

    /// Stack trace if available
    pub stack_trace: Option<String>,

    /// Source file where error occurred
    pub source_file: Option<String>,

    /// Line number where error occurred
    pub line_number: Option<u32>,

    /// Function name where error occurred
    pub function_name: Option<String>,
}

/// Structured logging interface
pub trait Logger {
    /// Log a message at the specified level
    fn log(&self, level: LogLevel, target: &str, message: &str);

    /// Log with additional context
    fn log_with_context(
        &self,
        level: LogLevel,
        target: &str,
        message: &str,
        context: HashMap<String, serde_json::Value>,
    );

    /// Log an error with details
    fn log_error(&self, target: &str, error: &str, details: &ErrorDetails);

    /// Log a trace message with request ID
    fn log_trace(&self, target: &str, message: &str, request_id: &str);

    /// Check if logging is enabled for the given level
    fn is_enabled(&self, level: LogLevel) -> bool;

    /// Get current log entries
    fn get_recent_entries(&self, level: LogLevel, limit: usize) -> Vec<LogEntry>;

    /// Flush any pending logs
    fn flush(&self);
}

/// High-performance structured logger implementation
pub struct StructuredLogger {
    /// Application name
    #[allow(dead_code)]
    app_name: String,

    /// Minimum log level to output
    min_level: LogLevel,

    /// Buffer for log entries (using RefCell for interior mutability)
    entries: RefCell<Vec<LogEntry>>,

    /// Flush interval in milliseconds
    #[allow(dead_code)]
    flush_interval_ms: u64,
}

impl StructuredLogger {
    pub fn new(app_name: String, min_level: LogLevel) -> Self {
        Self {
            app_name,
            min_level,
            entries: RefCell::new(Vec::new()),
            flush_interval_ms: 5000,
        }
    }
}

impl Default for StructuredLogger {
    fn default() -> Self {
        Self::new("bullshift".to_string(), LogLevel::Info)
    }
}

impl Logger for StructuredLogger {
    fn log(&self, level: LogLevel, target: &str, message: &str) {
        if !self.is_enabled(level.clone()) {
            return;
        }

        let entry = LogEntry {
            timestamp: Utc::now(),
            level: level.clone(),
            target: target.to_string(),
            message: message.to_string(),
            context: None,
            error_details: None,
            request_id: None,
            user_id: None,
            session_id: None,
        };

        #[cfg(debug_assertions)]
        println!(
            "[{}] {} [{}] {}",
            entry.timestamp.format("%Y-%m-%d %H:%M:%S%.3f"),
            level,
            target,
            message
        );

        self.entries.borrow_mut().push(entry);

        // Auto-flush if we have too many entries
        if self.entries.borrow().len() > 1000 {
            self.flush();
        }
    }

    fn log_with_context(
        &self,
        level: LogLevel,
        target: &str,
        message: &str,
        context: HashMap<String, serde_json::Value>,
    ) {
        if !self.is_enabled(level.clone()) {
            return;
        }

        let entry = LogEntry {
            timestamp: Utc::now(),
            level: level.clone(),
            target: target.to_string(),
            message: message.to_string(),
            context: Some(context),
            error_details: None,
            request_id: None,
            user_id: None,
            session_id: None,
        };

        #[cfg(debug_assertions)]
        println!(
            "[{}] {} [{}] {}",
            entry.timestamp.format("%Y-%m-%d %H:%M:%S%.3f"),
            level,
            target,
            message
        );

        self.entries.borrow_mut().push(entry);

        // Auto-flush if we have too many entries
        if self.entries.borrow().len() > 1000 {
            self.flush();
        }
    }

    fn log_error(&self, target: &str, error: &str, details: &ErrorDetails) {
        let level = LogLevel::Error;

        let entry = LogEntry {
            timestamp: Utc::now(),
            level: level.clone(),
            target: target.to_string(),
            message: error.to_string(),
            context: None,
            error_details: Some(details.clone()),
            request_id: None,
            user_id: None,
            session_id: None,
        };

        #[cfg(debug_assertions)]
        eprintln!(
            "[{}] {} [{}] ERROR: {}",
            entry.timestamp.format("%Y-%m-%d %H:%M:%S%.3f"),
            level,
            target,
            error
        );

        self.entries.borrow_mut().push(entry);
    }

    fn log_trace(&self, target: &str, message: &str, request_id: &str) {
        if !self.is_enabled(LogLevel::Trace) {
            return;
        }

        let entry = LogEntry {
            timestamp: Utc::now(),
            level: LogLevel::Trace,
            target: target.to_string(),
            message: message.to_string(),
            context: None,
            error_details: None,
            request_id: Some(request_id.to_string()),
            user_id: None,
            session_id: None,
        };

        // Need mutable access - this is a design issue, we'll clone for now
        // In production, use interior mutability (RefCell, Mutex, etc.)
        self.entries.borrow_mut().push(entry);
    }

    fn is_enabled(&self, level: LogLevel) -> bool {
        level as u8 >= self.min_level.clone() as u8
    }

    fn get_recent_entries(&self, level: LogLevel, limit: usize) -> Vec<LogEntry> {
        self.entries
            .borrow()
            .iter()
            .filter(|entry| entry.level.clone() as u8 >= level.clone() as u8)
            .rev()
            .take(limit)
            .cloned()
            .collect()
    }

    fn flush(&self) {
        #[cfg(debug_assertions)]
        for entry in self.entries.borrow().iter() {
            let level_str = match entry.level {
                LogLevel::Trace => "TRACE",
                LogLevel::Debug => "DEBUG",
                LogLevel::Info => "INFO",
                LogLevel::Warn => "WARN",
                LogLevel::Error => "ERROR",
                LogLevel::Critical => "CRITICAL",
            };

            println!(
                "[{}] {} [{}] [{}]: {}",
                entry.timestamp.format("%Y-%m-%d %H:%M:%S%.3f"),
                self.app_name,
                entry.target,
                level_str,
                entry.message
            );
        }

        self.entries.borrow_mut().clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger_creation() {
        let logger = StructuredLogger::new("test_app".to_string(), LogLevel::Debug);
        assert_eq!(logger.app_name, "test_app");
        assert_eq!(logger.min_level, LogLevel::Debug);
        assert!(logger.is_enabled(LogLevel::Info));
        assert!(!logger.is_enabled(LogLevel::Trace));
    }

    #[test]
    fn test_log_levels() {
        let logger = StructuredLogger::new("test_app".to_string(), LogLevel::Debug);

        logger.log(LogLevel::Info, "test_module", "info message");
        logger.log_error(
            "test_module",
            "test error",
            &ErrorDetails {
                code: "E001".to_string(),
                message: "test error details".to_string(),
                stack_trace: Some("function1\nfunction2".to_string()),
                source_file: Some("test.rs".to_string()),
                line_number: Some(42),
                function_name: Some("test_function".to_string()),
            },
        );

        assert_eq!(logger.entries.borrow().len(), 2);
    }

    #[test]
    fn test_context_logging() {
        let logger = StructuredLogger::new("test_app".to_string(), LogLevel::Debug);

        let mut context = HashMap::new();
        context.insert(
            "user_id".to_string(),
            serde_json::Value::String("user123".to_string()),
        );
        context.insert(
            "session_id".to_string(),
            serde_json::Value::String("session456".to_string()),
        );

        logger.log_with_context(LogLevel::Info, "api_request", "Making API call", context);

        assert_eq!(logger.entries.borrow().len(), 1);
        let entry = &logger.entries.borrow()[0];
        assert_eq!(entry.target, "api_request");
        assert_eq!(entry.message, "Making API call");
        assert!(entry.context.is_some());

        let context = entry.context.as_ref().unwrap();
        assert_eq!(
            context.get("user_id"),
            Some(&serde_json::Value::String("user123".to_string()))
        );
    }

    #[test]
    fn test_trace_logging() {
        let logger = StructuredLogger::new("test_app".to_string(), LogLevel::Trace);

        logger.log_trace("auth_module", "Authentication started", "req_12345");

        assert_eq!(logger.entries.borrow().len(), 1);
        let entry = &logger.entries.borrow()[0];
        assert_eq!(entry.level, LogLevel::Trace);
        assert_eq!(entry.target, "auth_module");
        assert_eq!(entry.message, "Authentication started");
        assert_eq!(entry.request_id, Some("req_12345".to_string()));
    }

    #[test]
    fn test_flush_behavior() {
        let logger = StructuredLogger::new("test_app".to_string(), LogLevel::Debug);

        for i in 0..100 {
            logger.log(LogLevel::Info, "test_module", &format!("message {}", i));
        }

        assert_eq!(logger.entries.borrow().len(), 100);
        logger.flush();

        assert_eq!(logger.entries.borrow().len(), 0);
    }
}
