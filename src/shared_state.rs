use once_cell::sync::Lazy;
use std::sync::{Arc, RwLock};
use tokio::sync::broadcast;

/// The shared application state for blink settings.
#[derive(Debug, Clone)]
pub struct BlinkSettings {
    pub blink_interval: u64,
    pub blink_duration: u64,
    pub posture_interval: u64,
    pub posture_duration: u64,
}

impl Default for BlinkSettings {
    fn default() -> Self {
        Self {
            blink_interval: 30,
            blink_duration: 3,
            posture_interval: 60,
            posture_duration: 5,
        }
    }
}

/// A globally accessible, thread-safe shared state for blink settings.
pub static SHARED_BLINK_SETTINGS: Lazy<Arc<RwLock<BlinkSettings>>> =
    Lazy::new(|| Arc::new(RwLock::new(BlinkSettings::default())));

/// Broadcast channel for settings changes (tokio broadcast)
pub static SETTINGS_CHANNEL: Lazy<broadcast::Sender<BlinkSettings>> = Lazy::new(|| {
    // 16 is the channel buffer size; adjust as needed
    let (tx, _) = broadcast::channel(16);
    tx
});

/// Helper functions for reading and writing the shared state.

/// Get the current blink interval.
pub fn get_blink_interval() -> u64 {
    SHARED_BLINK_SETTINGS.read().unwrap().blink_interval
}

/// Get the current posture interval.
pub fn get_posture_interval() -> u64 {
    SHARED_BLINK_SETTINGS.read().unwrap().posture_interval
}

/// Set the blink interval and broadcast the change.
pub fn set_blink_interval(val: u64) {
    {
        let mut state = SHARED_BLINK_SETTINGS.write().unwrap();
        state.blink_interval = val;
        // Broadcast the new state
        let _ = SETTINGS_CHANNEL.send(state.clone());
    }
}

/// Set the posture interval and broadcast the change.
pub fn set_posture_interval(val: u64) {
    {
        let mut state = SHARED_BLINK_SETTINGS.write().unwrap();
        state.posture_interval = val;
        // Broadcast the new state
        let _ = SETTINGS_CHANNEL.send(state.clone());
    }
}

/// Get the current blink duration.
pub fn get_blink_duration() -> u64 {
    SHARED_BLINK_SETTINGS.read().unwrap().blink_duration
}

/// Get the current posture duration.
pub fn get_posture_duration() -> u64 {
    SHARED_BLINK_SETTINGS.read().unwrap().posture_duration
}

/// Set the blink duration and broadcast the change.
pub fn set_blink_duration(val: u64) {
    {
        let mut state = SHARED_BLINK_SETTINGS.write().unwrap();
        state.blink_duration = val;
        // Broadcast the new state
        let _ = SETTINGS_CHANNEL.send(state.clone());
    }
}

/// Set the posture duration and broadcast the change.
pub fn set_posture_duration(val: u64) {
    {
        let mut state = SHARED_BLINK_SETTINGS.write().unwrap();
        state.posture_duration = val;
        // Broadcast the new state
        let _ = SETTINGS_CHANNEL.send(state.clone());
    }
}

/// Listen for settings changes (returns a new broadcast receiver)
pub fn settings_receiver() -> broadcast::Receiver<BlinkSettings> {
    SETTINGS_CHANNEL.subscribe()
}
