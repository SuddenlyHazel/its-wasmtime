#[allow(warnings)]
mod bindings;

use bindings::{component::simple_resource, Guest};

struct Component;

impl Guest for Component {
    /// Say hello!
    fn test() -> String {
        let resource = simple_resource::some_resource::FooResource::new();
        let value = resource.foo();
        format!("Hello, World! {value}")
    }
}

bindings::export!(Component with_types_in bindings);
