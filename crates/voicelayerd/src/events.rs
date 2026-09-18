//! Typed daemon event bus backed by a tokio broadcast channel.

use tokio::sync::broadcast;
use voicelayer_core::{DaemonEvent, EventEnvelope};

const EVENT_CHANNEL_CAPACITY: usize = 128;

/// Fan-out bus for daemon events, backed by a tokio broadcast channel
/// (capacity 128). Clones share the underlying sender.
#[derive(Clone)]
pub struct EventBus {
    sender: broadcast::Sender<EventEnvelope>,
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus {
    /// Create a bus with no subscribers.
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(EVENT_CHANNEL_CAPACITY);
        Self { sender }
    }

    /// Publish an event wrapped in a fresh [`EventEnvelope`]; sending
    /// with no subscribers present is a no-op.
    pub fn emit(&self, event: DaemonEvent) {
        // No subscribers is a normal state; drop the send result.
        let _ = self.sender.send(EventEnvelope::new(event));
    }

    /// Register a new broadcast receiver; a receiver that lags behind
    /// the channel capacity sees `RecvError::Lagged`, which the SSE
    /// handler translates into an `events_lost` event.
    pub fn subscribe(&self) -> broadcast::Receiver<EventEnvelope> {
        self.sender.subscribe()
    }
}
