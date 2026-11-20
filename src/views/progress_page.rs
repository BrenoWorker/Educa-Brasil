use crate::components::{PageHeader, ProgressBar, Stars};
use crate::content::{topics, topics_for, topics_for_year, SUBJECTS, YEARS};
use crate::progress::Progress;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn ProgressPage() -> Element {
    let mut progress = use_context::<Signal<Progress>>();
    let mut confirm_reset = use_signal(|| false);
    let snapshot = progress.read().clone();
    let all = topics();
    let completed = all.iter().filter(|topic| snapshot.completed(topic)).count();

    rsx! {
        PageHeader { title: "Seu progresso".to_string(), subtitle: Some(format!("Nível {} · {} XP", snapshot.level(), snapshot.xp())) }
        div { class: "stats",
            div { class: "card stat",
                strong { "{completed}/{all.len()}" }
                span { class: "muted small", "temas concluídos" }
            }
            div { class: "card stat",
                strong { "⭐ {snapshot.total_stars(all)}" }
                span { class: "muted small", "estrelas" }
            }
            div { class: "card stat",
                strong { "{snapshot.level()}" }
                span { class: "muted small", "nível" }
            }
        }

        for year in YEARS {
            {
                let year_topics = topics_for_year(year);
                let fraction = snapshot.completion(&year_topics);
                rsx! {
                    div { class: "card progress-year",
                        div { class: "topic-head",
                            Link { to: Route::YearPage { year }, h3 { "{year}º ano" } }
                            span { class: "muted small", "{(fraction * 100.0).round()}%" }
                        }
                        ProgressBar { fraction }
                        div { class: "progress-subjects",
                            for subject in SUBJECTS {
                                {
                                    let stars: u8 = topics_for(year, subject.id).iter().map(|topic| snapshot.stars(topic)).max().unwrap_or(0);
                                    rsx! {
                                        div { class: "progress-subject",
                                            span { "{subject.icon} {subject.name}" }
                                            Stars { count: stars }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        div { class: "danger-zone",
            if confirm_reset() {
                p { "Tem certeza? Todo o progresso será apagado." }
                div { class: "actions",
                    button {
                        class: "button danger",
                        onclick: move |_| {
                            progress.write().reset();
                            confirm_reset.set(false);
                        },
                        "Apagar progresso"
                    }
                    button { class: "button secondary", onclick: move |_| confirm_reset.set(false), "Cancelar" }
                }
            } else {
                button { class: "button secondary", onclick: move |_| confirm_reset.set(true), "Recomeçar do zero" }
            }
        }
    }
}
