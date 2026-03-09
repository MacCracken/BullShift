//! Monitoring and alerting module for BullShift production deployments.
//!
//! Provides health checks, metrics collection, and configurable alert rules.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Health checks
// ---------------------------------------------------------------------------

/// Overall system health status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

impl std::fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Healthy => write!(f, "healthy"),
            Self::Degraded => write!(f, "degraded"),
            Self::Unhealthy => write!(f, "unhealthy"),
        }
    }
}

/// Result of a single component health check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    pub name: String,
    pub status: HealthStatus,
    pub message: Option<String>,
    pub latency_ms: Option<f64>,
    pub checked_at: DateTime<Utc>,
}

/// Aggregated health report for the entire system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthReport {
    pub status: HealthStatus,
    pub version: String,
    pub uptime_seconds: u64,
    pub components: Vec<ComponentHealth>,
    pub timestamp: DateTime<Utc>,
}

/// Checks health of core subsystems and produces a report.
pub struct HealthChecker {
    version: String,
    started_at: DateTime<Utc>,
    checks: Vec<Box<dyn Fn() -> ComponentHealth + Send + Sync>>,
}

impl HealthChecker {
    pub fn new(version: &str) -> Self {
        Self {
            version: version.to_string(),
            started_at: Utc::now(),
            checks: Vec::new(),
        }
    }

    /// Register a named health check function.
    pub fn add_check<F>(&mut self, check: F)
    where
        F: Fn() -> ComponentHealth + Send + Sync + 'static,
    {
        self.checks.push(Box::new(check));
    }

    /// Run all registered checks and produce a report.
    pub fn check(&self) -> HealthReport {
        let components: Vec<ComponentHealth> = self.checks.iter().map(|c| c()).collect();

        let status = if components.iter().all(|c| c.status == HealthStatus::Healthy) {
            HealthStatus::Healthy
        } else if components
            .iter()
            .any(|c| c.status == HealthStatus::Unhealthy)
        {
            HealthStatus::Unhealthy
        } else {
            HealthStatus::Degraded
        };

        let uptime = (Utc::now() - self.started_at).num_seconds().max(0) as u64;

        HealthReport {
            status,
            version: self.version.clone(),
            uptime_seconds: uptime,
            components,
            timestamp: Utc::now(),
        }
    }
}

// ---------------------------------------------------------------------------
// Metrics
// ---------------------------------------------------------------------------

/// Atomic counter metric.
pub struct Counter {
    name: String,
    value: AtomicU64,
    labels: HashMap<String, String>,
}

impl Counter {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            value: AtomicU64::new(0),
            labels: HashMap::new(),
        }
    }

    pub fn with_label(mut self, key: &str, value: &str) -> Self {
        self.labels.insert(key.to_string(), value.to_string());
        self
    }

    pub fn increment(&self) {
        self.value.fetch_add(1, Ordering::Relaxed);
    }

    pub fn add(&self, n: u64) {
        self.value.fetch_add(n, Ordering::Relaxed);
    }

    pub fn get(&self) -> u64 {
        self.value.load(Ordering::Relaxed)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn labels(&self) -> &HashMap<String, String> {
        &self.labels
    }
}

/// Point-in-time gauge metric.
pub struct Gauge {
    name: String,
    /// Stored as integer bits of an f64 for atomic access.
    bits: AtomicU64,
    labels: HashMap<String, String>,
}

impl Gauge {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            bits: AtomicU64::new(0f64.to_bits()),
            labels: HashMap::new(),
        }
    }

    pub fn with_label(mut self, key: &str, value: &str) -> Self {
        self.labels.insert(key.to_string(), value.to_string());
        self
    }

    pub fn set(&self, val: f64) {
        self.bits.store(val.to_bits(), Ordering::Relaxed);
    }

    pub fn get(&self) -> f64 {
        f64::from_bits(self.bits.load(Ordering::Relaxed))
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn labels(&self) -> &HashMap<String, String> {
        &self.labels
    }
}

/// Simple histogram that tracks min, max, sum, count for a distribution.
#[derive(Debug)]
pub struct Histogram {
    name: String,
    min: f64,
    max: f64,
    sum: f64,
    count: u64,
    labels: HashMap<String, String>,
}

