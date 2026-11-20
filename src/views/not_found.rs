use crate::components::PageHeader;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    let path = segments.join("/");
    rsx! {
        PageHeader { title: "Página não encontrada".to_string(), subtitle: Some(format!("/{path}")) }
        Link { class: "button", to: Route::Home {}, "Voltar ao início" }
    }
}
