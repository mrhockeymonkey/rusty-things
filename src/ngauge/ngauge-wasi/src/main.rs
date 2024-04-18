mod discovery;
mod host;

use std::thread;
use tokio::time::{sleep, Duration};
use anyhow::Context;
use async_std::task::block_on;
use wasi_common::pipe::ReadPipe;
use wasmtime::{Config, Engine, Store};
use wasmtime::component::{Component, Linker};
use wasmtime_wasi::preview2::{command, WasiCtx, WasiCtxBuilder, WasiView, ResourceTable};
use crate::discovery::discovery::get_metric_plugins;
use crate::host::MetricHost;

wasmtime::component::bindgen!({
    path: "/home/scott/code/rusty-things/src/ngauge/add/wit/world.wit",
    world: "example",
    async: true
});

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

#[async_std::main]
async fn main() -> wasmtime::Result<()> {

    // TODO
    let host = MetricHost::new()?;

    // loop {
    //     let metrics = get_metric_plugins();
    //     metrics.iter().for_each(|m| println!("metric: {}", (m.path)));
    // }

    // let mut config = Config::default();
    // config.wasm_component_model(true);
    // config.async_support(true);
    //
    // let engine = Engine::new(&config)?;
    let mut linker = Linker::new(&host.engine);

    // Add the command world (aka WASI CLI) to the linker
    command::add_to_linker(&mut linker).context("Failed to link command world")?;

    // TODO this is slow, can we prebundle all components into one?
    println!("Compiling rs_gauge...");
    let rs_gauge = Component::from_file(&host.engine, "/home/scott/code/rusty-things/src/ngauge/target/wasm32-wasi/release/rs_gauge.wasm").context("Component file not found")?;

    println!("Compiling py_gauge...");
    let py_gauge = Component::from_file(&host.engine, "/home/scott/code/rusty-things/src/ngauge/examples/py-gauge/py-gauge.wasm").context("Component file not found")?;

    let pause = Duration::from_secs(3);
    loop {
        println!("hi");
        // let sl = sleep(Duration::from_secs(5));
        // block_on(sl);

        let wasi_view = ServerWasiView::new();
        let mut store = Store::new(&host.engine, wasi_view);

        let (instance, _) = Example::instantiate_async(&mut store, &rs_gauge, &linker)
            .await
            .context("Failed to instantiate the example world")?;

        let assets = ["foo".to_string(), "bar".to_string(), "baz".to_string()];
        let count = instance
            .call_measure(&mut store, &assets)
            .await
            .context("Failed to call add function")?;

        println!("rs-gauge measure returned was {}", count);

        let (py_instance, _) = Example::instantiate_async(&mut store, &py_gauge, &linker)
            .await
            .context("Failed to instantiate the example world")?;

        let assets1 = ["ping".to_string(), "pong".to_string(), "sing".to_string(), "song".to_string()];
        let count = py_instance
            .call_measure(&mut store, &assets1)
            //.call_measure(&mut store, &assets1)
            .await
            .context("Failed to call add function")?;

        println!("py-gauge measure returned was {}", count);
    }

    // add_to_linker(&mut linker, |s| s)?;
    //
    // let stdin = ReadPipe::from("THE INPUT");



    //
    // let wat = r#"
    //     (module
    //       ;; Declare the function type
    //       (func $add (param i32 i32) (result i32)
    //         ;; Function body
    //         (local.get 0)   ;; Get the first parameter
    //         (local.get 1)   ;; Get the second parameter
    //         (i32.add)       ;; Add the two parameters
    //       )
    //
    //       ;; Export the function so it can be used outside of the module
    //       (export "_start" (func $add))
    //     )
    // "#;
    // let wat_module = Module::new(&engine, wat)?;
    //
    // // All wasm objects operate within the context of a "store". Each
    // // `Store` has a type parameter to store host-specific data, which in
    // // this case we're using `4` for.
    // let mut store = Store::new(&engine, wasi);
    //
    // // Instantiate our module with the imports we've created, and run it.
    // let rs_module = Module::from_file(&engine, "/home/scott/code/rusty-things/src/ngauge/target/wasm32-wasi/debug/rs-metric.wasm")?;
    // linker.module(&mut store, "", &rs_module)?;
    // // linker.module(&mut store, "", &module)?;
    // let result = linker
    //     .get_default(&mut store, "")?
    //     .typed::<(), i32>(&store)?
    //     .call(&mut store, ())?;
    // // let result = linker
    // //     .get_default(&mut store, "")?
    // //     .typed::<(i32, i32), i32>(&store)?
    // //     .call(&mut store, (2, 2))?;
    // println!("got {} back", result);

    // let host_func = Func::wrap(&mut store, |caller: Caller<'_, u32>, param: i32| {
    //     println!("Got {} from WebAssembly", param);
    //     println!("my host state is: {}", caller.data());
    // });
    //
    // // Instantiation of a module requires specifying its imports and then
    // // afterwards we can fetch exports by name, as well as asserting the
    // // type signature of the function with `get_typed_func`.
    // let instance = Instance::new(&mut store, &module, &[host_func.into()])?;
    // let hello = instance.get_typed_func::<(), ()>(&mut store, "hello")?;
    //
    // // And finally we can call the wasm!
    // hello.call(&mut store, ())?;

    Ok(())
}



/*
You can execute this example with:
    cmake examples/
    cargo run --example wasi
*/

//
//
// fn main() -> Result<()> {
//     // Define the WASI functions globally on the `Config`.
//     let engine = Engine::default();
//
//
//     // Create a WASI context and put it in a Store; all instances in the store
//     // share this context. `WasiCtxBuilder` provides a number of ways to
//     // configure what the target program will have access to.
//     let wasi = WasiCtxBuilder::new()
//         .inherit_stdio()
//         .inherit_args()?
//         .build();
//     let mut store = Store::new(&engine, wasi);
//
//     // Instantiate our module with the imports we've created, and run it.
//     let module = Module::from_file(&engine, "target/wasm32-wasi/debug/wasi.wasm")?;
//     linker.module(&mut store, "", &module)?;
//     linker
//         .get_default(&mut store, "")?
//         .typed::<(), ()>(&store)?
//         .call(&mut store, ())?;
//
//     Ok(())
// }