use crate::components::animated_blink::AnimatedBlink;
use crate::reminder::ReminderType;
use crate::shared_state::{get_blink_duration, get_posture_duration};
use dioxus::desktop::use_window;
use dioxus::prelude::*;
use std::time::Duration;

#[derive(Props, Clone, PartialEq)]
pub struct ReminderWindowProps {
    pub kind: ReminderType,
}

pub fn ReminderWindow(props: ReminderWindowProps) -> Element {
    let win = use_window();

    // Determine duration and animation based on reminder type
    let (duration, content) = match props.kind {
        ReminderType::Blink => (get_blink_duration(), rsx! { AnimatedBlink {} }),
        ReminderType::Posture => (get_posture_duration(), rsx! { AnimatedPosture {} }),
    };

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

// AnimatedPosture: simple SVG animation for posture reminder
use dioxus_motion::prelude::*;

#[component]
fn AnimatedPosture() -> Element {
    let mut transform = use_motion(Transform::default());
    let mut color = use_motion(Color::from_rgba(67, 160, 71, 255)); // #43a047

    use_effect(move || {
        // Animate transform: scale and rotate
        let transform_seq = AnimationSequence::new()
            .then(
                Transform {
                    scale: 1.4,
                    rotation: 25.0_f32.to_radians(),
                    ..Default::default()
                },
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 400.0,
                    damping: 8.0,
                    mass: 1.0,
                    velocity: 8.0,
                })),
            )
            .then(
                Transform {
                    scale: 0.7,
                    rotation: -25.0_f32.to_radians(),
                    ..Default::default()
                },
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 300.0,
                    damping: 12.0,
                    mass: 1.0,
                    velocity: -6.0,
                })),
            )
            .then(
                Transform::default(),
                AnimationConfig::new(AnimationMode::Spring(Spring::default())),
            );

        let color_seq = AnimationSequence::new()
            .then(
                Color::from_rgba(67, 160, 71, 255), // #43a047
                AnimationConfig::new(AnimationMode::Spring(Spring::default()))
                    .with_loop(LoopMode::Infinite),
            )
            .then(
                Color::from_rgba(25, 118, 210, 255), // #1976d2
                AnimationConfig::new(AnimationMode::Spring(Spring::default())),
            )
            .then(
                Color::from_rgba(229, 57, 53, 255), // #e53935
                AnimationConfig::new(AnimationMode::Spring(Spring::default())),
            )
            .then(
                Color::from_rgba(67, 160, 71, 255),
                AnimationConfig::new(AnimationMode::Spring(Spring::default())),
            );

        transform.animate_sequence(transform_seq);
        color.animate_sequence(color_seq);
    });

    let style = format!(
        "transform: scale({}) rotate({}deg); transition: transform 0.2s cubic-bezier(.4,2,.6,1);",
        transform.get_value().scale,
        transform.get_value().rotation.to_degrees()
    );

    rsx! {
        svg {
            width: "120", height: "120", view_box: "0 0 120 120", xmlns: "http://www.w3.org/2000/svg",
            style: "{style}",
            rect { x: "55", y: "20", width: "10", height: "80", fill: format!("#{:02x}{:02x}{:02x}", color.get_value().r as u8, color.get_value().g as u8, color.get_value().b as u8), rx: "5" }
            ellipse { cx: "60", cy: "35", rx: "18", ry: "8", fill: "#1976d2", opacity: "0.7" }
            path { d: "M60 100 Q70 110 80 100", stroke: "#1976d2", stroke_width: "4", fill: "none" }
            path { d: "M60 100 Q50 110 40 100", stroke: "#1976d2", stroke_width: "4", fill: "none" }
            text { x: "60", y: "115", text_anchor: "middle", font_size: "14", fill: "#333", font_weight: "bold", "Sit up straight!" }
        }
    }
}