impl Histogram {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            min: f64::MAX,
            max: f64::MIN,
            sum: 0.0,
            count: 0,
            labels: HashMap::new(),
        }
    }

    pub fn with_label(mut self, key: &str, value: &str) -> Self {
        self.labels.insert(key.to_string(), value.to_string());
        self
    }

    pub fn observe(&mut self, value: f64) {
        self.count += 1;
        self.sum += value;
        if value < self.min {
            self.min = value;
        }
        if value > self.max {
            self.max = value;
        }
    }

    pub fn count(&self) -> u64 {
        self.count
    }

    pub fn mean(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            self.sum / self.count as f64
        }
    }

    pub fn min(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            self.min
        }
    }

    pub fn max(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            self.max
        }
    }

    pub fn sum(&self) -> f64 {
        self.sum
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn labels(&self) -> &HashMap<String, String> {
        &self.labels
    }
}

/// Snapshot of a single metric for serialization / export.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricSnapshot {
    pub name: String,
    pub kind: String,
    pub value: f64,
    pub labels: HashMap<String, String>,
    pub timestamp: DateTime<Utc>,
}

/// Registry that owns all application metrics.
pub struct MetricsRegistry {
    counters: Vec<Counter>,
    gauges: Vec<Gauge>,
    histograms: Vec<Histogram>,
}

impl MetricsRegistry {
    pub fn new() -> Self {
        Self {
            counters: Vec::new(),
            gauges: Vec::new(),
            histograms: Vec::new(),
        }
    }

    pub fn add_counter(&mut self, counter: Counter) -> usize {
        let idx = self.counters.len();
        self.counters.push(counter);
        idx
    }

    pub fn add_gauge(&mut self, gauge: Gauge) -> usize {
        let idx = self.gauges.len();
        self.gauges.push(gauge);
        idx
    }

    pub fn add_histogram(&mut self, histogram: Histogram) -> usize {
        let idx = self.histograms.len();
        self.histograms.push(histogram);
        idx
    }

    pub fn counter(&self, idx: usize) -> Option<&Counter> {
        self.counters.get(idx)
    }

    pub fn gauge(&self, idx: usize) -> Option<&Gauge> {
        self.gauges.get(idx)
    }

    pub fn histogram(&self, idx: usize) -> Option<&Histogram> {
        self.histograms.get(idx)
    }

    pub fn histogram_mut(&mut self, idx: usize) -> Option<&mut Histogram> {
        self.histograms.get_mut(idx)
    }

    /// Export all metrics as snapshots.
    pub fn snapshot(&self) -> Vec<MetricSnapshot> {
        let now = Utc::now();
        let mut out = Vec::new();

        for c in &self.counters {
            out.push(MetricSnapshot {
                name: c.name().to_string(),
                kind: "counter".to_string(),
                value: c.get() as f64,
                labels: c.labels().clone(),
                timestamp: now,
            });
        }
        for g in &self.gauges {
            out.push(MetricSnapshot {
                name: g.name().to_string(),
                kind: "gauge".to_string(),
                value: g.get(),
                labels: g.labels().clone(),
                timestamp: now,
            });
        }
        for h in &self.histograms {
            // Emit count, mean, min, max as separate snapshots
            for (suffix, val) in [
                ("_count", h.count() as f64),
                ("_mean", h.mean()),
                ("_min", h.min()),
                ("_max", h.max()),
            ] {
                out.push(MetricSnapshot {
                    name: format!("{}{}", h.name(), suffix),
                    kind: "histogram".to_string(),
                    value: val,
                    labels: h.labels().clone(),
                    timestamp: now,
                });
            }
        }

        out
    }

    /// Render all metrics in Prometheus text exposition format.
    pub fn prometheus_export(&self) -> String {
        let mut out = String::new();

        for c in &self.counters {
            let labels = format_prom_labels(c.labels());
            out.push_str(&format!(
                "# TYPE {} counter\n{}{} {}\n",
                c.name(),
                c.name(),
                labels,
                c.get()
            ));
        }
        for g in &self.gauges {
            let labels = format_prom_labels(g.labels());
            out.push_str(&format!(
                "# TYPE {} gauge\n{}{} {}\n",
                g.name(),
                g.name(),
                labels,
                g.get()
            ));
        }
        for h in &self.histograms {
            let labels = format_prom_labels(h.labels());
            out.push_str(&format!(
                "# TYPE {} summary\n{}_count{} {}\n{}_sum{} {}\n",
                h.name(),
                h.name(),
                labels,
                h.count(),
                h.name(),
                labels,
                h.sum()
            ));
        }

        out
    }
}

impl Default for MetricsRegistry {
    fn default() -> Self {
        Self::new()
    }
}

