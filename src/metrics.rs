use crate::config::MetricsConfig;
use crate::error::Result;
use metrics_exporter_prometheus::PrometheusBuilder;
use std::net::SocketAddr;

pub fn init(config: &MetricsConfig) -> Result<()> {
    if !config.enabled {
        return Ok(());
    }

    let addr = format!("{}:{}", config.host, config.port)
        .parse::<SocketAddr>()
        .map_err(|e| crate::error::SafeQuantaError::Metrics(e.to_string()))?;

    PrometheusBuilder::new()
        .with_http_listener(addr)
        .install()
        .map_err(|e| crate::error::SafeQuantaError::Metrics(e.to_string()))?;

    Ok(())
}

// Handshake metrics
pub fn record_handshake_time(duration_ms: u64) {
    metrics::histogram!("handshake_duration_ms", duration_ms as f64);
}

pub fn record_handshake_error() {
    metrics::counter!("handshake_errors_total").increment(1);
}

// TLS alert metrics
pub fn record_tls_alert(alert_type: &str) {
    metrics::counter!("tls_alerts_total", "type" => alert_type.to_string()).increment(1);
}

// CPU metrics
pub fn record_cpu_cycles(cycles: u64) {
    metrics::gauge!("cpu_cycles_total", cycles as f64);
}

// Connection metrics
pub fn record_active_connections(count: u64) {
    metrics::gauge!("active_connections", count as f64);
}

pub fn record_connection_error() {
    metrics::counter!("connection_errors_total").increment(1);
}

// Proxy metrics
pub fn record_proxy_request_duration(duration_ms: u64) {
    metrics::histogram!("proxy_request_duration_ms", duration_ms as f64);
}

pub fn record_proxy_bytes_sent(bytes: u64) {
    metrics::counter!("proxy_bytes_sent_total").increment(bytes);
}

pub fn record_proxy_bytes_received(bytes: u64) {
    metrics::counter!("proxy_bytes_received_total").increment(bytes);
} 