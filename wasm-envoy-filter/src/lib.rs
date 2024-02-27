use std::env;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use lazy_static::lazy_static;
use log::warn;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

mod config;

lazy_static! {
    pub static ref CONCURRENT_COUNTER: Arc<ConcurrentCounter> = {
        Arc::new(ConcurrentCounter::new())
    };
}

pub struct ConcurrentCounter {
    counter: AtomicUsize,
}

impl ConcurrentCounter {
    // 创建一个新的计数器
    fn new() -> Self {
        ConcurrentCounter {
            counter: AtomicUsize::new(0),
        }
    }

    // 增加计数器的值
    fn increment(&self) {
        self.counter.fetch_add(1, Ordering::Relaxed);
    }

    // 获取当前计数器的值
    fn get(&self) -> usize {
        self.counter.load(Ordering::Relaxed)
    }
}

pub fn global_counter() -> Arc<ConcurrentCounter> {
    CONCURRENT_COUNTER.clone()
}

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Info);
    warn!("start wasm");
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(CircuitBreakerRoot {})
    });
}}


struct CircuitBreakerRoot {}

impl Context for CircuitBreakerRoot {}

impl RootContext for CircuitBreakerRoot {
    fn on_vm_start(&mut self, _vm_configuration_size: usize) -> bool {
        warn!("on vm start");
        for (n, v) in env::vars() {
            warn!("{}: {}", n,v);
        }
        true
    }

    fn on_configure(&mut self, _: usize) -> bool {
        warn!("on configure");
        if let Some(config_bytes) = self.get_plugin_configuration() {
            warn!("plugin_configuration: {:?}", String::from_utf8(config_bytes).unwrap())
        }
        true
    }

    fn on_tick(&mut self) {
        warn!("on tick");
    }

    fn create_http_context(&self, context_id: u32) -> Option<Box<dyn HttpContext>> {
        warn!("on create_http_context");
        Some(Box::new(CircuitBreakerHttp {
            context_id,
        }))
    }

    fn get_type(&self) -> Option<ContextType> {
        warn!("on get_type");
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
        warn!("on_http_request_headers {}",self.context_id());
        for headers in self.get_http_request_headers() {
            warn!("header key:{},value:{}",headers.0,headers.1);
        }
        global_counter().increment();
        Action::Continue
    }

    fn on_http_request_body(&mut self, _body_size: usize, _end_of_stream: bool) -> Action {
        if !_end_of_stream {
            return Action::Pause;
        }
        warn!("on_http_request_body {}",self.context_id());
        if _end_of_stream {
            if let Some(body_bytes) = self.get_http_request_body(0, _body_size) {
                warn!("request body:{}", String::from_utf8_lossy(&body_bytes));
            }
        }
        Action::Continue
    }

    fn on_http_response_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        warn!("on_http_response_headers {}",self.context_id());
        for (name, value) in &self.get_http_response_headers() {
            warn!("{}={}", name, value);
        }
        Action::Continue
    }

    fn on_http_response_body(&mut self, _body_size: usize, _end_of_stream: bool) -> Action {
        if !_end_of_stream {
            return Action::Pause;
        }
        warn!("on_http_response_body {}",self.context_id());
        if _end_of_stream {
            if let Some(body_bytes) = self.get_http_response_body(0, _body_size) {
                let body_str = String::from_utf8(body_bytes).unwrap();
                warn!("response body:{}", body_str);
            }
        }
        Action::Continue
    }

    fn on_log(&mut self) {
        warn!("on_log {}",self.context_id());
        warn!("global_counter={}", global_counter().get())
    }
}