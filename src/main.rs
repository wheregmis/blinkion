//! Multiwindow with tray icon example
//!
//! This example shows how to implement a simple multiwindow application and tray icon using dioxus.
//! This works by spawning a new window when the user clicks a button. We have to build a new virtualdom which has its
//! own context, root elements, etc.
//!
//! This is useful for apps that incorporate settings panels or persistent windows like Raycast.

use dioxus::desktop::{trayicon::init_tray_icon, window, WindowCloseBehaviour};
use dioxus::desktop::{Config, LogicalSize, WindowBuilder};
use dioxus::prelude::*;
use dioxus_desktop::muda::{Menu, MenuItem};
use dioxus_desktop::trayicon::{DioxusTrayIcon, TrayIconBuilder};
use dioxus_desktop::use_muda_event_handler;
use image::io::Reader as ImageReader;
use image::GenericImageView;
use std::time::Duration;

mod components;
mod reminder;
mod shared_state;
mod signals;

use crate::shared_state::{
    get_blink_interval, get_posture_duration, get_posture_interval, settings_receiver,
};
use components::reminder_window::{reminder_window, ReminderWindowProps};
use components::settings_window::SettingsWindow;
use reminder::ReminderType;
use signals::SHOW_WINDOW;

pub const STYLE: Asset = asset!("/assets/tailwind.css");

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
    let menu_item = MenuItem::new("Settings", true, None);
    let menu_item_id = menu_item.id().clone();
    let _ = tray_menu.append(&menu_item);

    // Decode PNG at runtime for tray icon
    let img = ImageReader::open("assets/tray.png")
        .expect("icon.png not found")
        .decode()
        .expect("decode failed");
    let (width, height) = img.dimensions();
    let rgba = img.to_rgba8().into_raw();
    let icon = DioxusTrayIcon::from_rgba(rgba, width, height).expect("icon parse failed");
    init_tray_icon(tray_menu, Some(icon));

    use_muda_event_handler(move |event| {
        if *event.id() == menu_item_id {
            window().new_window(
                VirtualDom::new(SettingsWindow),
                Config::default().with_window(
                    WindowBuilder::new()
                        .with_title("Blinkion Settings")
                        .with_transparent(false)
                        .with_always_on_top(true)
                        .with_decorations(true)
                        .with_inner_size(LogicalSize::new(800.0, 500.0)),
                ),
            );
        }
    });

    // Timer: set SHOW_WINDOW to true every blink_interval seconds
    // Listen for blink interval changes via the broadcast channel
    use_effect(move || {
        spawn(async move {
            use std::time::{Duration, Instant};
            let mut last_trigger = Instant::now();
            let mut interval = get_blink_interval();

            // Create a new broadcast receiver for settings changes
            let mut rx = settings_receiver();

            loop {
                tokio::time::sleep(Duration::from_millis(100)).await;
                // Check for new settings
                while let Ok(settings) = rx.try_recv() {
                    interval = settings.blink_interval;
                }
                if last_trigger.elapsed().as_secs() >= interval {
                    *SHOW_WINDOW.write() = true;
                    last_trigger = Instant::now();
                }
            }
        });
    });

    // Posture Reminder Timer: triggers posture reminder window every posture_interval seconds
    use_effect(move || {
        spawn(async move {
            use std::time::{Duration, Instant};
            let mut last_trigger = Instant::now();
            let mut interval = get_posture_interval();
            let mut duration = get_posture_duration();

            // Create a new broadcast receiver for settings changes
            let mut rx = settings_receiver();

            loop {
                tokio::time::sleep(Duration::from_millis(100)).await;
                // Check for new settings
                while let Ok(settings) = rx.try_recv() {
                    interval = settings.posture_interval;
                    duration = settings.posture_duration;
                }
                if last_trigger.elapsed().as_secs() >= interval {
                    // Open a posture reminder window, passing duration as a prop
                    window().new_window(
                        VirtualDom::new_with_props(
                            reminder_window,
                            ReminderWindowProps {
                                kind: ReminderType::Posture,
                                duration: Some(duration),
                            },
                        ),
                        Config::default().with_window(
                            WindowBuilder::new()
                                .with_title("Posture Reminder")
                                .with_transparent(true)
                                .with_always_on_top(true)
                                .with_decorations(false)
                                .with_inner_size(LogicalSize::new(240.0, 200.0)),
                        ),
                    );
                    // Wait for the posture duration before closing (handled in ReminderWindow)
                    last_trigger = Instant::now();
                }
            }
        });
    });

    // Poll SHOW_WINDOW every 100ms, show window if true, then reset
    use_future(|| async move {
        loop {
            tokio::time::sleep(Duration::from_millis(100)).await;
            if *SHOW_WINDOW.read() {
                window().new_window(
                    VirtualDom::new_with_props(
                        reminder_window,
                        ReminderWindowProps {
                            kind: ReminderType::Blink,
                            duration: None,
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

    VNode::empty()
}
