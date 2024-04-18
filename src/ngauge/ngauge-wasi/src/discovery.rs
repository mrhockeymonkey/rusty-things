
pub mod discovery {

    pub struct MetricPlugin {
        pub path: String
    }

    // TODO make this dynamic somehow, possible read all wasm files from a single directory
    pub fn get_metric_plugins() -> Vec<MetricPlugin> {
        vec![
            MetricPlugin{
                path: "/home/scott/code/rusty-things/src/ngauge/target/wasm32-wasi/debug/add.wasm".to_string()
            },
            MetricPlugin{
                path: "/home/scott/code/rusty-things/src/ngauge/examples/py-gauge/py-gauge.wasm".to_string()
            }
        ]
    }
}