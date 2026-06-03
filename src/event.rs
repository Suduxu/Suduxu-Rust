use std::sync::{Arc, Mutex};

type Subscriber<T> = Box<dyn Fn(T) + Send + Sync>;

#[derive(Clone)]
pub struct EventBus<T>
where
    T: Clone,
{
    subscribers: Arc<Mutex<Vec<Subscriber<T>>>>,
}

impl<T: Clone> EventBus<T> {
    pub fn new() -> Self {
        EventBus {
            subscribers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn subscribe<F>(&self, callback: F)
    where
        F: Fn(T) + Send + Sync + 'static,
    {
        let mut subs = self.subscribers.lock().unwrap();
        subs.push(Box::new(callback));
    }

    pub fn publish(&self, event: T) {
        if let Ok(subs) = self.subscribers.lock() {
            for cb in subs.iter() {
                cb(event.clone());
            }
        }
    }

    pub fn clear_subscribers(&self) {
        if let Ok(mut subs) = self.subscribers.lock() {
            subs.clear();
        }
    }

    pub fn subscriber_count(&self) -> usize {
        if let Ok(subs) = self.subscribers.lock() {
            subs.len()
        } else {
            0
        }
    }
}

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct EventObject {
    kind: u16,
    r#type: String,
    value: Value
}

impl EventObject {
    pub fn new(kind: u16, r#type: String, value: Value) -> Self {
        EventObject { kind, r#type, value }
    }

    pub fn valueless_new(kind: u16, r#type: String) -> Self {
        EventObject { kind, r#type, value: Value::Null}
    }

    pub fn kind(&self) -> u16 {
        self.kind
    }

    pub fn r#type(&self) -> &str {
        &self.r#type
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
    pub fn into_value(self) -> Value {
        self.value
    }
}