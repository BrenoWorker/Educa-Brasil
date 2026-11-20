mod components;
mod content;
mod progress;
mod storage;
mod views;

use dioxus::prelude::*;
use progress::Progress;
use views::{Home, LessonPage, NotFound, ProgressPage, QuizPage, Shell, SubjectPage, YearPage};

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Shell)]
        #[route("/")]
        Home {},
        #[route("/ano/:year")]
        YearPage { year: u8 },
        #[route("/ano/:year/:subject")]
        SubjectPage { year: u8, subject: String },
        #[route("/aula/:id")]
        LessonPage { id: String },
        #[route("/quiz/:id")]
        QuizPage { id: String },
        #[route("/progresso")]
        ProgressPage {},
        #[route("/:..segments")]
        NotFound { segments: Vec<String> },
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(Progress::load()));

    rsx! {
        document::Title { "Educa Brasil" }
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1, viewport-fit=cover" }
        document::Meta { name: "theme-color", content: "#009c3b" }
        document::Stylesheet { href: MAIN_CSS }
        Router::<Route> {}
    }
}
