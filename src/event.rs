use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::{Arc, Mutex};

type Subscriber<T> = Arc<dyn Fn(T) + Send + Sync>;

/// A simple event bus that allows subscribers to listen for events of type `T`.
#[derive(Clone)]
pub struct EventBus<T>
where
    T: Clone,
{
    subscribers: Arc<Mutex<Vec<Subscriber<T>>>>,
}

impl<T: Clone> EventBus<T> {
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Subscribes a callback function to the event bus.
    /// The callback will be called whenever an event of type `T` is published.
    ///
    /// # Arguments
    /// * `callback` - A function that takes an event of type `T` and returns nothing.
    ///
    /// # Usage
    /// `event_bus.subscribe(|event| { /* handle event */ });`
    pub fn subscribe<F>(&self, callback: F)
    where
        F: Fn(T) + Send + Sync + 'static,
    {
        let mut subs = self.subscribers.lock().unwrap();
        subs.push(Arc::new(callback));
    }

    /// Publishes an event to all subscribers.
    ///
    /// # Arguments
    /// * `event` - The event of type `T` to be published.
    ///
    /// # Usage
    /// `event_bus.publish(event);`
    pub fn publish(&self, event: T) {
        let callbacks = {
            let subs = self.subscribers.lock().unwrap();
            subs.clone()
        };

        for cb in callbacks {
            cb(event.clone());
        }
    }

    /// Clears all subscribers from the event bus.
    pub fn clear_subscribers(&self) {
        self.subscribers.lock().unwrap().clear();
    }

    /// Returns the number of subscribers currently registered to the event bus.
    pub fn subscriber_count(&self) -> usize {
        self.subscribers.lock().unwrap().len()
    }
}

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