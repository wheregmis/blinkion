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
                Color::from_rgba(25, 118, 210, 255),
                AnimationConfig::new(AnimationMode::Spring(Spring::default()))
                    .with_loop(LoopMode::Infinite),
            )
            .then(
                Color::from_rgba(67, 160, 71, 255),
                AnimationConfig::new(AnimationMode::Spring(Spring::default()))
                    .with_loop(LoopMode::Infinite),
            )
            .then(
                Color::from_rgba(229, 57, 53, 255),
                AnimationConfig::new(AnimationMode::Spring(Spring::default()))
                    .with_loop(LoopMode::Infinite),
            )
            .then(
                Color::from_rgba(25, 118, 210, 255),
                AnimationConfig::new(AnimationMode::Spring(Spring::default()))
                    .with_loop(LoopMode::Infinite),
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
        style: "{style}",
        fill: "#000000",
        height: "200px",
        id: "Layer_1",
        "space": "preserve",
        version: "1.1",
        view_box: "0 0 512 512",
        width: "200px",
        "xlink": "http://www.w3.org/1999/xlink",
        xmlns: "http://www.w3.org/2000/svg",
        g { id: "SVGRepo_bgCarrier", stroke_width: "0" }
        g {
            id: "SVGRepo_tracerCarrier",
            stroke_linecap: "round",
            stroke_linejoin: "round",
        }
        g { id: "SVGRepo_iconCarrier",
            g {
                g {
                    path { d: "M256,0C114.842,0,0,114.84,0,256s114.842,256,256,256s256-114.84,256-256S397.158,0,256,0z M256,474.537 c-120.501,0-218.537-98.036-218.537-218.537S135.499,37.463,256,37.463S474.537,135.499,474.537,256S376.501,474.537,256,474.537z " }
                }
            }
            g {
                g {
                    path { d: "M293.463,224.781v37.463c19.945,0,43.388,2.601,71.664,7.951l6.964-36.812 C341.493,227.594,315.773,224.781,293.463,224.781z" }
                }
            }
            g {
                g {
                    ellipse {
                        cx: "196.683",
                        cy: "243.512",
                        rx: "21.854",
                        ry: "37.463",
                        fill: {format!("#{:02x}{:02x}{:02x}", color.get_value().r as u8, color.get_value().g as u8, color.get_value().b as u8)}
                    }
                }
            }
            g {
                g {
                    path { d: "M300.693,337.422c-8.346,24.717-22.965,36.732-44.691,36.732c-21.796,0-36.805-12.276-44.611-36.487l-35.656,11.496 c12.797,39.69,42.054,62.454,80.267,62.454c17.361,0,33.521-4.95,46.736-14.316c15.074-10.685,26.328-26.798,33.452-47.893 L300.693,337.422z" }
                }
            }
            g {
                g {
                    path { d: "M293.462,170.787v37.463c42.939,0,81.334,7.658,117.378,23.408l15-34.329C384.97,179.47,341.669,170.787,293.462,170.787z " }
                }
            }
            g {
                g {
                    path { d: "M211.034,126.445c-44.171,19.309-80.369,44.609-110.664,77.343l27.494,25.446c26.718-28.869,58.831-51.262,98.177-68.463 L211.034,126.445z" }
                }
            }
        }
    }
    }
}
