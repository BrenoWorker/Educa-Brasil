use crate::components::{BackLink, PageHeader, ProgressBar};
use crate::content::{stage_label, topics_for, SUBJECTS, YEARS};
use crate::progress::Progress;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn YearPage(year: u8) -> Element {
    let progress = use_context::<Signal<Progress>>();
    let progress = progress.read();

    if !YEARS.contains(&year) {
        return rsx! {
            BackLink { to: Route::Home {}, label: "Início".to_string() }
            PageHeader { title: "Ano não encontrado".to_string() }
        };
    }

    rsx! {
        BackLink { to: Route::Home {}, label: "Início".to_string() }
        PageHeader { title: format!("{year}º ano"), subtitle: Some(stage_label(year).to_string()) }
        div { class: "list",
            for subject in SUBJECTS {
                {
                    let subject_topics = topics_for(year, subject.id);
                    let fraction = progress.completion(&subject_topics);
                    let stars: u32 = subject_topics.iter().map(|topic| progress.stars(topic) as u32).sum();
                    let max = subject_topics.len() * 3;
                    let count = subject_topics.len();
                    rsx! {
                        Link {
                            class: "card subject-card",
                            style: "--accent: {subject.color};",
                            to: Route::SubjectPage { year, subject: subject.id.to_string() },
                            span { class: "subject-icon", "{subject.icon}" }
                            div { class: "subject-body",
                                strong { "{subject.name}" }
                                span { class: "muted small", "{count} tema(s) · ⭐ {stars}/{max}" }
                                ProgressBar { fraction, color: subject.color.to_string() }
                            }
                            span { class: "chevron", "›" }
                        }
                    }
                }
            }
        }
    }
}
