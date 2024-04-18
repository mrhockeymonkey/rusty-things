use wasmtime::{Config, Engine};
use anyhow::{Result};

pub struct MetricHost {
    pub engine: Engine
}

impl MetricHost {

    pub fn new() -> Result<Self> {
        let mut config = Config::default();
        config.wasm_component_model(true);
        config.async_support(true);

        let engine = Engine::new(&config)?;

        Ok(MetricHost {
            engine
        })
    }
}
