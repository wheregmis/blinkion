use dioxus::prelude::GlobalSignal;

pub static BLINK_INTERVAL: GlobalSignal<u64> = GlobalSignal::new(|| 30);
pub static BLINK_DURATION: GlobalSignal<u64> = GlobalSignal::new(|| 3);
pub static SHOW_WINDOW: GlobalSignal<bool> = GlobalSignal::new(|| false);
pub static TIMER_RESET: GlobalSignal<u64> = GlobalSignal::new(|| 0);
