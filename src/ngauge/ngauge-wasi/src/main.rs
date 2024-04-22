mod discovery;
mod host;

use std::time::Duration;
use async_std::task::sleep;

// generate bindings based on the exported wit
wasmtime::component::bindgen!({
    path: "./wit/world.wit",
    world: "example",
    async: true
});

#[async_std::main]
async fn main() -> wasmtime::Result<()> {

    // metric host deals with compiling and running wasm components
    let mut host = host::MetricHost::new()?;

    // load wasm plugins from disk
    discovery::get_metric_plugins()
        .into_iter()
        .for_each(|plugin| host.load(plugin).unwrap());


    let pause = Duration::from_secs(5);
    let mut i = 1;
    loop {

        println!("Calling collection - {}", i);
        host.collect_metrics().await?;
        i += 1;

        println!("Sleeping {:?}", pause);
        sleep(pause).await;
    }
}