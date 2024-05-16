#[allow(warnings)]
mod bindings;

use std::sync::atomic::AtomicI32;

use bindings::Guest;

use crate::bindings::host;

static COUNTER: AtomicI32 = AtomicI32::new(0);

struct Component;

impl Guest for Component {
    /// Say hello!
    fn hello_world() -> String {
        format!("{} {}", host::get_data(), COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    }
}

bindings::export!(Component with_types_in bindings);
