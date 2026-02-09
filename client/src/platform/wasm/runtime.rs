static RUNTIME: std::sync::OnceLock<tokio::runtime::Runtime> = std::sync::OnceLock::new();

pub fn init() {
  let rt = tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build()
    .unwrap();

  RUNTIME.set(rt).expect("runtime already initialized");
}

pub fn get() -> &'static tokio::runtime::Runtime {
  RUNTIME.get().expect("runtime not initialized")
}

pub fn _spawn_async<F>(fut: F)
where
  F: std::future::Future<Output = ()> + 'static,
{
  wasm_bindgen_futures::spawn_local(fut);
}

