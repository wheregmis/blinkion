use crate::{
    shared_state::{
        get_blink_duration, get_blink_interval, set_blink_duration, set_blink_interval,
    },
    STYLE,
};
use dioxus::prelude::*;

#[component]
pub fn SettingsWindow() -> Element {
    let mut local_interval = use_signal(|| get_blink_interval());
    let mut local_duration = use_signal(|| get_blink_duration());

    rsx! {
                document::Link { rel: "stylesheet", href: STYLE }
        div {
            class: "w-screen h-screen flex items-center justify-center bg-gradient-to-br from-white to-blue-50",
            div {
                class: "bg-white p-10 rounded-2xl min-w-[340px] min-h-[260px] shadow-2xl flex flex-col items-center",
                h2 { class: "text-4xl font-extrabold mb-8 text-center", "Blinkion Settings" }
                div { class: "w-full mb-6",
                    label { class: "block mb-2 text-lg font-medium", "Blink interval (seconds):" }
                    input {
                        r#type: "number",
                        min: "1",
                        class: "w-full mb-4 px-3 py-2 border-2 border-gray-200 rounded-lg focus:outline-none focus:border-blue-500 transition",
                        value: local_interval().to_string(),
                        oninput: move |e| if let Ok(val) = e.value().parse() { local_interval.set(val); },
                    }
                }
                div { class: "w-full mb-8",
                    label { class: "block mb-2 text-lg font-medium", "Blink duration (seconds):" }
                    input {
                        r#type: "number",
                        min: "1",
                        class: "w-full px-3 py-2 border-2 border-gray-200 rounded-lg focus:outline-none focus:border-blue-500 transition",
                        value: local_duration().to_string(),
                        oninput: move |e| if let Ok(val) = e.value().parse() { local_duration.set(val); },
                    }
                }
                button {
                    class: "w-full py-3 rounded-lg bg-blue-700 text-white text-xl font-semibold shadow hover:bg-blue-800 transition focus:outline-none focus:ring-2 focus:ring-blue-400",
                    onclick: move |_| {
                        set_blink_interval(local_interval());
                        set_blink_duration(local_duration());
                        println!("Saved: interval={} duration={}", local_interval(), local_duration());
                    },
                    "Save"
                }
            }
        }
    }
}