fn format_prom_labels(labels: &HashMap<String, String>) -> String {
    if labels.is_empty() {
        return String::new();
    }
    let inner: Vec<String> = labels
        .iter()
        .map(|(k, v)| format!("{}=\"{}\"", k, v))
        .collect();
    format!("{{{}}}", inner.join(","))
}

// ---------------------------------------------------------------------------
// Alerting
// ---------------------------------------------------------------------------

/// Severity levels for alerts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

impl std::fmt::Display for AlertSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Info => write!(f, "info"),
            Self::Warning => write!(f, "warning"),
            Self::Critical => write!(f, "critical"),
        }
    }
}

/// Comparison operator for threshold-based alert rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertCondition {
    GreaterThan,
    LessThan,
    EqualTo,
}

/// A rule that fires an alert when a metric crosses a threshold.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    pub id: Uuid,
    pub name: String,
    pub metric_name: String,
    pub condition: AlertCondition,
    pub threshold: f64,
    pub severity: AlertSeverity,
    pub enabled: bool,
    pub cooldown_seconds: u64,
}

/// A fired alert instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub id: Uuid,
    pub rule_id: Uuid,
    pub rule_name: String,
    pub severity: AlertSeverity,
    pub message: String,
    pub metric_value: f64,
    pub fired_at: DateTime<Utc>,
    pub resolved: bool,
}

/// Manages alert rules and fires alerts when metric thresholds are breached.
pub struct AlertManager {
    rules: Vec<AlertRule>,
    alerts: Vec<Alert>,
    last_fired: HashMap<Uuid, DateTime<Utc>>,
    max_alerts: usize,
}

impl AlertManager {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            alerts: Vec::new(),
            last_fired: HashMap::new(),
            max_alerts: 1000,
        }
    }

    pub fn add_rule(&mut self, rule: AlertRule) {
        self.rules.push(rule);
    }

    pub fn remove_rule(&mut self, rule_id: &Uuid) -> bool {
        let before = self.rules.len();
        self.rules.retain(|r| r.id != *rule_id);
        self.rules.len() < before
    }

    pub fn rules(&self) -> &[AlertRule] {
        &self.rules
    }

    pub fn alerts(&self) -> &[Alert] {
        &self.alerts
    }

    pub fn active_alerts(&self) -> Vec<&Alert> {
        self.alerts.iter().filter(|a| !a.resolved).collect()
    }

    pub fn resolve_alert(&mut self, alert_id: &Uuid) -> bool {
        if let Some(alert) = self.alerts.iter_mut().find(|a| a.id == *alert_id) {
            alert.resolved = true;
            true
        } else {
            false
        }
    }

    /// Evaluate all enabled rules against the provided metrics snapshot.
    /// Returns newly fired alerts.
    pub fn evaluate(&mut self, metrics: &[MetricSnapshot]) -> Vec<Alert> {
        let now = Utc::now();
        let mut new_alerts = Vec::new();

        for rule in &self.rules {
            if !rule.enabled {
                continue;
            }

            // Check cooldown
            if let Some(last) = self.last_fired.get(&rule.id) {
                let elapsed = (now - *last).num_seconds().max(0) as u64;
                if elapsed < rule.cooldown_seconds {
                    continue;
                }
            }

            // Find matching metric
            let Some(metric) = metrics.iter().find(|m| m.name == rule.metric_name) else {
                continue;
            };

            let triggered = match rule.condition {
                AlertCondition::GreaterThan => metric.value > rule.threshold,
                AlertCondition::LessThan => metric.value < rule.threshold,
                AlertCondition::EqualTo => (metric.value - rule.threshold).abs() < f64::EPSILON,
            };

            if triggered {
                let alert = Alert {
                    id: Uuid::new_v4(),
                    rule_id: rule.id,
                    rule_name: rule.name.clone(),
                    severity: rule.severity,
                    message: format!(
                        "{}: {} is {} (threshold: {})",
                        rule.name, metric.name, metric.value, rule.threshold
                    ),
                    metric_value: metric.value,
                    fired_at: now,
                    resolved: false,
                };
                new_alerts.push(alert.clone());
                self.last_fired.insert(rule.id, now);

                // Evict oldest if at capacity
                if self.alerts.len() >= self.max_alerts {
                    self.alerts.remove(0);
                }
                self.alerts.push(alert);
            }
        }

        new_alerts
    }
}

impl Default for AlertManager {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_status_display() {
        assert_eq!(HealthStatus::Healthy.to_string(), "healthy");
        assert_eq!(HealthStatus::Degraded.to_string(), "degraded");
        assert_eq!(HealthStatus::Unhealthy.to_string(), "unhealthy");
    }

