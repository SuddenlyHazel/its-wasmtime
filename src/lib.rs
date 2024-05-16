use wasmtime::{component::Linker, Config, Engine, Store};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

pub struct RuntimeView<T: NestedView> {
    pub table: ResourceTable,
    pub ctx: WasiCtx,
    pub nested_view: T,
}

impl<T> RuntimeView<T>
where
    T: NestedView,
{
    fn new(nested_view: T) -> Self {
        let table = ResourceTable::new();
        let ctx = WasiCtxBuilder::new().inherit_stdio().build();

        Self {
            table,
            ctx,
            nested_view,
        }
    }
}

impl<T> WasiView for RuntimeView<T>
where
    T: Send + NestedView,
{
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

pub trait NestedView: Send + Sized {
    fn add_all_to_linker(&mut self, linker: &mut Linker<RuntimeView<Self>>) -> anyhow::Result<()>;
}

pub struct Runtime<T: NestedView> {
    pub engine: Engine,
    pub linker: Linker<RuntimeView<T>>,
    pub store: Store<RuntimeView<T>>,
}

pub fn runtime<T>(with_wasi: bool, mut nested_view: T) -> anyhow::Result<Runtime<T>>
where
    T: NestedView,
{
    let config = {
        let mut config = Config::new();
        config.wasm_component_model(true);
        config.async_support(true);
        config
    };

    let engine = Engine::new(&config)?;

    let mut linker = Linker::new(&engine);

    if with_wasi {
        wasmtime_wasi::add_to_linker_async(&mut linker)?;
    }

    nested_view.add_all_to_linker(&mut linker)?;

    let runtime_view = RuntimeView::new(nested_view);
    let store = Store::new(&engine, runtime_view);

    Ok(Runtime {
        engine,
        linker,
        store,
    })
}

#[cfg(test)]
mod simple_component_test {
    use super::*;
    use wasmtime::{component::Component, AsContextMut};
    use wasmtime_wasi::async_trait;

    wasmtime::component::bindgen!({
        path: "./wasm_test_components/simple_component/wit/world.wit",
        world: "example",
        async: true,
    });

    struct SimpleComponentView {
        message: String,
    }

    #[async_trait]
    impl host::Host for SimpleComponentView {
        async fn get_data(&mut self) -> wasmtime::Result<String> {
            Ok(self.message.clone())
        }
    }

    impl NestedView for SimpleComponentView {
        fn add_all_to_linker(
            &mut self,
            linker: &mut Linker<RuntimeView<Self>>,
        ) -> anyhow::Result<()> {
            Ok(host::add_to_linker(linker, |v| &mut v.nested_view)?)
        }
    }

    #[tokio::test]
    async fn it_invokes_simple_component() {
        let nested_view = SimpleComponentView {
            message: "Hello, World!".into(),
        };

        let mut runtime = runtime(true, nested_view).expect("Failed to build runtime");

        let component = Component::from_file(&runtime.engine, "./wasm_test_components/simple_component/target/wasm32-wasi/debug/simple_component.wasm")
        .expect("Failed to load component from disk. Did you compile it using `cargo component build`?");

        let (instance, _) =
            Example::instantiate_async(&mut runtime.store, &component, &runtime.linker)
                .await
                .expect("failed to instantiate component");

        let store = runtime.store.as_context_mut();

        let result = instance
            .call_hello_world(store)
            .await
            .expect("failed to invoke demo function");

        assert_eq!(result, "Hello, World! 0");

        let store = runtime.store.as_context_mut();

        let result = instance
            .call_hello_world(store)
            .await
            .expect("failed to invoke demo function");

        assert_eq!(result, "Hello, World! 1");
    }
}
