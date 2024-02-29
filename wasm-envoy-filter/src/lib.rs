use std::env;
use std::sync::{Arc, RwLock};

use lazy_static::lazy_static;
use log::warn;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

mod config;

lazy_static! {
    pub static ref WR_COUNTER: Arc<RwLock<u32>> = Arc::new(RwLock::new(0));
}

pub fn get_wr_counter() -> Arc<RwLock<u32>> {
    WR_COUNTER.clone()
}

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Info);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(CircuitBreakerRoot {})
    });
}}


struct CircuitBreakerRoot {}

impl Context for CircuitBreakerRoot {}

impl RootContext for CircuitBreakerRoot {
    fn on_vm_start(&mut self, _vm_configuration_size: usize) -> bool {
        for (n, v) in env::vars() {
            warn!("{}: {}", n,v);
        }
        true
    }

    fn on_configure(&mut self, _: usize) -> bool {
        if let Some(config_bytes) = self.get_plugin_configuration() {
            warn!("plugin_configuration: {:?}", String::from_utf8(config_bytes).unwrap())
        }
        true
    }

    fn create_http_context(&self, context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(CircuitBreakerHttp {
            context_id,
        }))
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}

struct CircuitBreakerHttp {
    context_id: u32,
}

impl CircuitBreakerHttp {
    fn context_id(&self) -> u32 {
        self.context_id
    }
}

impl Context for CircuitBreakerHttp {}

impl HttpContext for CircuitBreakerHttp {
    fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {
        let binding = get_wr_counter();
        let mut count = binding.write().unwrap();
        *count += 1;
        Action::Continue
    }

    fn on_log(&mut self) {
        warn!("on_log {}",self.context_id());
        let binding = get_wr_counter();
        let count = binding.read().unwrap();
        warn!("global_counter={}", count)
    }
}