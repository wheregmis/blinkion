use dioxus::prelude::*;
use dioxus_motion::prelude::*;

use crate::STYLE;

#[component]
pub fn AnimatedBlink() -> Element {
    let mut scale_y = use_motion(1.0f32);

    use_effect(move || {
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
         document::Link { rel: "stylesheet", href: STYLE },
        svg {
            width: "120", height: "120", view_box: "0 0 120 120", xmlns: "http://www.w3.org/2000/svg",
            style: "{style}",
            path { d: "M10 60 Q60 10 110 60 Q60 110 10 60 Z", fill: "none", stroke: "#1976d2", stroke_width: "6" }
            circle { cx: "60", cy: "60", r: "16", fill: "#1976d2", opacity: "0.85" }
            circle { cx: "60", cy: "60", r: "7", fill: "#fff" }
        }
    }
}
