use crate::components::animated_blink::AnimatedBlink;
use crate::components::animated_posture::AnimatedPosture;
use crate::reminder::ReminderType;
use crate::shared_state::{get_blink_duration, get_posture_duration};
use dioxus::desktop::use_window;
use dioxus::prelude::*;
use std::time::Duration;

#[derive(Props, Clone, PartialEq)]
pub struct ReminderWindowProps {
    pub kind: ReminderType,
    pub duration: Option<u64>,
}

pub fn reminder_window(props: ReminderWindowProps) -> Element {
    let win = use_window();

    // Determine duration and animation based on reminder type
    let (default_duration, content) = match props.kind {
        ReminderType::Blink => (get_blink_duration(), rsx! { AnimatedBlink {} }),
        ReminderType::Posture => (get_posture_duration(), rsx! { AnimatedPosture {} }),
    };
    let duration = props.duration.unwrap_or(default_duration);

    use_effect(move || {
        let win = win.clone();
        spawn(async move {
            tokio::time::sleep(Duration::from_secs(duration)).await;
            win.close();
        });
    });

    rsx! {
        div {
            style: "width: 100vw; height: 100vh; overflow: hidden; display: flex; align-items: center; justify-content: center; background: rgba(255,255,255,0.0);",
            {content}
        }
    }
}
