use dioxus::prelude::GlobalSignal;

pub static SHOW_WINDOW: GlobalSignal<bool> = GlobalSignal::new(|| false);
