use crate::components::{BackLink, PageHeader, ProgressBar, Stars};
use crate::content::{subject, topic, Question};
use crate::progress::Progress;
use crate::Route;
use dioxus::prelude::*;

fn seed_for(text: &str, round: u32) -> u64 {
    text.bytes()
        .fold(0xcbf2_9ce4_8422_2325u64 ^ round as u64, |hash, byte| {
            (hash ^ byte as u64).wrapping_mul(0x0100_0000_01b3)
        })
}

fn shuffled_order(len: usize, seed: u64) -> Vec<usize> {
    let mut order: Vec<usize> = (0..len).collect();
    let mut state = seed | 1;
    for index in (1..len).rev() {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        order.swap(index, (state % (index as u64 + 1)) as usize);
    }
    order
}

fn option_class(question: &Question, original: usize, selected: Option<usize>) -> &'static str {
    match selected {
        None => "option",
        Some(_) if original == question.answer => "option correct",
        Some(choice) if choice == original => "option wrong",
        Some(_) => "option faded",
    }
}

#[component]
pub fn QuizPage(id: String) -> Element {
    let mut progress = use_context::<Signal<Progress>>();
    let mut current = use_signal(|| 0usize);
    let mut selected = use_signal(|| None::<usize>);
    let mut score = use_signal(|| 0u8);
    let mut finished = use_signal(|| false);
    let mut round = use_signal(|| progress.peek().get(&id).attempts);

    let Some(topic) = topic(&id) else {
        return rsx! {
            BackLink { to: Route::Home {}, label: "Início".to_string() }
            PageHeader { title: "Quiz não encontrado".to_string() }
        };
    };

    let color = subject(&topic.subject)
        .map(|s| s.color)
        .unwrap_or("var(--green)");
    let total = topic.questions.len();
    let back = rsx! {
        BackLink { to: Route::SubjectPage { year: topic.year, subject: topic.subject.clone() }, label: "Temas".to_string() }
    };

    if total == 0 {
        return rsx! {
            {back}
            PageHeader { title: topic.title.clone(), subtitle: Some("Este tema ainda não tem perguntas.".to_string()) }
        };
    }

    if finished() {
        let final_score = score();
        let stars = ((final_score as usize * 3) / total).min(3) as u8;
        let message = match stars {
            3 => "Perfeito! Você acertou tudo!",
            2 => "Muito bem! Falta pouco para a nota máxima.",
            1 => "Bom começo! Leia a aula de novo e tente outra vez.",
            _ => "Não desanime! Revise a aula e tente de novo.",
        };
        let lesson_id = topic.id.clone();
        return rsx! {
            {back}
            div { class: "card result", style: "--accent: {color};",
                h1 { "{final_score}/{total}" }
                Stars { count: stars }
                p { class: "lead", "{message}" }
                p { class: "muted", "+{final_score as u32 * 10} XP possíveis neste tema" }
                div { class: "actions",
                    button {
                        class: "button",
                        onclick: move |_| {
                            round += 1;
                            current.set(0);
                            selected.set(None);
                            score.set(0);
                            finished.set(false);
                        },
                        "Tentar de novo"
                    }
                    Link { class: "button secondary", to: Route::LessonPage { id: lesson_id }, "Rever aula" }
                }
            }
        };
    }

    let index = current().min(total - 1);
    let question = &topic.questions[index];
    let order = shuffled_order(question.options.len(), seed_for(&question.prompt, round()));
    let choice = selected();
    let is_last = index + 1 == total;
    let topic_id = topic.id.clone();
    let answer = question.answer;

    rsx! {
        {back}
        div { class: "quiz", style: "--accent: {color};",
            div { class: "quiz-meta",
                span { class: "muted", "{topic.title}" }
                span { class: "muted", "Pergunta {index + 1} de {total}" }
            }
            ProgressBar { fraction: index as f32 / total as f32, color: color.to_string() }
            h2 { class: "prompt", "{question.prompt}" }
            div { class: "options",
                for original in order {
                    button {
                        key: "{original}",
                        class: option_class(question, original, choice),
                        disabled: choice.is_some(),
                        onclick: move |_| {
                            if selected().is_none() {
                                selected.set(Some(original));
                                if original == answer {
                                    score += 1;
                                }
                            }
                        },
                        "{question.options[original]}"
                    }
                }
            }
            if let Some(choice) = choice {
                div { class: if choice == answer { "feedback good" } else { "feedback bad" },
                    strong { if choice == answer { "Acertou! 🎉" } else { "Quase lá!" } }
                    p { "{question.explanation}" }
                }
                button {
                    class: "button wide",
                    onclick: move |_| {
                        if is_last {
                            progress.write().record_quiz(&topic_id, score());
                            finished.set(true);
                        } else {
                            current += 1;
                            selected.set(None);
                        }
                    },
                    if is_last { "Ver resultado" } else { "Próxima pergunta" }
                }
            }
        }
    }
}
