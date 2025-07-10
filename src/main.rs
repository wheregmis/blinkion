use dioxus::prelude::*;
use dioxus::LaunchBuilder;
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};
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
    // State: which reminder is active
    let mut blink_active = use_signal(|| false);
    let mut posture_active = use_signal(|| false);

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
                svg { width: "120", height: "120", view_box: "0 0 120 120", xmlns: "http://www.w3.org/2000/svg",
                    circle { cx: "60", cy: "60", r: "55", fill: "#e0f7fa", stroke: "#00796b", stroke_width: "5" }
                    text { x: "60", y: "70", text_anchor: "middle", font_size: "22", fill: "#00796b", font_family: "Arial", "Posture!" }
                }
            } else if blink_active() {
                svg { width: "120", height: "120", view_box: "0 0 120 120", xmlns: "http://www.w3.org/2000/svg",
                    ellipse { cx: "60", cy: "60", rx: "50", ry: "30", fill: "#fffde7", stroke: "#fbc02d", stroke_width: "5" }
                    circle { cx: "60", cy: "60", r: "12", fill: "#1976d2" }
                    rect { x: "10", y: "30", width: "100", height: "60", fill: "#fffde7", opacity: "0.3" }
                }
            }
        }
    }
}
