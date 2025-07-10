//! Multiwindow with tray icon example
//!
//! This example shows how to implement a simple multiwindow application and tray icon using dioxus.
//! This works by spawning a new window when the user clicks a button. We have to build a new virtualdom which has its
//! own context, root elements, etc.
//!
//! This is useful for apps that incorporate settings panels or persistent windows like Raycast.

use dioxus::desktop::{trayicon::init_tray_icon, window, WindowCloseBehaviour};
use dioxus::desktop::{use_window, Config, LogicalSize, WindowBuilder};
use dioxus::prelude::*;
use dioxus_desktop::muda::{Menu, MenuItem};
use dioxus_desktop::use_muda_event_handler;
use dioxus_motion::prelude::*;
use std::time::Duration;

#[derive(Clone, Copy, PartialEq)]
enum ReminderType {
    Blink,
}

// Global signals for blink interval and duration
static BLINK_INTERVAL: GlobalSignal<u64> = GlobalSignal::new(|| 30);
static BLINK_DURATION: GlobalSignal<u64> = GlobalSignal::new(|| 3);
// Global signal to control showing the reminder window
static SHOW_WINDOW: GlobalSignal<bool> = GlobalSignal::new(|| false);

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
    // Listen for tray menu events

    window().set_close_behavior(WindowCloseBehaviour::WindowHides);
    // Create a tray menu with a Settings item
    let tray_menu = Menu::new();
    let menu_item = MenuItem::new("settings", true, None);
    let menu_item_id = menu_item.id().clone();
    let _ = tray_menu.append(&menu_item);

    init_tray_icon(tray_menu, None);

    use_muda_event_handler(move |event| {
        if *event.id() == menu_item_id {
            println!("Settings");
            window().new_window(
                VirtualDom::new(SettingsWindow),
                Config::default().with_window(
                    WindowBuilder::new()
                        .with_title("Blinkion Settings")
                        .with_transparent(false)
                        .with_always_on_top(true)
                        .with_decorations(true)
                        .with_inner_size(LogicalSize::new(360.0, 260.0)),
                ),
            );
        }
    });

    // Timer: set SHOW_WINDOW to true every blink_interval seconds
    use_future(|| async move {
        loop {
            let interval = *BLINK_INTERVAL.read();
            tokio::time::sleep(Duration::from_secs(interval)).await;
            *SHOW_WINDOW.write() = true;
        }
    });

    // Poll SHOW_WINDOW every 100ms, show window if true, then reset
    use_future(|| async move {
        loop {
            tokio::time::sleep(Duration::from_millis(100)).await;
            if *SHOW_WINDOW.read() {
                window().new_window(
                    VirtualDom::new_with_props(
                        ReminderWindow,
                        ReminderWindowProps {
                            kind: ReminderType::Blink,
                        },
                    ),
                    Config::default().with_window(
                        WindowBuilder::new()
                            .with_title("Blinkion")
                            .with_transparent(true)
                            .with_always_on_top(true)
                            .with_decorations(false)
                            .with_inner_size(LogicalSize::new(220.0, 180.0)),
                    ),
                );
                *SHOW_WINDOW.write() = false;
            }
        }
    });

    rsx!()
}

#[component]
fn ReminderWindow(kind: ReminderType) -> Element {
    let win = use_window();
    use_future(move || {
        let win = win.clone();
        async move {
            // Use global blink duration
            let duration = *BLINK_DURATION.read();
            tokio::time::sleep(Duration::from_secs(duration)).await;
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
fn SettingsWindow() -> Element {
    let mut blink_interval = use_signal(|| *BLINK_INTERVAL.read());
    let mut blink_duration = use_signal(|| *BLINK_DURATION.read());

    rsx! {
        div {
            style: "width: 100vw; height: 100vh; display: flex; align-items: center; justify-content: center; background: white; color: black;",
            div {
                style: "padding: 32px; border-radius: 12px; min-width: 300px; min-height: 200px; box-shadow: 0 2px 16px rgba(0,0,0,0.2);",
                h2 { style: "font-size: 2rem; font-weight: bold;", "Blinkion Settings" }
                div { style: "margin-top: 24px;",
                    label { style: "display: block; margin-bottom: 8px;", "Blink interval (seconds):" }
                    input {
                        r#type: "number",
                        min: "1",
                        value: "{blink_interval}",
                        oninput: move |e| if let Ok(val) = e.value().parse() { blink_interval.set(val) },
                        style: "width: 80px; margin-bottom: 16px;"
                    }
                }
                div {
                    label { style: "display: block; margin-bottom: 8px;", "Blink duration (seconds):" }
                    input {
                        r#type: "number",
                        min: "1",
                        value: "{blink_duration}",
                        oninput: move |e| if let Ok(val) = e.value().parse() { blink_duration.set(val) },
                        style: "width: 80px; margin-bottom: 16px;"
                    }
                }
                button {
                    style: "margin-top: 24px; padding: 8px 24px; border-radius: 6px; background: #1976d2; color: white; border: none; font-size: 1rem; cursor: pointer;",
                    onclick: move |_| {
                        *BLINK_INTERVAL.write() = *blink_interval.read();
                        *BLINK_DURATION.write() = *blink_duration.read();
                        println!("Saved: interval={} duration={}", blink_interval(), blink_duration());
                    },
                    "Save"
                }
            }
        }
    }
}

#[component]
fn AnimatedBlink() -> Element {
    // Animate the vertical scale for a blink effect
    let mut scale_y = use_motion(1.0f32);

    use_effect(move || {
        // Animate to closed (scaleY=0.1), then open (scaleY=1.0), repeat 3 times by chaining .then
        let sequence = AnimationSequence::new()
            .then(
                0.1,
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 300.0,
                    damping: 15.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            )
            .then(
                1.0,
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 200.0,
                    damping: 10.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            )
            // 2nd blink
            .then(
                0.1,
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 300.0,
                    damping: 15.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            )
            .then(
                1.0,
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 200.0,
                    damping: 10.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            )
            // 3rd blink
            .then(
                0.1,
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 300.0,
                    damping: 15.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            )
            .then(
                1.0,
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 200.0,
                    damping: 10.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            );
        scale_y.animate_sequence(sequence);
    });

    let style = format!(
        "transform: scaleY({}); transition: transform 0.2s cubic-bezier(.4,2,.6,1); background: none; display: block;",
        scale_y.get_value()
    );

    rsx! {
        svg {
            width: "120", height: "120", view_box: "0 0 120 120", xmlns: "http://www.w3.org/2000/svg",
            style: "{style}",
            // Eye outline
            path { d: "M10 60 Q60 10 110 60 Q60 110 10 60 Z", fill: "none", stroke: "#1976d2", stroke_width: "6" }
            // Iris
            circle { cx: "60", cy: "60", r: "16", fill: "#1976d2", opacity: "0.85" }
            // Pupil
            circle { cx: "60", cy: "60", r: "7", fill: "#fff" }
        }
    }
}
