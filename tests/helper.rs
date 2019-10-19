use std::future::Future;

/// Used for running async tests.
pub fn block_on<T>(f: impl Future<Output = T>) {
    let mut runtime = tokio::runtime::current_thread::Runtime::new().unwrap();
    runtime.block_on(f);
    runtime.run().unwrap();
}
