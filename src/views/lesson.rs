use crate::components::{BackLink, PageHeader};
use crate::content::{subject, topic};
use crate::progress::Progress;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn LessonPage(id: String) -> Element {
    let mut progress = use_context::<Signal<Progress>>();
    let navigator = use_navigator();

    let Some(topic) = topic(&id) else {
        return rsx! {
            BackLink { to: Route::Home {}, label: "Início".to_string() }
            PageHeader { title: "Aula não encontrada".to_string() }
        };
    };
    let info = subject(&topic.subject);
    let color = info.map(|s| s.color).unwrap_or("var(--green)");
    let badge = info
        .map(|s| format!("{} {} · {}º ano", s.icon, s.name, topic.year))
        .unwrap_or_default();
    let quiz_id = topic.id.clone();

    rsx! {
        BackLink { to: Route::SubjectPage { year: topic.year, subject: topic.subject.clone() }, label: "Temas".to_string() }
        article { class: "lesson", style: "--accent: {color};",
            span { class: "badge", "{badge}" }
            h1 { "{topic.title}" }
            p { class: "lead", "{topic.summary}" }
            for paragraph in topic.lesson.iter() {
                p { "{paragraph}" }
            }
            div { class: "example",
                strong { "Exemplo" }
                p { "{topic.example}" }
            }
            button {
                class: "button wide",
                onclick: move |_| {
                    progress.write().mark_lesson(&quiz_id);
                    navigator.push(Route::QuizPage { id: quiz_id.clone() });
                },
                "Entendi! Ir para o quiz"
            }
        }
    }
}
