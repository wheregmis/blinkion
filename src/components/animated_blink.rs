use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
pub fn AnimatedBlink() -> Element {
    let mut transform = use_motion(Transform::default());
    let mut color = use_motion(Color::from_rgba(25, 118, 210, 255)); // #1976d2

    use_effect(move || {
        // Animate transform: scale and rotate
        let transform_seq = AnimationSequence::new()
            .then(
                Transform {
                    scale: 1.5,
                    rotation: 30.0_f32.to_radians(),
                    ..Default::default()
                },
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 500.0,
                    damping: 7.0,
                    mass: 1.0,
                    velocity: 12.0,
                })),
            )
            .then(
                Transform {
                    scale: 0.6,
                    rotation: -30.0_f32.to_radians(),
                    ..Default::default()
                },
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 350.0,
                    damping: 14.0,
                    mass: 1.0,
                    velocity: -8.0,
                })),
            )
            .then(
                Transform::default(),
                AnimationConfig::new(AnimationMode::Spring(Spring::default())),
            );

        let color_seq = AnimationSequence::new()
            .then(
                Color::from_rgba(25, 118, 210, 255), // #1976d2
                AnimationConfig::new(AnimationMode::Spring(Spring::default()))
                    .with_loop(LoopMode::Infinite),
            )
            .then(
                Color::from_rgba(67, 160, 71, 255), // #43a047
                AnimationConfig::new(AnimationMode::Spring(Spring::default())),
            )
            .then(
                Color::from_rgba(229, 57, 53, 255), // #e53935
                AnimationConfig::new(AnimationMode::Spring(Spring::default())),
            )
            .then(
                Color::from_rgba(25, 118, 210, 255),
                AnimationConfig::new(AnimationMode::Spring(Spring::default())),
            );

        transform.animate_sequence(transform_seq);
        color.animate_sequence(color_seq);
    });

    let style = format!(
        "transform: scale({}) rotate({}deg); transition: transform 0.2s cubic-bezier(.4,2,.6,1); background: none; display: block;",
        transform.get_value().scale,
        transform.get_value().rotation.to_degrees()
    );

    rsx! {
        svg {
            width: "120", height: "120", view_box: "0 0 120 120", xmlns: "http://www.w3.org/2000/svg",
            style: "{style}",
            path { d: "M10 60 Q60 10 110 60 Q60 110 10 60 Z", fill: "none", stroke: format!("#{:02x}{:02x}{:02x}", color.get_value().r as u8, color.get_value().g as u8, color.get_value().b as u8), stroke_width: "6" }
            circle { cx: "60", cy: "60", r: "16", fill: format!("#{:02x}{:02x}{:02x}", color.get_value().r as u8, color.get_value().g as u8, color.get_value().b as u8), opacity: "0.85" }
            circle { cx: "60", cy: "60", r: "7", fill: "#fff" }
            text { x: "60", y: "115", text_anchor: "middle", font_size: "14", fill: "#333", font_weight: "bold", "Blink!" }
        }
    }
}
