use crate::components::animated_blink::AnimatedBlink;
use crate::reminder::ReminderType;
use crate::shared_state::get_blink_duration;
use crate::STYLE;
use dioxus::desktop::use_window;
use dioxus::prelude::*;
use std::time::Duration;

#[derive(Props, Clone, PartialEq)]
pub struct ReminderWindowProps {
    pub kind: ReminderType,
}

pub fn ReminderWindow(props: ReminderWindowProps) -> Element {
    let win = use_window();

    let duration = get_blink_duration();

    use_effect(move || {
        let win = win.clone();
        spawn(async move {
            tokio::time::sleep(Duration::from_secs(duration)).await;
            win.close();
        });
    });

    rsx! {
         document::Link { rel: "stylesheet", href: STYLE },
        div {
            style: "width: 100vw; height: 100vh; overflow: hidden; display: flex; align-items: center; justify-content: center; background: rgba(255,255,255,0.0);",
            match props.kind {
                ReminderType::Blink => rsx! { AnimatedBlink {} },
            }
        }
    }
}
