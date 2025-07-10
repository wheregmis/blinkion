use dioxus::prelude::*;
use dioxus::LaunchBuilder;
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};
use dioxus_motion::prelude::*;
use std::time::Duration;

fn main() {
    LaunchBuilder::new()
        .with_cfg(
            Config::default().with_window(
                WindowBuilder::new()
                    .with_title("Blinkion")
                    .with_transparent(true)
                    .with_always_on_top(true)
                    .with_decorations(false)
                    .with_inner_size(LogicalSize::new(220.0, 180.0)),
            ),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    let mut blink_active = use_signal(|| false);
    let mut posture_active = use_signal(|| false);

    // Blink timer (every 30s)
    use_future(move || async move {
        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;
            blink_active.set(true);
            tokio::time::sleep(Duration::from_secs(3)).await; // Show for 3s
            blink_active.set(false);
        }
    });
    // Posture timer (every 2min)
    use_future(move || async move {
        loop {
            tokio::time::sleep(Duration::from_secs(12)).await;
            posture_active.set(true);
            tokio::time::sleep(Duration::from_secs(5)).await; // Show for 5s
            posture_active.set(false);
        }
    });

    rsx! {
        div {
            style: "width: 100vw; height: 100vh; display: flex; align-items: center; justify-content: center; background: rgba(255,255,255,0.0);",
            if posture_active() {
                AnimatedPosture {}
            } else if blink_active() {
                AnimatedBlink {}
            }
        }
    }
}

#[component]
fn AnimatedBlink() -> Element {
    // Pulse scale animation
    let mut scale = use_motion(1.0f32);
    use_effect(move || {
        scale.animate_to(
            1.18,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 120.0,
                damping: 7.0,
                mass: 0.5,
                velocity: 1.0,
            }))
            .with_loop(LoopMode::Infinite),
        );
    });
    rsx! {
        svg {
            width: "120", height: "120", view_box: "0 0 120 120", xmlns: "http://www.w3.org/2000/svg",
            style: "background: none; display: block; transform: scale({scale.get_value()}); transition: transform 0.2s;",
            // Eye outline
            path { d: "M10 60 Q60 10 110 60 Q60 110 10 60 Z", fill: "none", stroke: "#1976d2", stroke_width: "6" }
            // Iris
            circle { cx: "60", cy: "60", r: "16", fill: "#1976d2", opacity: "0.85" }
            // Pupil
            circle { cx: "60", cy: "60", r: "7", fill: "#fff" }
        }
    }
}

#[component]
fn AnimatedPosture() -> Element {
    // Gentle bounce animation
    let mut translate = use_motion(0.0f32);
    use_effect(move || {
        translate.animate_to(
            -10.0,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 80.0,
                damping: 8.0,
                mass: 0.7,
                velocity: 1.0,
            }))
            .with_loop(LoopMode::Infinite),
        );
    });
    rsx! {
        svg {
            width: "120", height: "120", view_box: "0 0 120 120", xmlns: "http://www.w3.org/2000/svg",
            style: "background: none; display: block; transform: translateY({translate.get_value()}px); transition: transform 0.2s;",
            // Stylized posture icon: a person sitting upright
            // Head
            circle { cx: "60", cy: "38", r: "13", fill: "#00796b", opacity: "0.85" }
            // Body
            rect { x: "54", y: "51", width: "12", height: "32", rx: "6", fill: "#00796b", opacity: "0.85" }
            // Legs
            rect { x: "54", y: "83", width: "5", height: "20", rx: "2.5", fill: "#00796b", opacity: "0.85" }
            rect { x: "61", y: "83", width: "5", height: "20", rx: "2.5", fill: "#00796b", opacity: "0.85" }
            // Arms
            rect { x: "44", y: "55", width: "8", height: "22", rx: "4", fill: "#00796b", opacity: "0.7", transform: "rotate(-18 48 66)" }
            rect { x: "68", y: "55", width: "8", height: "22", rx: "4", fill: "#00796b", opacity: "0.7", transform: "rotate(18 72 66)" }
        }
    }
}
