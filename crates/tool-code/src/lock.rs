use std::sync::{Arc, Mutex, MutexGuard};

use eyre::{eyre, Result};

pub trait Get<T> {
    fn get(&self) -> T;
}

#[derive(Clone)]
pub struct Pointer<T> {
    value: Arc<Mutex<T>>,
}
impl<T> Pointer<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: Arc::new(Mutex::new(value)),
        }
    }
    pub fn lock(&self) -> MutexGuard<T> {
        self.value.lock().unwrap()
    }
    pub fn set(&self, value: T) {
        *self.lock() = value;
    }
}
impl<T: Clone> Get<T> for Pointer<T> {
    fn get(&self) -> T {
        self.lock().clone()
    }
}

#[derive(Clone)]
pub struct Container<T> {
    value: Pointer<Vec<T>>,
}
impl<T> Container<T> {
    pub fn new() -> Self {
        Self {
            value: Pointer::new(Vec::new()),
        }
    }
    pub fn push(&self, value: T) {
        self.value.lock().push(value);
    }
    pub fn remove(&self, index: usize) {
        self.value.lock().remove(index);
    }
}

#[derive(Clone)]
pub struct PointerPreNew<T> {
    value: Pointer<Option<T>>,
}
impl<T: Clone> PointerPreNew<T> {
    pub fn new() -> Self {
        Self {
            value: Pointer::new(None),
        }
    }
    pub fn set(&self, value: T) {
        *self.value.lock() = Some(value);
    }
    pub fn get(&self) -> Result<T> {
        self.value.lock().clone().ok_or(eyre!("没有设置值"))
    }
}
