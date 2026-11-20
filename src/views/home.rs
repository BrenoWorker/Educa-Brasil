use crate::components::ProgressBar;
use crate::content::{stage_label, topics, topics_for_year, YEARS};
use crate::progress::Progress;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let progress = use_context::<Signal<Progress>>();
    let progress = progress.read();
    let level = progress.level();
    let xp = progress.xp();
    let stars = progress.total_stars(topics());
    let max_stars = topics().len() * 3;

    rsx! {
        section { class: "hero",
            div {
                p { class: "hero-kicker", "Ensino fundamental · 1º ao 9º ano" }
                h1 { "Aprender é uma aventura!" }
                p { "Escolha seu ano, leia a aula e mostre o que aprendeu no quiz." }
            }
            div { class: "level-card",
                div { class: "level-row",
                    span { class: "level-badge", "Nível {level}" }
                    span { class: "muted", "{xp} XP · ⭐ {stars}/{max_stars}" }
                }
                ProgressBar { fraction: progress.level_fraction(), color: "var(--yellow)".to_string() }
            }
        }

        h2 { class: "section-title", "Escolha o seu ano" }
        div { class: "grid years",
            for year in YEARS {
                {
                    let year_topics = topics_for_year(year);
                    let fraction = progress.completion(&year_topics);
                    let percent = (fraction * 100.0).round();
                    rsx! {
                        Link { class: "card year-card", to: Route::YearPage { year },
                            span { class: "year-number", "{year}º" }
                            span { class: "year-label", "ano" }
                            span { class: "muted small", "{stage_label(year)}" }
                            ProgressBar { fraction }
                            span { class: "muted small", "{percent}% concluído" }
                        }
                    }
                }
            }
        }
    }
}
