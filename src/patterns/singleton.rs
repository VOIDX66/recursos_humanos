use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Singleton<T> {
    inner: Arc<Mutex<T>>,
}

impl<T> Singleton<T> {
    pub fn new(value: T) -> Self {
        Singleton {
            inner: Arc::new(Mutex::new(value)),
        }
    }

    pub async fn get(&self) -> T {
        let lock = self.inner.lock().await;
        lock.clone()
    }
}
