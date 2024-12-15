use std::sync::{Arc, Mutex, MutexGuard};

pub struct MLock<T> {
    value: Arc<Mutex<T>>,
}
impl<T> MLock<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: Arc::new(Mutex::new(value)),
        }
    }
    pub fn lock(&self) -> MutexGuard<T> {
        self.value.lock().unwrap()
    }
}
impl<T> Clone for MLock<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}
