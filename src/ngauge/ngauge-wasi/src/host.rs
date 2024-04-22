use wasmtime::{Config, Engine, Store};
use anyhow::{Context, Result};
use wasmtime::component::{Component, ResourceTable, Linker};
use wasmtime_wasi::preview2::{command, WasiCtx, WasiCtxBuilder, WasiView};

use crate::discovery::MetricPlugin;
use crate::Example;

pub struct MetricHost {
    pub engine: Engine,
    components: Vec<Component>,
    linker: Linker<ServerWasiView>
}

impl MetricHost {

    pub fn new() -> Result<Self> {
        let mut config = Config::default();
        config.wasm_component_model(true);
        config.async_support(true);

        let engine = Engine::new(&config)?;
        let mut linker = Linker::new(&engine);

        // Add the command world (aka WASI CLI) to the linker
        command::add_to_linker(&mut linker).context("Failed to link command world")?;

        Ok(MetricHost {
            engine,
            components: vec![],
            linker
        })
    }

    pub fn load(&mut self, plugin: MetricPlugin) -> Result<()> {
        println!("MetricHost - Adding wasi component: {}", plugin.path);
        let component = Component::from_file(&self.engine, plugin.path)
            .context("Component file not found")?;

        self.components.push(component);

        Ok(())
    }

    pub async fn collect_metrics(&self) -> Result<()> {
        let wasi_view = ServerWasiView::new();
        let mut store = Store::new(&self.engine, wasi_view);

        let mut i = 1;
        for component in &self.components {
            let (instance, _) = Example::instantiate_async(&mut store, &component, &self.linker)
                .await
                .context("Failed to instantiate the example world")?;

            // TODO dummy data
            let assets = ["foo".to_string(), "bar".to_string(), "baz".to_string()];
            let count = instance
                .call_measure(&mut store, &assets)
                .await
                .context("Failed to call add function")?;

            println!("MetricHost - Component {} Measure returned {}", i, count);
            i += 1;

        }

        Ok(())
    }
}


struct ServerWasiView {
    table: ResourceTable,
    ctx: WasiCtx,
}

impl ServerWasiView {
    fn new() -> Self {
        let table = ResourceTable::new();
        let ctx = WasiCtxBuilder::new().inherit_stdio().build();

        Self { table, ctx }
    }
}

impl WasiView for ServerWasiView {
    fn table(&self) -> &ResourceTable {
        &self.table
    }

    fn table_mut(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&self) -> &WasiCtx {
        &self.ctx
    }

    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}
