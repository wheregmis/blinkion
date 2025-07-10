use futures_channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender};
use once_cell::sync::Lazy;
use std::sync::{Arc, RwLock};

/// The shared application state for blink settings.
#[derive(Debug, Clone)]
pub struct BlinkSettings {
    pub blink_interval: u64,
    pub blink_duration: u64,
}

impl Default for BlinkSettings {
    fn default() -> Self {
        Self {
            blink_interval: 30,
            blink_duration: 3,
        }
    }
}

/// A globally accessible, thread-safe shared state for blink settings.
pub static SHARED_BLINK_SETTINGS: Lazy<Arc<RwLock<BlinkSettings>>> =
    Lazy::new(|| Arc::new(RwLock::new(BlinkSettings::default())));

/// Channel for broadcasting settings changes
pub static SETTINGS_CHANNEL: Lazy<(
    UnboundedSender<BlinkSettings>,
    Arc<RwLock<UnboundedReceiver<BlinkSettings>>>,
)> = Lazy::new(|| {
    let (tx, rx) = unbounded();
    (tx, Arc::new(RwLock::new(rx)))
});

/// Helper functions for reading and writing the shared state.

/// Get the current blink interval.
pub fn get_blink_interval() -> u64 {
    SHARED_BLINK_SETTINGS.read().unwrap().blink_interval
}

/// Set the blink interval and broadcast the change.
pub fn set_blink_interval(val: u64) {
    {
        let mut state = SHARED_BLINK_SETTINGS.write().unwrap();
        state.blink_interval = val;
        // Broadcast the new state
        let _ = SETTINGS_CHANNEL.0.unbounded_send(state.clone());
    }
}

/// Get the current blink duration.
pub fn get_blink_duration() -> u64 {
    SHARED_BLINK_SETTINGS.read().unwrap().blink_duration
}

/// Set the blink duration and broadcast the change.
pub fn set_blink_duration(val: u64) {
    {
        let mut state = SHARED_BLINK_SETTINGS.write().unwrap();
        state.blink_duration = val;
        // Broadcast the new state
        let _ = SETTINGS_CHANNEL.0.unbounded_send(state.clone());
    }
}

/// Listen for settings changes (returns a receiver)
pub fn settings_receiver() -> Arc<RwLock<UnboundedReceiver<BlinkSettings>>> {
    SETTINGS_CHANNEL.1.clone()
}
