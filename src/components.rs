use dioxus::prelude::*;

#[component]
pub fn Stars(count: u8) -> Element {
    rsx! {
        span { class: "stars", aria_label: "{count} de 3 estrelas",
            for index in 0..3u8 {
                span { class: if index < count { "star on" } else { "star" }, "★" }
            }
        }
    }
}

#[component]
pub fn ProgressBar(
    fraction: f32,
    #[props(default = "var(--green)".to_string())] color: String,
) -> Element {
    let percent = (fraction.clamp(0.0, 1.0) * 100.0).round();
    rsx! {
        div { class: "bar",
            div { class: "bar-fill", style: "width: {percent}%; background: {color};" }
        }
    }
}

#[component]
pub fn PageHeader(title: String, #[props(default)] subtitle: Option<String>) -> Element {
    rsx! {
        div { class: "page-header",
            h1 { "{title}" }
            if let Some(subtitle) = subtitle {
                p { class: "muted", "{subtitle}" }
            }
        }
    }
}

#[component]
pub fn BackLink(to: crate::Route, label: String) -> Element {
    rsx! {
        Link { class: "back", to, "← {label}" }
    }
}
