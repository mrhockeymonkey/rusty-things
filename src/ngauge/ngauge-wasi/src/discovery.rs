
pub struct MetricPlugin {
    pub path: String
}

// TODO currently just simulates finding wasm files, but could be made to read from disk as new ones are added
pub fn get_metric_plugins() -> Vec<MetricPlugin> {
    vec![
        MetricPlugin{
            path: "../target/wasm32-wasi/release/rs_gauge.wasm".to_string()
        },
        MetricPlugin{
            path: "../examples/py-gauge/py-gauge.wasm".to_string()
        }
    ]
}
