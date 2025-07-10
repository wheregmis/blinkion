//! Multiwindow with tray icon example
//!
//! This example shows how to implement a simple multiwindow application and tray icon using dioxus.
//! This works by spawning a new window when the user clicks a button. We have to build a new virtualdom which has its
//! own context, root elements, etc.
//!
//! This is useful for apps that incorporate settings panels or persistent windows like Raycast.

use dioxus::desktop::{
    trayicon::{default_tray_icon, init_tray_icon},
    window, WindowCloseBehaviour,
};
use dioxus::desktop::{use_window, Config, LogicalSize, WindowBuilder};
use dioxus::prelude::*;
use std::time::Duration;

#[derive(Clone, Copy, PartialEq)]
enum ReminderType {
    Blink,
}

fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(
            Config::default().with_window(
                WindowBuilder::new()
                    .with_title("Blinkion")
                    .with_transparent(true)
                    .with_always_on_top(true)
                    .with_decorations(false)
                    .with_inner_size(LogicalSize::new(1.0, 1.0))
                    .with_visible(true),
            ),
        )
        .launch(app);
}

fn app() -> Element {
    use_hook(|| {
        window().set_close_behavior(WindowCloseBehaviour::WindowHides);
        init_tray_icon(default_tray_icon(), None)
    });

    // Blink reminder every 30s
    use_future(|| async move {
        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;
            window().new_window(
                VirtualDom::new_with_props(
                    ReminderWindow,
                    ReminderWindowProps {
                        kind: ReminderType::Blink,
                    },
                ),
                Default::default(),
            );
        }
    });

    // No visible UI
    rsx!()
}

#[component]
fn ReminderWindow(kind: ReminderType) -> Element {
    let win = use_window();
    use_future(move || {
        let win = win.clone();
        async move {
            // Wait 3 seconds before closing
            tokio::time::sleep(Duration::from_secs(3)).await;
            win.close();
        }
    });
    rsx! {
        div {
            style: "width: 100vw; height: 100vh; display: flex; align-items: center; justify-content: center; background: rgba(255,255,255,0.0);",
            match kind {
                ReminderType::Blink => rsx! { AnimatedBlink {} },
            }
        }
    }
}

#[component]
fn AnimatedBlink() -> Element {
    // Pulse scale animation (replace with your dioxus-motion code if needed)
    rsx! {
        svg {
            width: "120", height: "120", view_box: "0 0 120 120", xmlns: "http://www.w3.org/2000/svg",
            style: "background: none; display: block;",
            // Eye outline
            path { d: "M10 60 Q60 10 110 60 Q60 110 10 60 Z", fill: "none", stroke: "#1976d2", stroke_width: "6" }
            // Iris
            circle { cx: "60", cy: "60", r: "16", fill: "#1976d2", opacity: "0.85" }
            // Pupil
            circle { cx: "60", cy: "60", r: "7", fill: "#fff" }
        }
    }
}
