use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CircuitBreakerConfigRoot {
    pub circuit_breaker: Vec<CircuitBreakerConfig>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CircuitBreakerConfig {
    pub name: String,
    pub circuit_breaker_match: CircuitBreakerMatch,
    pub breaker_config: Vec<BreakerConfig>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CircuitBreakerMatch {
    pub host: String,
    pub port: u16,
    pub method: String,
    pub path: String,
}

impl CircuitBreakerMatch {
    fn resource_name(&self) -> String {
        return format!("{}{}{}{}", self.method, self.host, self.port, self.path);
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BreakerConfig {
    pub window_size: u8,
    pub break_duration: u16,
    pub slow_request_rt: f32,
    pub max_slow_requests: u32,
    pub error_percent: f32,
    pub min_request_amount: u32,
    pub custom_response: CustomResponse,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CustomResponse {
    pub status_code: u16,
    pub header_to_add: HashMap<String, String>,
    pub body: String,
}