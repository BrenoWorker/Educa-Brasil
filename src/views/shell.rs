use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Shell() -> Element {
    rsx! {
        div { class: "app",
            header { class: "topbar",
                Link { class: "brand", to: Route::Home {},
                    span { class: "brand-mark", "EB" }
                    span { "Educa Brasil" }
                }
                nav { class: "nav",
                    Link { class: "nav-link", active_class: "active", to: Route::Home {}, "Início" }
                    Link { class: "nav-link", active_class: "active", to: Route::ProgressPage {}, "Progresso" }
                }
            }
            main { class: "content", Outlet::<Route> {} }
        }
    }
}
