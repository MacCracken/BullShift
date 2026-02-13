use bullshift_core::logging::{LogLevel, Logger, StructuredLogger};
use bullshift_core::security::SecurityManager;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;

fn bench_logging(c: &mut Criterion) {
    let logger = StructuredLogger::new("bench".to_string(), LogLevel::Info);

    c.bench_function("log_simple_message", |b| {
        b.iter(|| {
            logger.log(
                black_box(LogLevel::Info),
                black_box("bench"),
                black_box("Test message"),
            );
        });
    });

    c.bench_function("log_with_context", |b| {
        let mut context = HashMap::new();
        context.insert(
            "key1".to_string(),
            serde_json::Value::String("value1".to_string()),
        );
        context.insert(
            "key2".to_string(),
            serde_json::Value::String("value2".to_string()),
        );

        b.iter(|| {
            logger.log_with_context(
                black_box(LogLevel::Info),
                black_box("bench"),
                black_box("Test message with context"),
                black_box(context.clone()),
            );
        });
    });

    c.bench_function("log_error", |b| {
        let details = bullshift_core::logging::ErrorDetails {
            code: "E001".to_string(),
            message: "Error message".to_string(),
            stack_trace: None,
            source_file: Some("bench.rs".to_string()),
            line_number: Some(42),
            function_name: Some("bench_function".to_string()),
        };

        b.iter(|| {
            logger.log_error(
                black_box("bench"),
                black_box("Error occurred"),
                black_box(&details),
            );
        });
    });

    c.bench_function("get_recent_entries", |b| {
        for i in 0..100 {
            logger.log(LogLevel::Info, "bench", &format!("Message {}", i));
        }

        b.iter(|| {
            logger.get_recent_entries(black_box(LogLevel::Info), black_box(10));
        });
    });

    c.bench_function("flush", |b| {
        for i in 0..50 {
            logger.log(LogLevel::Info, "bench", &format!("Message {}", i));
        }

        b.iter(|| {
            logger.flush();
        });
    });
}

fn bench_security(c: &mut Criterion) {
    c.bench_function("security_manager_new", |b| {
        b.iter(|| SecurityManager::new().ok());
    });

    c.bench_function("encrypt_small_data", |b| {
        let sm = SecurityManager::new().unwrap();

        b.iter(|| sm.encrypt_sensitive_data(black_box("test_data")));
    });

    c.bench_function("encrypt_large_data", |b| {
        let sm = SecurityManager::new().unwrap();
        let large_data = "x".repeat(10000);

        b.iter(|| sm.encrypt_sensitive_data(black_box(large_data.as_str())));
    });
}

fn bench_log_level_check(c: &mut Criterion) {
    let logger = StructuredLogger::new("bench".to_string(), LogLevel::Info);

    c.bench_function("is_enabled_check", |b| {
        b.iter(|| {
            logger.is_enabled(black_box(LogLevel::Debug));
        });
    });
}

criterion_group!(
    benches,
    bench_logging,
    bench_security,
    bench_log_level_check
);
criterion_main!(benches);
