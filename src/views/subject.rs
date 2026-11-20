use crate::components::{BackLink, PageHeader, Stars};
use crate::content::{subject as find_subject, topics_for};
use crate::progress::Progress;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn SubjectPage(year: u8, subject: String) -> Element {
    let progress = use_context::<Signal<Progress>>();
    let progress = progress.read();

    let Some(info) = find_subject(&subject) else {
        return rsx! {
            BackLink { to: Route::YearPage { year }, label: format!("{year}º ano") }
            PageHeader { title: "Matéria não encontrada".to_string() }
        };
    };

    let items = topics_for(year, &subject);

    rsx! {
        BackLink { to: Route::YearPage { year }, label: format!("{year}º ano") }
        PageHeader { title: format!("{} {}", info.icon, info.name), subtitle: Some(format!("{year}º ano")) }
        if items.is_empty() {
            p { class: "muted", "Ainda não há temas para esta matéria." }
        }
        div { class: "list",
            for topic in items {
                {
                    let state = progress.get(&topic.id);
                    let stars = progress.stars(topic);
                    rsx! {
                        div { class: "card topic-card", style: "--accent: {info.color};",
                            div { class: "topic-head",
                                h3 { "{topic.title}" }
                                Stars { count: stars }
                            }
                            p { class: "muted", "{topic.summary}" }
                            div { class: "tags",
                                if state.lesson_read {
                                    span { class: "tag done", "Aula lida" }
                                }
                                if state.attempts > 0 {
                                    span { class: "tag", "Melhor: {state.best_score}/{topic.questions.len()}" }
                                }
                            }
                            div { class: "actions",
                                Link { class: "button secondary", to: Route::LessonPage { id: topic.id.clone() }, "Ler aula" }
                                Link { class: "button", to: Route::QuizPage { id: topic.id.clone() }, "Fazer quiz" }
                            }
                        }
                    }
                }
            }
        }
    }
}