    #[test]
    fn test_health_checker_all_healthy() {
        let mut checker = HealthChecker::new("2026.3.6");
        checker.add_check(|| ComponentHealth {
            name: "database".to_string(),
            status: HealthStatus::Healthy,
            message: None,
            latency_ms: Some(1.2),
            checked_at: Utc::now(),
        });
        checker.add_check(|| ComponentHealth {
            name: "broker".to_string(),
            status: HealthStatus::Healthy,
            message: None,
            latency_ms: Some(15.0),
            checked_at: Utc::now(),
        });

        let report = checker.check();
        assert_eq!(report.status, HealthStatus::Healthy);
        assert_eq!(report.components.len(), 2);
        assert_eq!(report.version, "2026.3.6");
    }

    #[test]
    fn test_health_checker_degraded() {
        let mut checker = HealthChecker::new("2026.3.6");
        checker.add_check(|| ComponentHealth {
            name: "database".to_string(),
            status: HealthStatus::Healthy,
            message: None,
            latency_ms: None,
            checked_at: Utc::now(),
        });
        checker.add_check(|| ComponentHealth {
            name: "cache".to_string(),
            status: HealthStatus::Degraded,
            message: Some("High latency".to_string()),
            latency_ms: Some(500.0),
            checked_at: Utc::now(),
        });

        let report = checker.check();
        assert_eq!(report.status, HealthStatus::Degraded);
    }

    #[test]
    fn test_health_checker_unhealthy() {
        let mut checker = HealthChecker::new("2026.3.6");
        checker.add_check(|| ComponentHealth {
            name: "database".to_string(),
            status: HealthStatus::Unhealthy,
            message: Some("Connection refused".to_string()),
            latency_ms: None,
            checked_at: Utc::now(),
        });

        let report = checker.check();
        assert_eq!(report.status, HealthStatus::Unhealthy);
    }

    #[test]
    fn test_counter() {
        let counter = Counter::new("requests_total").with_label("method", "GET");
        assert_eq!(counter.get(), 0);
        counter.increment();
        counter.increment();
        counter.add(3);
        assert_eq!(counter.get(), 5);
        assert_eq!(counter.name(), "requests_total");
        assert_eq!(counter.labels().get("method").unwrap(), "GET");
    }

    #[test]
    fn test_gauge() {
        let gauge = Gauge::new("cpu_usage");
        gauge.set(45.5);
        assert!((gauge.get() - 45.5).abs() < f64::EPSILON);
        gauge.set(72.3);
        assert!((gauge.get() - 72.3).abs() < f64::EPSILON);
    }

