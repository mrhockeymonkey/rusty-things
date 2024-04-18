#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn hello_world() -> String {
        "Hello, World!".to_string()
    }

    fn measure(assets: Vec<String>) -> u32 {
        assets
            .iter()
            .inspect(|s| {println!("got {}", s)})
            .count() as u32
    }
}

bindings::export!(Component with_types_in bindings);
