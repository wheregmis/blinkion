use crate::{
    shared_state::{
        get_blink_duration, get_blink_interval, get_posture_duration, get_posture_interval,
        set_blink_duration, set_blink_interval, set_posture_duration, set_posture_interval,
    },
    STYLE,
};
use dioxus::prelude::*;

#[component]
pub fn SettingsWindow() -> Element {
    let mut local_interval = use_signal(get_blink_interval);
    let mut local_duration = use_signal(get_blink_duration);
    let mut local_posture_interval = use_signal(get_posture_interval);
    let mut local_posture_duration = use_signal(get_posture_duration);

    rsx! {
        document::Link { rel: "stylesheet", href: STYLE }
        div {
            class: "w-screen h-screen flex items-center justify-center bg-gradient-to-br from-white to-blue-50",
            div {
                class: "bg-white p-5 rounded-xl w-[320px] shadow-lg flex flex-col items-center",
                h2 { class: "text-2xl font-bold mb-4 text-center", "Blinkion Settings" }
                div { class: "w-full mb-3",
                    label { class: "block mb-1 text-base font-medium", "Blink interval (seconds):" }
                    input {
                        r#type: "number",
                        min: "1",
                        class: "w-full mb-2 px-2 py-1 border border-gray-200 rounded focus:outline-none focus:border-blue-500 text-base",
                        value: local_interval().to_string(),
                        oninput: move |e| if let Ok(val) = e.value().parse() { local_interval.set(val); },
                    }
                }
                div { class: "w-full mb-3",
                    label { class: "block mb-1 text-base font-medium", "Blink duration (seconds):" }
                    input {
                        r#type: "number",
                        min: "1",
                        class: "w-full mb-2 px-2 py-1 border border-gray-200 rounded focus:outline-none focus:border-blue-500 text-base",
                        value: local_duration().to_string(),
                        oninput: move |e| if let Ok(val) = e.value().parse() { local_duration.set(val); },
                    }
                }
                div { class: "w-full mb-3",
                    label { class: "block mb-1 text-base font-medium", "Posture interval (seconds):" }
                    input {
                        r#type: "number",
                        min: "1",
                        class: "w-full mb-2 px-2 py-1 border border-gray-200 rounded focus:outline-none focus:border-green-500 text-base",
                        value: local_posture_interval().to_string(),
                        oninput: move |e| if let Ok(val) = e.value().parse() { local_posture_interval.set(val); },
                    }
                }
                div { class: "w-full mb-4",
                    label { class: "block mb-1 text-base font-medium", "Posture duration (seconds):" }
                    input {
                        r#type: "number",
                        min: "1",
                        class: "w-full px-2 py-1 border border-gray-200 rounded focus:outline-none focus:border-green-500 text-base",
                        value: local_posture_duration().to_string(),
                        oninput: move |e| if let Ok(val) = e.value().parse() { local_posture_duration.set(val); },
                    }
                }
                button {
                    class: "w-full py-2 rounded bg-blue-700 text-white text-base font-semibold shadow hover:bg-blue-800 transition focus:outline-none focus:ring-2 focus:ring-blue-400",
                    onclick: move |_| {
                        set_blink_interval(local_interval());
                        set_blink_duration(local_duration());
                        set_posture_interval(local_posture_interval());
                        set_posture_duration(local_posture_duration());
                        println!(
                            "Saved: interval={} duration={} posture_interval={} posture_duration={}",
                            local_interval(),
                            local_duration(),
                            local_posture_interval(),
                            local_posture_duration()
                        );
                    },
                    "Save"
                }
            }
        }
    }
}