    #[test]
    fn test_histogram() {
        let mut hist = Histogram::new("request_duration_ms");
        hist.observe(10.0);
        hist.observe(20.0);
        hist.observe(30.0);
        assert_eq!(hist.count(), 3);
        assert!((hist.mean() - 20.0).abs() < f64::EPSILON);
        assert!((hist.min() - 10.0).abs() < f64::EPSILON);
        assert!((hist.max() - 30.0).abs() < f64::EPSILON);
        assert!((hist.sum() - 60.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_histogram_empty() {
        let hist = Histogram::new("empty");
        assert_eq!(hist.count(), 0);
        assert!((hist.mean() - 0.0).abs() < f64::EPSILON);
        assert!((hist.min() - 0.0).abs() < f64::EPSILON);
        assert!((hist.max() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_metrics_registry_snapshot() {
        let mut registry = MetricsRegistry::new();
        let ci = registry.add_counter(Counter::new("orders_total"));
        let gi = registry.add_gauge(Gauge::new("balance"));
        let hi = registry.add_histogram(Histogram::new("latency_ms"));

        registry.counter(ci).unwrap().add(42);
        registry.gauge(gi).unwrap().set(10000.0);
        registry.histogram_mut(hi).unwrap().observe(5.0);
        registry.histogram_mut(hi).unwrap().observe(15.0);

        let snap = registry.snapshot();
        // 1 counter + 1 gauge + 4 histogram facets = 6
        assert_eq!(snap.len(), 6);

        let orders = snap.iter().find(|s| s.name == "orders_total").unwrap();
        assert!((orders.value - 42.0).abs() < f64::EPSILON);

        let balance = snap.iter().find(|s| s.name == "balance").unwrap();
        assert!((balance.value - 10000.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_prometheus_export() {
        let mut registry = MetricsRegistry::new();
        registry.add_counter(Counter::new("http_requests"));
        registry.counter(0).unwrap().add(100);

        let prom = registry.prometheus_export();
        assert!(prom.contains("# TYPE http_requests counter"));
        assert!(prom.contains("http_requests 100"));
    }

    #[test]
    fn test_alert_rule_fires() {
        let mut mgr = AlertManager::new();
        let rule_id = Uuid::new_v4();
        mgr.add_rule(AlertRule {
            id: rule_id,
            name: "High CPU".to_string(),
            metric_name: "cpu_usage".to_string(),
            condition: AlertCondition::GreaterThan,
            threshold: 90.0,
            severity: AlertSeverity::Critical,
            enabled: true,
            cooldown_seconds: 0,
        });

        let metrics = vec![MetricSnapshot {
            name: "cpu_usage".to_string(),
            kind: "gauge".to_string(),
            value: 95.0,
            labels: HashMap::new(),
            timestamp: Utc::now(),
        }];

        let fired = mgr.evaluate(&metrics);
        assert_eq!(fired.len(), 1);
        assert_eq!(fired[0].severity, AlertSeverity::Critical);
        assert!(!fired[0].resolved);
        assert_eq!(mgr.active_alerts().len(), 1);
    }

    #[test]
    fn test_alert_rule_does_not_fire_below_threshold() {
        let mut mgr = AlertManager::new();
        mgr.add_rule(AlertRule {
            id: Uuid::new_v4(),
            name: "High CPU".to_string(),
            metric_name: "cpu_usage".to_string(),
            condition: AlertCondition::GreaterThan,
            threshold: 90.0,
            severity: AlertSeverity::Warning,
            enabled: true,
            cooldown_seconds: 0,
        });

        let metrics = vec![MetricSnapshot {
            name: "cpu_usage".to_string(),
            kind: "gauge".to_string(),
            value: 50.0,
            labels: HashMap::new(),
            timestamp: Utc::now(),
        }];

        let fired = mgr.evaluate(&metrics);
        assert!(fired.is_empty());
    }

    #[test]
    fn test_alert_resolve() {
        let mut mgr = AlertManager::new();
        mgr.add_rule(AlertRule {
            id: Uuid::new_v4(),
            name: "Low balance".to_string(),
            metric_name: "balance".to_string(),
            condition: AlertCondition::LessThan,
            threshold: 1000.0,
            severity: AlertSeverity::Warning,
            enabled: true,
            cooldown_seconds: 0,
        });

        let metrics = vec![MetricSnapshot {
            name: "balance".to_string(),
            kind: "gauge".to_string(),
            value: 500.0,
            labels: HashMap::new(),
            timestamp: Utc::now(),
        }];

        let fired = mgr.evaluate(&metrics);
        assert_eq!(fired.len(), 1);
        let alert_id = fired[0].id;

        assert!(mgr.resolve_alert(&alert_id));
        assert!(mgr.active_alerts().is_empty());
    }

    #[test]
    fn test_disabled_rule_does_not_fire() {
        let mut mgr = AlertManager::new();
        mgr.add_rule(AlertRule {
            id: Uuid::new_v4(),
            name: "Disabled".to_string(),
            metric_name: "cpu_usage".to_string(),
            condition: AlertCondition::GreaterThan,
            threshold: 0.0,
            severity: AlertSeverity::Info,
            enabled: false,
            cooldown_seconds: 0,
        });

        let metrics = vec![MetricSnapshot {
            name: "cpu_usage".to_string(),
            kind: "gauge".to_string(),
            value: 100.0,
            labels: HashMap::new(),
            timestamp: Utc::now(),
        }];

        let fired = mgr.evaluate(&metrics);
        assert!(fired.is_empty());
    }

    #[test]
    fn test_remove_rule() {
        let mut mgr = AlertManager::new();
        let id = Uuid::new_v4();
        mgr.add_rule(AlertRule {
            id,
            name: "test".to_string(),
            metric_name: "x".to_string(),
            condition: AlertCondition::GreaterThan,
            threshold: 0.0,
            severity: AlertSeverity::Info,
            enabled: true,
            cooldown_seconds: 0,
        });
        assert_eq!(mgr.rules().len(), 1);
        assert!(mgr.remove_rule(&id));
        assert!(mgr.rules().is_empty());
    }

    #[test]
    fn test_alert_severity_display() {
        assert_eq!(AlertSeverity::Info.to_string(), "info");
        assert_eq!(AlertSeverity::Warning.to_string(), "warning");
        assert_eq!(AlertSeverity::Critical.to_string(), "critical");
    }
}
