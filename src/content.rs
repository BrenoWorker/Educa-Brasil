use serde::Deserialize;
use std::sync::LazyLock;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Question {
    pub prompt: String,
    pub options: Vec<String>,
    pub answer: usize,
    pub explanation: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Topic {
    pub id: String,
    pub year: u8,
    pub subject: String,
    pub title: String,
    pub summary: String,
    pub lesson: Vec<String>,
    pub example: String,
    pub questions: Vec<Question>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Subject {
    pub id: &'static str,
    pub name: &'static str,
    pub icon: &'static str,
    pub color: &'static str,
}

pub const SUBJECTS: [Subject; 5] = [
    Subject {
        id: "portugues",
        name: "Português",
        icon: "📖",
        color: "#2563eb",
    },
    Subject {
        id: "matematica",
        name: "Matemática",
        icon: "➗",
        color: "#dc2626",
    },
    Subject {
        id: "ciencias",
        name: "Ciências",
        icon: "🔬",
        color: "#16a34a",
    },
    Subject {
        id: "historia",
        name: "História",
        icon: "🏛️",
        color: "#b45309",
    },
    Subject {
        id: "geografia",
        name: "Geografia",
        icon: "🌎",
        color: "#0891b2",
    },
];

pub const YEARS: std::ops::RangeInclusive<u8> = 1..=9;

static TOPICS: LazyLock<Vec<Topic>> = LazyLock::new(|| {
    serde_json::from_str(include_str!("../content/fundamental.json")).unwrap_or_default()
});

pub fn topics() -> &'static [Topic] {
    &TOPICS
}

pub fn topic(id: &str) -> Option<&'static Topic> {
    TOPICS.iter().find(|topic| topic.id == id)
}

pub fn topics_for(year: u8, subject: &str) -> Vec<&'static Topic> {
    TOPICS
        .iter()
        .filter(|topic| topic.year == year && topic.subject == subject)
        .collect()
}

pub fn topics_for_year(year: u8) -> Vec<&'static Topic> {
    TOPICS.iter().filter(|topic| topic.year == year).collect()
}

pub fn subject(id: &str) -> Option<Subject> {
    SUBJECTS.iter().copied().find(|subject| subject.id == id)
}

pub fn stage_label(year: u8) -> &'static str {
    if year <= 5 {
        "Anos iniciais"
    } else {
        "Anos finais"
    }
}
